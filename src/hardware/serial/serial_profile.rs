use super::{Keypad, SerialOperations};
use crate::{
  data::{
    Config,
    profiles::{KEYPAD_BUTTONS, Profile},
  },
  hardware::commands::{KeypadCommands, profile, stick, switch},
};
use log::error;
use serialport::SerialPort;
use std::{
  sync::{Arc, Mutex},
  thread::sleep,
  time::Duration,
};

// / Трейт для работы с профилями кейпада через последовательный порт
pub trait SerialProfile {
  /**
  Записывает профиль на устройство через последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт (`Arc<Mutex>` для потокобезопасности)
  * `profile` - Профиль для записи
  */
  fn write_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>, profile: Profile);

  /**
  Читает текущий профиль с устройства через последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт
  # Возвращает
  Прочитанный профиль
  */
  fn read_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) -> Profile;

  /**
  Сохраняет текущий профиль с устройства в хранилище
  # Аргументы
  * `port` - Ссылка на последовательный порт
  */
  fn save_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>);
}
impl SerialProfile for Keypad {
  fn write_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>, profile: Profile) {
    // Записываем конфигурацию кнопок
    (1..=KEYPAD_BUTTONS).for_each(|i| {
      Keypad::write_port(
        port,
        &KeypadCommands::Swtich(switch::Command::SetCodeASCII(
          i,
          profile.buttons[i as usize - 1],
        )),
      )
      .unwrap()
    });

    // Записываем конфигурацию стика
    (1..=4).for_each(|i| {
      Self::write_port(
        port,
        &KeypadCommands::Stick(stick::Command::SetPositionASCII(
          i,
          profile.stick[i as usize - 1],
        )),
      )
      .unwrap()
    });

    // Записываем имя профиля
    let mut name = [0u8; 15];
    profile
      .name
      .chars()
      .take(name.len())
      .enumerate()
      .for_each(|(i, c)| name[i] = c as u8);

    Self::write_port(
      port,
      &KeypadCommands::Profile(profile::Command::SetName(name)),
    )
    .unwrap()
  }

  fn read_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) -> Profile {
    let mut keypad_profile = Profile::default();

    // Читаем конфигурацию кнопок (1..16)
    (1..=KEYPAD_BUTTONS).for_each(|button_num| {
      Self::write_port(
        port,
        &KeypadCommands::Swtich(switch::Command::RequestCodeASCII(button_num)),
      )
      .unwrap();

      // Задержка между запросами
      // Иначе не читает??????
      sleep(Duration::from_millis(10));

      match Self::read_port(port) {
        Ok(mut buf) => {
          buf.drain(0..2);
          let button_data: [u8; 6] = buf.try_into().unwrap();
          keypad_profile.buttons[button_num as usize - 1] = button_data
        }
        Err(e) => error!("Ошибка чтения: stick {button_num}; с ошибкой: {e}"),
      }
    });

    // Читаем конфигурацию стика (4 направления)
    (1usize..=4).for_each(|stick_num| {
      Self::write_port(
        port,
        &KeypadCommands::Stick(stick::Command::RequestPositionASCII),
      )
      .unwrap();

      // Задержка между запросами
      // Иначе не читает??????
      sleep(Duration::from_millis(10));

      match Self::read_port(port) {
        Ok(buf) => keypad_profile.stick[stick_num - 1] = buf[stick_num],
        Err(e) => error!("Ошибка чтения: stick {stick_num}; с ошибкой: {e}"),
      }
    });

    // Читаем имя профиля
    Self::write_port(
      port,
      &KeypadCommands::Profile(profile::Command::RequestName),
    )
    .unwrap();

    // Задержка между запросами
    // Иначе не читает??????
    sleep(Duration::from_millis(10));

    match Self::read_port(port) {
      Ok(buf) => keypad_profile.name = String::from_utf8_lossy(&buf).trim_start().into(),
      Err(e) => error!("Ошибка чтения: name; с ошибкой: {e}"),
    }

    keypad_profile
  }

  fn save_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) {
    let keypad_profile = Self::read_profile(port);
    keypad_profile.save()
  }
}
