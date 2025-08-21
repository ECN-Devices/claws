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
        deadzone: 50,
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
      16 | 212 => "Del".to_string(),
      27 | 177 => "Esc".to_string(),
      128 => "LCtrl".to_string(),
      129 => "LShift".to_string(),
      130 => "LAlt".to_string(),
      131 => "LWin".to_string(),
      132 => "RCtrl".to_string(),
      133 => "RShift".to_string(),
      134 => "RAlt".to_string(),
      135 => "RWin".to_string(),
      176 => "Enter".to_string(),
      178 => "Backspace".to_string(),
      179 => "Tab".to_string(),
      193 => "CapsLock".to_string(),
      206 => "PrScr".to_string(),
      207 => "ScrollLock".to_string(),
      208 => "Pause".to_string(),
      209 => "Insert".to_string(),
      210 => "Home".to_string(),
      213 => "End".to_string(),
      211 => "PgUp".to_string(),
      214 => "PgDn".to_string(),
      219 => "NumLock".to_string(),
      194 => "F1".to_string(),
      195 => "F2".to_string(),
      196 => "F3".to_string(),
      197 => "F4".to_string(),
      198 => "F5".to_string(),
      199 => "F6".to_string(),
      200 => "F7".to_string(),
      201 => "F8".to_string(),
      202 => "F9".to_string(),
      203 => "F10".to_string(),
      204 => "F11".to_string(),
      205 => "F12".to_string(),
      240 => "F13".to_string(),
      241 => "F14".to_string(),
      242 => "F15".to_string(),
      243 => "F16".to_string(),
      244 => "F17".to_string(),
      245 => "F18".to_string(),
      246 => "F19".to_string(),
      247 => "F20".to_string(),
      248 => "F21".to_string(),
      249 => "F22".to_string(),
      250 => "F23".to_string(),
      251 => "F24".to_string(),
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
