// Убираем консоль при старте приложения на windows
// #![windows_subsystem = "windows"]

use std::env;

use application::Claws;
use fonts::{load, set, UI_FONT_MEDIUM};
use iced::{
    window::{self, icon},
    Pixels, Size,
};

mod application;
mod configuration;
mod fonts;
mod screens;
mod tests;

static WINDOW_ICON: &[u8] = include_bytes!("../icons/claws.ico");

fn main() -> iced::Result {
    env::set_var("RUST_LOG", "claws");

    // Инициализация логгера env_logger
    env_logger::init();

    // Загрузка шрифтов
    set();

    // Импорт иконки приложения из файла
    let icon = icon::from_file_data(WINDOW_ICON, None);

    // Создание настроек приложения iced
    let iced_settings = iced::Settings {
        id: Some(String::from("Claws")),
        default_text_size: Pixels::from(18), // Установка размера шрифта по умолчанию
        default_font: UI_FONT_MEDIUM.clone().into(), // Установка шрифта по умолчанию
        fonts: load(),                       // Загрузка шрифтов
        antialiasing: true,                  // Включение сглаживания
    };

    // Создание настроек окна
    let window_settings = window::Settings {
        size: Size::new(800., 600.),           // Установка размера окна
        min_size: Some(Size::new(600., 600.)), // Установка минимального размера окна
        resizable: true,                       // Включение масштабируемости приложения
        exit_on_close_request: true,           // Включение запроса выхода
        icon: icon.ok(),                       // Установка иконки приложения
        ..window::Settings::default()
    };

    // Создание приложения iced с указанными настройками
    iced::application(Claws::title, Claws::update, Claws::view)
        .settings(iced_settings) // Установка настроек приложения
        .window(window_settings) // Установка настроек окна
        .theme(Claws::theme) // Установка темы приложения
        .run_with(Claws::new) // Запуск приложения с указанным стартовым состоянием
}
