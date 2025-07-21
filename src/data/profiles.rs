use super::Config;
use crate::{
  assets::APPLICATION_NAME,
  hardware::{
    commands::{KeypadCommands, profile, stick, switch},
    serial::{Keypad, SerialOperations},
  },
};
use log::error;
use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use std::{
  path::PathBuf,
  sync::{Arc, Mutex},
  thread::sleep,
  time::Duration,
};

pub const KEYPAD_BUTTONS: u8 = 16;

/**
Профиль конфигурации контроллера

Содержит настройки кнопок и стиков для конкретного профиля.
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
  pub name: String,

  /**
  Конфигурация кнопок
  Двумерный массив где:
  - Первый уровень (16 элементов) соответствует кнопкам
  - Второй уровень (6 элементов) содержит коды клавиш/действий
  */
  pub buttons: [[u8; 6]; 16],

  /**
  Конфигурация стика
  Массив из 4 элементов, содержащий коды для:
  [Вверх, Вправо, Вниз, Влево]
  */
  pub stick: [u8; 4],
}

impl Default for Profile {
  fn default() -> Self {
    Self {
      name: "Default".to_string(),
      buttons: [[0; 6]; 16],
      stick: [0; 4],
    }
  }
}

impl Profile {
  /**
  Загружает профиль из хранилища по имени
  # Аргументы
  * `name` - Имя профиля для загрузки
  */
  pub fn load(name: &str) -> Self {
    confy::load(APPLICATION_NAME, format!("profile_{name}").as_str())
      .expect("Не удалось загрузить конфигурацию профиля")
  }

  /**
  Загружает профиль из указанного файла
  # Аргументы
  * `path` - Путь к файлу профиля
  */
  pub fn load_file(path: PathBuf) -> Self {
    confy::load_path(&path).expect("Не удалось загрузить конфигурацию профиля из файла")
  }

  pub fn get_button_label(&self, button_id: usize) -> String {
    let button_codes = &self.buttons[button_id];
    button_codes
      .iter()
      .map(|&code| char::from_u32(code as u32).unwrap())
      .collect()
  }
}
impl Config for Profile {
  /**
  Сохраняет текущий профиль в хранилище
  Имя файла будет сформировано как "profile_{имя_профиля}"
  */
  fn save(&self) {
    confy::store(
      APPLICATION_NAME,
      format!("profile_{}", self.name).as_str(),
      self,
    )
    .expect("Не удалось записать конфигурацию профиля")
  }
}

/// Трейт для работы с профилями кейпада через последовательный порт
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
  fn save_profile_file(port: &mut Arc<Mutex<Box<dyn SerialPort>>>);
}
impl SerialProfile for Profile {
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
      Keypad::write_port(
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

    Keypad::write_port(
      port,
      &KeypadCommands::Profile(profile::Command::SetName(name)),
    )
    .unwrap()
  }

  fn read_profile(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) -> Profile {
    let mut keypad_profile = Profile::default();

    // Читаем конфигурацию кнопок (1..16)
    (1..=KEYPAD_BUTTONS).for_each(|button_num| {
      Keypad::write_port(
        port,
        &KeypadCommands::Swtich(switch::Command::RequestCodeASCII(button_num)),
      )
      .unwrap();

      // Задержка между запросами
      // Иначе не читает??????
      sleep(Duration::from_millis(10));

      match Keypad::read_port(port) {
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
      Keypad::write_port(
        port,
        &KeypadCommands::Stick(stick::Command::RequestPositionASCII),
      )
      .unwrap();

      // Задержка между запросами
      // Иначе не читает??????
      sleep(Duration::from_millis(10));

      match Keypad::read_port(port) {
        Ok(buf) => keypad_profile.stick[stick_num - 1] = buf[stick_num],
        Err(e) => error!("Ошибка чтения: stick {stick_num}; с ошибкой: {e}"),
      }
    });

    // Читаем имя профиля
    Keypad::write_port(
      port,
      &KeypadCommands::Profile(profile::Command::RequestName),
    )
    .unwrap();

    // Задержка между запросами
    // Иначе не читает??????
    sleep(Duration::from_millis(10));

    match Keypad::read_port(port) {
      Ok(buf) => keypad_profile.name = String::from_utf8_lossy(&buf).trim_start().into(),
      Err(e) => error!("Ошибка чтения: name; с ошибкой: {e}"),
    }

    keypad_profile
  }

  fn save_profile_file(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) {
    let keypad_profile = Self::read_profile(port);
    keypad_profile.save()
  }
}
