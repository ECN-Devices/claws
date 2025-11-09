/*!
Модуль стилей пользовательского интерфейса

Содержит константы и функции для создания единообразных стилей
элементов интерфейса, включая кнопки, контейнеры и поля ввода.
*/

pub mod button;
pub mod container;
pub mod text_input;

/// Базовая ширина окна по умолчанию
pub const WINDOW_WIDTH: f32 = 800.;
/// Базовая высота окна по умолчанию
pub const WINDOW_HEIGH: f32 = 600.;

/// Радиус закругления углов элементов интерфейса
pub const BORDER_RADIUS: f32 = 5.;

/// Стандартный отступ между элементами интерфейса
pub const SPACING: u16 = 10;

/// Стандартные внутренние отступы контейнеров
pub const PADDING: u16 = 10;

/// Размер шрифта для заголовков
pub const HEADING_SIZE: u16 = 30;

/// Стандартная высота кнопок
pub const BUTTON_HEIGH_PROFILE: u16 = 100;

/// Стандартная ширина кнопок
pub const BUTTON_WIDTH_PROFILE: u16 = 90;

pub const BUTTON_HEIGH: u16 = 35;

pub const RULE_WIDTH: u16 = 2;
