// Убираем консоль при старте приложения на windows
// #![windows_subsystem = "windows"]

use application::Lapa; // Импортируем структуру приложения
use fonts::{load, set, UI_FONT_MEDIUM}; // Загружаем шрифты
use iced::{
    window::{self, icon},
    Pixels, Size,
};

mod application; // Импортируем модуль приложения
mod configuration;
mod fonts; // Импортируем модуль шрифтов
mod screens; // Импортируем модуль состояния
mod tests;

static WINDOW_ICON: &[u8] = include_bytes!("../icons/lapa.ico");

fn main() -> iced::Result {
    env_logger::init();

    // Даем шрифтам имена
    set();

    let icon = icon::from_file_data(WINDOW_ICON, None);

    let iced_settings = iced::Settings {
        default_text_size: Pixels::from(18),
        default_font: UI_FONT_MEDIUM.clone().into(),
        fonts: load(),
        antialiasing: true,
        ..iced::Settings::default()
    };

    let window_settings = window::Settings {
        size: Size::new(800., 600.),
        min_size: Some(Size::new(600., 600.)),
        resizable: true,
        exit_on_close_request: true,
        icon: icon.ok(),
        ..window::Settings::default()
    };

    iced::application(Lapa::title, Lapa::update, Lapa::view)
        .settings(iced_settings)
        .window(window_settings)
        .theme(Lapa::theme)
        .run_with(Lapa::new)
}
