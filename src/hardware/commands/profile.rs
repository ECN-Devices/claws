use std::{
  sync::{Arc, Mutex},
  thread::sleep,
  time::Duration,
};

use log::error;
use serialport::SerialPort;

use crate::{
  data::{
    Config,
    profiles::{KEYPAD_BUTTONS, Profile},
  },
  hardware::serial::{DeviceIO, Keypad},
};

use super::{KeypadCommands, Value, stick, switch};

#[derive(Debug, Clone)]
pub enum Command {
  /**
  Запрос: 0x73, 0x1, 0x10, 0x65

  Ответ: 0x73, 0x2, 0x10, (номер профиля), 0x65
  */
  RequestActiveNum,

  /**
  Запрос: 0x73, 0x1, 0x11, 0x65

  Ответ: 0x73, 0x16, 0x11, (15 байт названия) , 0x65

  Название должно иметь длину 15 байт. Если название короче 15 символов, не использованные байты заполнятются нулями
  */
  RequestName,

  /**
  Запрос: 0x73, 0x16, 0x12, (15 байт названия) , 0x65

  Ответ: нет ответа

  Название должно иметь длину 15 байт. Если название короче 15 символов, не использованные байты заполняются нулями
  */
  SetName([u8; 15]),

  /**
  Запрос: 0x73, 0x2, 0x13, (номер профиля ОЗУ) , 0x65

  Ответ: нет ответа

  Кейпад имеет 4 профиля
  */
  WriteActiveToRam(u8),

  /**
  Запрос: 0x73, 0x2, 0x14, (номер профиля ПЗУ) , 0x65

  Ответ: нет ответа
  */
  WriteActiveToFlash(u8),

  /**
  Запрос: 0x73, 0x2, 0x15, (номер профиля ram) , 0x65

  Ответ: нет ответа

  Кейпад имеет 4 профиля
  */
  LoadRamToActive(u8),

  /**
  Запрос: 0x73, 0x1, 0x16, 0x65

  Загружает все профили из flash в ram. В следствии этого все профили в ram, а также активный профиль, будут перезаписаны.
  */
  LoadFlashToRam,
}

impl Value for Command {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestActiveNum => vec![10],
      Self::RequestName => vec![11],
      Self::SetName(profile_name) => {
        let mut result = vec![12];
        for e in profile_name {
          result.push(*e)
        }
        result
      }
      Self::WriteActiveToRam(num) => vec![13, *num],
      Self::WriteActiveToFlash(num) => vec![14, *num],
      Self::LoadRamToActive(num) => vec![15, *num],
      Self::LoadFlashToRam => vec![16],
    }
  }
}

/// Трейт для работы с профилями кейпада через последовательный порт
pub trait SerialProfile {
  /**
  Читает текущий профиль с устройства через последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт
  # Возвращает
  Прочитанный профиль
  */
  fn receive_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) -> Profile;

  /**
  Сохраняет текущий профиль с устройства в хранилище
  # Аргументы
  * `port` - Ссылка на последовательный порт
  */
  fn save_profile_file(port: &mut Arc<Mutex<Box<dyn SerialPort>>>);

  /**
  Записывает профиль на устройство через последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт (`Arc<Mutex>` для потокобезопасности)
  * `profile` - Профиль для записи
  */
  fn send_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>, profile: Profile);
}

impl SerialProfile for Command {
  fn receive_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) -> Profile {
    let mut keypad_profile = Profile::default();

    // Читаем конфигурацию кнопок (1..16)
    (1..=KEYPAD_BUTTONS).for_each(|button_num| {
      Keypad::send(
        port,
        &KeypadCommands::Swtich(switch::Command::RequestCodeASCII(button_num)),
      )
      .unwrap();

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
      Keypad::send(
        port,
        &KeypadCommands::Stick(stick::Command::RequestPositionASCII),
      )
      .unwrap();

      // Задержка между запросами
      // Иначе не читает??????
      sleep(Duration::from_millis(10));

      match Keypad::receive(port) {
        Ok(buf) => keypad_profile.stick[stick_num - 1] = buf[stick_num],
        Err(e) => error!("Ошибка чтения: stick {stick_num}; с ошибкой: {e}"),
      }
    });

    // Читаем имя профиля
    Keypad::send(port, &KeypadCommands::Profile(Command::RequestName)).unwrap();

    // Задержка между запросами
    // Иначе не читает??????
    sleep(Duration::from_millis(10));

    match Keypad::receive(port) {
      Ok(buf) => keypad_profile.name = String::from_utf8_lossy(&buf).trim_start().into(),
      Err(e) => error!("Ошибка чтения: name; с ошибкой: {e}"),
    }

    keypad_profile
  }

  fn save_profile_file(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) {
    let keypad_profile = Self::receive_profile(port);
    keypad_profile.save()
  }

  fn send_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>, profile: Profile) {
    // Записываем конфигурацию кнопок
    (1..=KEYPAD_BUTTONS).for_each(|i| {
      Keypad::send(
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
      Keypad::send(
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

    Keypad::send(port, &KeypadCommands::Profile(Command::SetName(name))).unwrap()
  }
}
