use std::borrow::Cow;
use std::sync::OnceLock;

use iced::font;

pub static JETBRAINS_MONO_MEDIUM: Font = Font::new(false);

#[derive(Debug, Clone)]
pub struct Font {
    bold: bool,
    inner: OnceLock<iced::Font>,
}

impl Font {
    const fn new(bold: bool) -> Self {
        Self {
            bold,
            inner: OnceLock::new(),
        }
    }

    fn set(&self, name: String) {
        let name = Box::leak(name.into_boxed_str());
        let weight = if self.bold {
            font::Weight::Bold
        } else {
            font::Weight::Normal
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
    JETBRAINS_MONO_MEDIUM.set("JetBrainsMono-Medium".to_string())
}

pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![include_bytes!("../fonts/JetBrainsMono-Medium.ttf")
        .as_slice()
        .into()]
}
