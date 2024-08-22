// Убираем консоль при старте приложения на windows
#![windows_subsystem = "windows"]

use application::{Lapa, Message}; // Импортируем структуру приложения
use fonts::{load, set, UI_FONT_MEDIUM}; // Загружаем шрифты
use iced::{
    window::{self, icon},
    Pixels, Size,
};
use image::ImageFormat;

mod application; // Импортируем модуль приложения
mod condition; // Импортируем модуль состояния
mod fonts; // Импортируем модуль шрифтов

static WINDOW_ICON: &[u8] = include_bytes!("../icons/lapa.ico");

fn main() -> iced::Result {
    // Даем шрифтам имена
    set();

    let window_icon = match image::load_from_memory_with_format(WINDOW_ICON, ImageFormat::Ico)
        .map(|i| (i.to_rgba8().into_raw(), i.width(), i.height()))
        .map_err(anyhow::Error::new)
        .and_then(|(i, width, height)| {
            icon::from_rgba(i, width, height).map_err(anyhow::Error::new)
        }) {
        Ok(icon) => icon,
        Err(_e) => todo!(),
    };

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
        icon: Some(window_icon),
        ..window::Settings::default()
    };

    iced::application(Lapa::title, Lapa::update, Lapa::view)
        .settings(iced_settings)
        .window(window_settings)
        .theme(Lapa::theme)
        .run_with(Lapa::new)
}
