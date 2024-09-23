use std::borrow::Cow;
use std::sync::OnceLock;

use iced::font::Weight;

// Определение статических переменных для шрифтов с весом Medium и Normal
pub static UI_FONT_MEDIUM: Font = Font::new(Weight::Medium); // Шрифт с весом Medium
pub static UI_FONT_NORMAL: Font = Font::new(Weight::Normal); // Шрифт с весом Normal

// Определение структуры шрифта
#[derive(Debug, Clone)]
pub struct Font {
    weight: Weight, // Вес шрифта
    inner: OnceLock<iced::Font>,
}

// Реализация методов для структуры Font
impl Font {
    // Конструктор для создания нового шрифта с указанным весом
    const fn new(weight: Weight) -> Self {
        Self {
            weight,
            inner: OnceLock::new(),
        }
    }

    // Метод для установки имени шрифта
    fn set(&self, name: String) {
        let name = Box::leak(name.into_boxed_str());
        let _ = self.inner.set(iced::Font {
            weight: self.weight,
            ..iced::Font::with_name(name)
        });
    }
}

// Реализация преобразования Font в iced::Font
impl From<Font> for iced::Font {
    fn from(value: Font) -> Self {
        value
            .inner
            .get()
            .copied()
            .expect("Шрифт устанавливается при запуске")
    }
}

// Функция для установки имен шрифтам
pub fn set() {
    UI_FONT_MEDIUM.set("Inter".to_string());
    UI_FONT_NORMAL.set("Inter".to_string());
}

// Функция для загрузки шрифтов
pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![include_bytes!("../fonts/Inter-Medium.ttf")
        .as_slice()
        .into()]
}
