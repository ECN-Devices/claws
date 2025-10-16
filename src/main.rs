//Убираем консоль при старте приложения на windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
  data::{device::Device, stick::Stick},
  hardware::serial::buttons::KeypadButton,
};
use assets::{APPLICATION_NAME, INTER_FONT, INTER_FONT_BYTES, WINDOW_ICON};
use data::{profiles::Profile, window::Window};
use hardware::{buffers::Buffers, serial::Keypad};
use iced::{
  Pixels, Point, Size,
  window::{Position, icon},
};
use std::borrow::Cow;
use ui::pages::Pages;
use utils::logger::init_logger;

mod assets;
mod data;
mod errors;
mod hardware;
mod ui;
mod utils;

/// Глобальное состояние приложения Iced.
#[derive(Debug, Clone, Default)]
pub struct State {
  /// Номер активного профиля на устройстве, если известен
  pub active_profile_num: Option<u8>,

  /// Разрешение на запись комбинации клавиш/стика из UI
  pub allow_write: bool,

  /// Двунаправленные буферы обмена с устройством
  pub buffers: Buffers,

  /// Текущее редактируемое состояние кнопки/стика
  pub button: KeypadButton,

  /// Информация об устройстве
  pub device_info: Device,

  /// Флаг записи в ПЗУ (true) или ОЗУ (false)
  pub is_rom: bool,

  /// Дескриптор последовательного порта и его состояние
  pub keypad: Keypad,

  /// Текущая страница
  pub pages: Pages,

  /// Активный профиль
  pub profile: Profile,
  pub profile_vec: Vec<(usize, Profile)>,
  profile_id: usize,

  pub stick_callibrate: bool,
  pub stick_callibrate_time: Option<std::time::Instant>,
  pub stick_info: Stick,
  pub stick_show_calibrate_parameters: bool,

  /// Таймер для автоотмены режима записи
  pub time_write: Option<std::time::Instant>,

  /// Параметры окна
  pub window_settings: Window,
}

/**
Главная точка входа приложения.
Эта функция выполняет следующие действия:
1. Инициализирует логгер для записи диагностической информации
2. Загружает конфигурацию окна из файла
3. Устанавливает иконку приложения
4. Настраивает параметры приложения Iced
5. Настраивает параметры окна
6. Запускает приложение с указанными настройками
# Возвращает
Возвращает `Result` из Iced, который может содержать ошибку, если что-то пошло не так
во время выполнения приложения.
*/
fn main() -> iced::Result {
  // Инициализация логгера env_logger
  init_logger();

  let window_config = Window::load();

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
      width: 1069.,
      height: 540.,
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
  iced::application(State::title, State::update, State::view)
    .settings(iced_settings) // Установка настроек приложения
    .window(window_settings) // Установка настроек окна
    .theme(State::theme)
    .subscription(State::subscription)
    .run_with(State::new) // Запуск приложения с указанным стартовым состоянием
}
