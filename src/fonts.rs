use std::borrow::Cow;
use std::sync::OnceLock;

use iced::font::Weight;

pub static UI_FONT_MEDIUM: Font = Font::new(Weight::Medium);
pub static UI_FONT_NORMAL: Font = Font::new(Weight::Normal);

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
        let _ = self.inner.set(iced::Font {
            weight: self.weight,
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
    UI_FONT_MEDIUM.set("Inter".to_string());
    UI_FONT_NORMAL.set("Inter".to_string());
}

pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![include_bytes!("../fonts/Inter-Medium.ttf")
        .as_slice()
        .into()]
}
