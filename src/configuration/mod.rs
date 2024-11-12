pub mod command;
pub mod config;
pub mod logger;
pub mod port;

pub const APPLICATION_NAME: &str = "Claws";
pub static WINDOW_ICON: &[u8] = include_bytes!("../../icons/claws.ico");
pub const ARRAY_LEN: usize = 9;
