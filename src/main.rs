//Убираем консоль при старте приложения на windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use application::Claws;
use configuration::{APPLICATION_NAME, WINDOW_ICON, logger::init_logger};
use fonts::{UI_FONT_MEDIUM, load, set};
use iced::{
    Pixels, Size,
    window::{self, icon},
};

mod application;
mod configuration;
mod fonts;
mod screens;
mod tests;

/** Главная функция приложения.
 * Эта функция инициализирует приложение, настраивает логирование, загружает шрифты,
 * устанавливает иконку и конфигурирует параметры окна и приложения перед его запуском.
 * # Возвращает
 * Возвращает результат выполнения приложения типа `iced::Result`.
 */
fn main() -> iced::Result {
    // Инициализация логгера env_logger
    init_logger();

    // Загрузка шрифтов
    set();

    // Импорт иконки приложения из файла
    let icon = icon::from_file_data(WINDOW_ICON, None);

    // Создание настроек приложения iced
    let iced_settings = iced::Settings {
        id: Some(APPLICATION_NAME.to_string()),
        default_text_size: Pixels::from(18), // Установка размера шрифта по умолчанию
        default_font: UI_FONT_MEDIUM.clone().into(), // Установка шрифта по умолчанию
        fonts: load(),                       // Загрузка шрифтов
        antialiasing: true,                  // Включение сглаживания
    };

    // Создание настроек окна
    let window_settings = window::Settings {
        size: Size::new(800., 660.),           // Установка размера окна
        min_size: Some(Size::new(600., 660.)), // Установка минимального размера окна
        resizable: true,                       // Включение масштабируемости приложения
        exit_on_close_request: true,           // Включение запроса выхода
        icon: icon.ok(),                       // Установка иконки приложения
        ..window::Settings::default()
    };

    // Создание приложения iced с указанными настройками
    iced::application(Claws::title, Claws::update, Claws::view)
        .settings(iced_settings) // Установка настроек приложения
        .window(window_settings) // Установка настроек окна
        .subscription(Claws::subscription)
        .run_with(Claws::new) // Запуск приложения с указанным стартовым состоянием
}
