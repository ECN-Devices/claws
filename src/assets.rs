use iced::{Font, font::Weight};

pub const APPLICATION_NAME: &str = "Claws";
pub static WINDOW_ICON: &[u8] = include_bytes!("../assets/icons/claws.ico");
pub const INTER_FONT: Font = Font {
  weight: Weight::Medium,
  ..Font::with_name("Inter")
};
pub const INTER_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-Medium.ttf");
