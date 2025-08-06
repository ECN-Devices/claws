use super::{DeviceIO, Keypad, KeypadError};
use crate::{
  data::{
    Config,
    profiles::{KEYPAD_BUTTONS, Profile},
  },
  hardware::buffers::Buffers,
};
use log::error;
use anyhow::Result;
use serialport::SerialPort;
use std::{
  sync::{Arc, Mutex},
  thread::sleep,
  time::Duration,
};

/// Трейт для работы с профилями кейпада через последовательный порт
pub trait SerialProfile {
  /**
  Читает текущий профиль с устройства через последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт
  * `buffers` - Буферы для обмена данными
  # Возвращает
  Прочитанный профиль или ошибку
  */
  fn receive_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>, buffers: &mut Buffers) -> Profile;

  /**
  Сохраняет текущий профиль с устройства в хранилище
  # Аргументы
  * `port` - Ссылка на последовательный порт
  */
  fn save_profile_file(port: &mut Arc<Mutex<Box<dyn SerialPort>>>, buffers: &mut Buffers);

  /**
  Записывает профиль на устройство через последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт (`Arc<Mutex>` для потокобезопасности)
  * `profile` - Профиль для записи
  */
  fn send_profile(
    port: &mut Arc<Mutex<Box<dyn SerialPort>>>,
    profile: Profile,
    buffers: &mut Buffers,
  ) -> Result<()>;
}

impl SerialProfile for Keypad {
  fn receive_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>, buffers: &mut Buffers) -> Profile {
    let mut keypad_profile = Profile::default();

    // Читаем конфигурацию кнопок (1..16)
    (1..=KEYPAD_BUTTONS).for_each(|button_num| {
      let _ = Keypad::send(port, buffers);

      // Задержка между запросами
      // Иначе не читает??????
      sleep(Duration::from_millis(10));

      match Keypad::receive(port) {
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
      // Keypad::send(port, buffers);

      // Задержка между запросами
      // Иначе не читает??????
      sleep(Duration::from_millis(10));

      match Keypad::receive(port) {
        Ok(buf) => keypad_profile.stick[stick_num - 1] = buf[stick_num],
        Err(e) => error!("Ошибка чтения: stick {stick_num}; с ошибкой: {e}"),
      }
    });

    // Читаем имя профиля
    let _ = Keypad::send(port, buffers);

    // Задержка между запросами
    // Иначе не читает??????
    sleep(Duration::from_millis(10));

    match Keypad::receive(port) {
      Ok(buf) => keypad_profile.name = String::from_utf8_lossy(&buf).trim_start().into(),
      Err(e) => error!("Ошибка чтения: name; с ошибкой: {e}"),
    }

    keypad_profile
  }

  fn save_profile_file(port: &mut Arc<Mutex<Box<dyn SerialPort>>>, buffers: &mut Buffers) {
    let keypad_profile = Self::receive_profile(port, buffers);
    keypad_profile.save()
  }

  fn send_profile(
    port: &mut Arc<Mutex<Box<dyn SerialPort>>>,
    profile: Profile,
    buffers: &mut Buffers,
  ) -> Result<()> {
    // Записываем конфигурацию кнопок
    // (1..=KEYPAD_BUTTONS).for_each(|i| {
    //   Keypad::send(
    //     port,
    //     &KeypadCommands::Swtich(switch::Command::SetCodeASCII(
    //       i,
    //       profile.buttons[i as usize - 1],
    //     )),
    //   )
    //   .unwrap()
    // });
    for _ in 1..=KEYPAD_BUTTONS {
      Keypad::send(port, buffers)?
    }

    // Записываем конфигурацию стика
    for _ in 1..=4 {
      Keypad::send(port, buffers)?
    }

    // Записываем имя профиля
    let mut name = [0u8; 15];
    profile
      .name
      .chars()
      .take(name.len())
      .enumerate()
      .for_each(|(i, c)| name[i] = c as u8);

    Keypad::send(port, buffers)
  }
}
