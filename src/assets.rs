use iced::{Font, font::Weight};

/// Имя приложения, используется как идентификатор конфигурации и логгера
pub const APPLICATION_NAME: &str = "Claws";

/// Версия приложения из Cargo.toml
pub const APPLICATION_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Иконка окна приложения (ICO)
pub static WINDOW_ICON: &[u8] = include_bytes!("../assets/icons/claws.ico");

/// Шрифт по умолчанию (Inter Medium)
pub const INTER_FONT: Font = Font {
  weight: Weight::Medium,
  ..Font::with_name("Inter")
};

/// Байты шрифта Inter Medium для регистрации в Iced
pub const INTER_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-Medium.ttf");
