use super::Config;
use crate::assets::APPLICATION_NAME;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
