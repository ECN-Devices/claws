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
  /// Координата X окна на экране
  pub x: f32,

  /// Координата Y окна на экране
  pub y: f32,

  /// Ширина окна в пикселях
  pub width: f32,

  /// Высота окна в пикселях
  pub height: f32,
}

impl Default for Window {
  /**
  Создает конфигурацию окна с параметрами по умолчанию

  Устанавливает стандартные размеры и позицию окна при первом запуске
  */
  fn default() -> Self {
    Self {
      x: 600.,              // Позиция X по умолчанию
      y: 660.,              // Позиция Y по умолчанию
      width: WINDOW_WIDTH,  // Стандартная ширина окна
      height: WINDOW_HEIGH, // Стандартная высота окна
    }
  }
}

impl Window {
  /**
  Загружает конфигурацию окна из файла конфигурации

  # Возвращает
  Загруженную конфигурацию окна или паникует при ошибке чтения

  # Паникует
  Если не удается загрузить конфигурацию из файла
  */
  pub fn load() -> Self {
    confy::load(APPLICATION_NAME, "window").expect("Не удалось загрузить конфигурацию окна")
  }
}
impl Config for Window {
  /**
  Сохраняет текущую конфигурацию окна в файл конфигурации

  # Паникует
  Если не удается записать конфигурацию в файл
  */
  fn save(&self) {
    confy::store(APPLICATION_NAME, "window", self).expect("Не удалось записать конфигурацию окна")
  }
}
