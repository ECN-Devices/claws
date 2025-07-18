//Убираем консоль при старте приложения на windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use assets::{APPLICATION_NAME, INTER_FONT, INTER_FONT_BYTES, WINDOW_ICON};
use data::ConfigWindow;
use hardware::Keypad;
use iced::{
  Pixels, Point, Size,
  window::{Position, icon},
};
use std::borrow::Cow;
use ui::pages::Pages;
use utils::logger::init_logger;

mod assets;
mod data;
mod hardware;
mod ui;
mod utils;

#[derive(Debug, Clone, Default)]
pub struct App {
  pub keypad: Keypad,
  pub pages: Pages,
  pub window_settings: ConfigWindow,
}

/** Главная функция приложения.
 * Эта функция инициализирует приложение, настраивает логирование, загружает шрифты,
 * устанавливает иконку и конфигурирует параметры окна и приложения перед его запуском.
 * # Возвращает
 * Возвращает результат выполнения приложения типа `iced::Result`.
 */
fn main() -> iced::Result {
  // Инициализация логгера env_logger
  init_logger();

  let window_config = ConfigWindow::load();

  // Импорт иконки приложения из файла
  let icon = icon::from_file_data(WINDOW_ICON, None);

  // Создание настроек приложения iced
  let iced_settings = iced::Settings {
    id: Some(APPLICATION_NAME.to_string().to_lowercase()),
    default_text_size: Pixels::from(18), // Установка размера шрифта по умолчанию
    default_font: INTER_FONT,            // Установка шрифта по умолчанию
    fonts: vec![Cow::Borrowed(INTER_FONT_BYTES)], // Загрузка шрифтов
    antialiasing: true,                  // Включение сглаживания
  };

  // Создание настроек окна
  let window_settings = iced::window::Settings {
    size: Size {
      width: window_config.width,
      height: window_config.height,
    }, // Установка размера окна
    min_size: Some(Size {
      width: 600.,
      height: 660.,
    }), // Установка минимального размера окна
    position: Position::Specific(Point {
      x: window_config.x,
      y: window_config.y,
    }),
    resizable: true,             // Включение масштабируемости приложения
    exit_on_close_request: true, // Включение запроса выхода
    icon: icon.ok(),             // Установка иконки приложения
    ..Default::default()
  };

  // Создание приложения iced с указанными настройками
  iced::application(App::title, App::update, App::view)
    .settings(iced_settings) // Установка настроек приложения
    .window(window_settings) // Установка настроек окна
    .theme(App::theme)
    .subscription(App::subscription)
    .run_with(App::new) // Запуск приложения с указанным стартовым состоянием
}
