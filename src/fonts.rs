use std::borrow::Cow;
use std::sync::OnceLock;

use iced::font::Weight;

pub static UI_FONT: Font = Font::new(Weight::Medium);

#[derive(Debug, Clone)]
pub struct Font {
    weight: Weight,
    inner: OnceLock<iced::Font>,
}

impl Font {
    const fn new(weight: Weight) -> Self {
        Self {
            weight,
            inner: OnceLock::new(),
        }
    }

    fn set(&self, name: String) {
        let name = Box::leak(name.into_boxed_str());
        let weight = match &self.weight {
            Weight::Thin => Weight::Thin,
            Weight::ExtraLight => Weight::ExtraLight,
            Weight::Light => Weight::Light,
            Weight::Normal => Weight::Normal,
            Weight::Medium => Weight::Medium,
            Weight::Semibold => Weight::Semibold,
            Weight::Bold => Weight::Bold,
            Weight::ExtraBold => Weight::ExtraBold,
            Weight::Black => Weight::Black,
        };

        let _ = self.inner.set(iced::Font {
            weight,
            ..iced::Font::with_name(name)
        });
    }
}

impl From<Font> for iced::Font {
    fn from(value: Font) -> Self {
        value.inner.get().copied().expect("font is set on startup")
    }
}

pub fn set() {
    UI_FONT.set("JetBrains Mono".to_string())
}

pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![include_bytes!("../fonts/JetBrainsMono-Medium.ttf")
        .as_slice()
        .into()]
}
