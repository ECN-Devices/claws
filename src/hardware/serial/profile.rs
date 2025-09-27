use super::Keypad;
use crate::{
  data::{
    Config,
    profiles::{KEYPAD_BUTTONS, Profile},
  },
  hardware::{
    buffers::{Buffers, BuffersIO},
    commands::{Value, profile, stick, switch},
  },
};
use anyhow::Result;
use log::{debug, info};

impl Keypad {
  /**
  Читает текущий профиль с устройства через последовательный порт
  # Аргументы
  * `buffers` - Буферы для обмена данными
  # Возвращает
  Прочитанный профиль или ошибку
  */
  pub fn profile_receive(buffers: &mut Buffers) -> Result<Profile> {
    let mut keypad_profile = Profile::default();

    // Читаем имя профиля
    let profile_name = String::from_utf8_lossy(&profile::request_name(buffers)?).into_owned();
    keypad_profile.name = profile_name;

    // Читаем конфигурацию кнопок (1..16)
    let buttons_s = switch::request_code_ascii(buffers)?;
    keypad_profile.buttons = buttons_s;

    // Читаем конфигурацию стика (4 направления)
    let stick_s = stick::request_position_ascii(buffers).unwrap_or([b'?'; 4]);
    keypad_profile.stick.word = stick_s;

    let stick_d = stick::calibration_request(buffers)?;
    keypad_profile.stick.deadzone = *stick_d.last().unwrap();

    Ok(keypad_profile)
  }

  /**
  Записывает профиль на устройство через последовательный порт
  # Аргументы
  * `buffers` - Буферы для обмена данными
  * `profile` - Профиль для записи
  */
  pub fn profile_send(buffers: &mut Buffers, profile: Profile) -> Result<()> {
    let mut profile_name = [0u8; 15];
    profile
      .name
      .chars()
      .take(profile_name.len())
      .enumerate()
      .for_each(|(i, c)| profile_name[i] = c as u8);

    let switch_s = profile.buttons;
    let stick_s = profile.stick.word;
    let stick_d = profile.stick.deadzone;

    // Записываем имя профиля
    buffers
      .send()
      .push(profile::Command::SetName(profile_name).get());
    info!("profile_send: записываю имя профиля: {profile_name:?}");

    // Записываем конфигурацию кнопок
    (1..=KEYPAD_BUTTONS).for_each(|i| {
      buffers
        .send()
        .push(switch::Command::SetCodeASCII(i, switch_s[i as usize - 1]).get());
    });
    info!("profile_send: записываю конфигурацию кнопок");
    debug!("profile_send: конфигурация кнопок {:?}", profile.buttons);

    // Записываем конфигурацию стика
    (1..=4).for_each(|i| {
      buffers
        .send()
        .push(stick::Command::SetPositionASCII(i, stick_s[i as usize - 1]).get());
    });
    info!("profile_send: записываю конфигурацию стика");
    debug!("profile_send: конфигурация стика {:?}", profile.stick);

    buffers
      .send()
      .push(stick::Command::SetParameters(4, stick_d).get());
    info!("profile_send: записываю мертвую зону");

    Ok(())
  }

  /**
  Сохраняет профиль в хранилище
  */
  pub fn save_profile_file(profile: Profile) {
    profile.save()
  }
}
