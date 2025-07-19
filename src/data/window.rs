use crate::{
  assets::APPLICATION_NAME,
  ui::{WINDOW_HEIGH, WINDOW_WIDTH},
};
use serde::{Deserialize, Serialize};

use super::Config;

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
  pub fn load() -> Self {
    confy::load(APPLICATION_NAME, "window").expect("Не удалось загрузить конфигурацию окна")
  }
}
impl Config for Window {
  fn save(&self) {
    confy::store(APPLICATION_NAME, "window", self).expect("Не удалось записать конфигурацию окна")
  }
}
