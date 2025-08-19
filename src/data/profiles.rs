use super::Config;
use crate::{assets::APPLICATION_NAME, hardware::serial::stick::Stick};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const KEYPAD_BUTTONS: u8 = 16;
const SEPARATOR: &str = " ";

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
  pub stick: Stick,
}

impl Default for Profile {
  fn default() -> Self {
    Self {
      name: "Default".to_string(),
      buttons: [[0; 6]; 16],
      stick: Stick {
        word: [0u8; 4],
        deadzone: 20,
      },
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
  pub fn load_file(path: &Path) -> Self {
    confy::load_path(path).expect("Не удалось загрузить конфигурацию профиля из файла")
  }

  /// Преобразует код клавиши в читаемый символ
  fn code_to_char(code: u8) -> String {
    match code {
      16 => "Del".to_string(),
      27 => "Esc".to_string(),
      128 => "Ctrl".to_string(),
      179 => "Tab".to_string(),
      // stick
      218 => "↑".to_string(),
      215 => "→".to_string(),
      217 => "↓".to_string(),
      216 => "←".to_string(),
      _ => char::from_u32(code as u32).unwrap_or('?').to_string(),
    }
  }

  pub fn get_button_label(&self, button_id: usize) -> String {
    self.buttons[button_id]
      .into_iter()
      .filter(|code| *code != 0)
      .map(Self::code_to_char)
      .collect::<Vec<_>>()
      .join(SEPARATOR)
  }

  pub fn get_stick_label(&self, stick_id: usize) -> String {
    let code = self.stick.word[stick_id];

    [code]
      .into_iter() // Создаем итератор из одного элемента
      .filter(|&c| c != 0)
      .map(Self::code_to_char)
      .collect::<Vec<_>>()
      .join(SEPARATOR)
  }

  fn char_to_code(symbol: &str) -> Option<u8> {
    match symbol {
      "Del" => Some(16),
      "Esc" => Some(27),
      "Ctrl" => Some(128),
      "Tab" => Some(179),
      "↑" => Some(218),
      "→" => Some(215),
      "↓" => Some(217),
      "←" => Some(216),
      _ => {
        if symbol.len() == 1 {
          let c = symbol.chars().next().unwrap();
          if c.is_ascii() { Some(c as u8) } else { None }
        } else {
          None
        }
      }
    }
  }

  pub fn parse_button_label(label: &str) -> Vec<u8> {
    let mut code: Vec<u8> = label.split(" + ").filter_map(Self::char_to_code).collect();
    code.resize(6, 0);
    code
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
