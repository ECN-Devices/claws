use crate::{
  assets::APPLICATION_NAME,
  ui::{WINDOW_HEIGH, WINDOW_WIDTH},
};
use log::error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

pub fn is_first_run() {
  let config_file = tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(get_config_dir())
    .unwrap()
    .join("window_settings.toml");
  if !config_file.exists() {
    let window_settings = WindowSettings::default();
    window_settings.save()
  }
}

pub async fn get_config_dir() -> tokio::io::Result<PathBuf> {
  let config_dir = match std::env::consts::OS {
    "linux" => dirs::config_dir()
      .ok_or_else(|| {
        std::io::Error::new(
          tokio::io::ErrorKind::NotFound,
          "Не могу найти папку '.config'",
        )
      })?
      .join(APPLICATION_NAME),
    "windows" => dirs::document_dir()
      .ok_or_else(|| {
        std::io::Error::new(
          tokio::io::ErrorKind::NotFound,
          "Не могу найти папку 'Документы'",
        )
      })?
      .join(APPLICATION_NAME),
    os => {
      return Err(tokio::io::Error::new(
        std::io::ErrorKind::Unsupported,
        format!("Система не поддерживается: {os}"),
      ));
    }
  };

  if !config_dir.exists() {
    fs::create_dir_all(&config_dir).await?;
  }

  Ok(config_dir)
}

pub async fn get_config_file(name: &str) -> std::io::Result<PathBuf> {
  let config_dir = get_config_dir().await?;
  let file_path = config_dir.join(name);

  if !file_path.exists() {
    std::fs::File::create(&file_path)?;
  }

  Ok(file_path)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

impl Default for WindowSettings {
  fn default() -> Self {
    Self {
      x: 600.,
      y: 660.,
      width: WINDOW_WIDTH,
      height: WINDOW_HEIGH,
    }
  }
}

impl WindowSettings {
  pub fn load() -> Self {
    let config_file = tokio::runtime::Runtime::new()
      .unwrap()
      .block_on(get_config_file("window_settings.toml"))
      .unwrap();

    match std::fs::read_to_string(config_file) {
      Ok(s) => match toml::from_str(s.as_str()) {
        Ok(s) => s,
        Err(e) => {
          error!("Не удалось прочитать конфигурационный файл: {e}");
          WindowSettings::default()
        }
      },
      Err(e) => {
        error!("Не удалось прочитать конфигурационный файл: {e}");
        WindowSettings::default()
      }
    }
  }

  pub fn save(&self) {
    let config_file = tokio::runtime::Runtime::new()
      .unwrap()
      .block_on(get_config_dir())
      .unwrap()
      .join("window_settings.toml");
    if let Ok(contents) = toml::to_string_pretty(self) {
      let _ = std::fs::write(config_file, contents);
    }
  }
}
