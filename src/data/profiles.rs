use super::Config;
use crate::assets::APPLICATION_NAME;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
  pub name: String,
  pub buttons: [[u8; 6]; 16],
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
  pub fn load(name: &str) -> Self {
    confy::load(APPLICATION_NAME, format!("profile_{name}").as_str())
      .expect("Не удалось загрузить конфигурацию окна")
  }
}
impl Config for Profile {
  fn save(&self) {
    confy::store(
      APPLICATION_NAME,
      format!("profile_{}", self.name).as_str(),
      self,
    )
    .expect("Не удалось записать конфигурацию профиля")
  }
}
