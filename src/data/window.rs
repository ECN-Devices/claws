use super::Config;
use crate::{
  assets::APPLICATION_NAME,
  ui::{WINDOW_HEIGH, WINDOW_WIDTH},
};
use serde::{Deserialize, Serialize};

/**
Конфигурация окна приложения
Содержит параметры положения и размеров главного окна приложения
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

impl Default for Window {
  fn default() -> Self {
    Self {
      x: 600.,
      y: 660.,
      width: WINDOW_WIDTH,
      height: WINDOW_HEIGH,
    }
  }
}

impl Window {
  /// Загружает конфигурацию окна из хранилища
  pub fn load() -> Self {
    confy::load(APPLICATION_NAME, "window").expect("Не удалось загрузить конфигурацию окна")
  }
}
impl Config for Window {
  /// Сохраняет текущую конфигурацию окна в хранилище
  fn save(&self) {
    confy::store(APPLICATION_NAME, "window", self).expect("Не удалось записать конфигурацию окна")
  }
}
