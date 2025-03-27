use std::borrow::Cow;
use std::sync::OnceLock;

use iced::font::Weight;

/** Определение статических переменных для шрифтов с весом Medium и Normal.
 *
 * Эти переменные представляют собой шрифты, которые могут быть использованы в пользовательском интерфейсе.
 */
pub static UI_FONT_MEDIUM: Font = Font::new(Weight::Medium); // Шрифт с весом Medium
pub static UI_FONT_NORMAL: Font = Font::new(Weight::Normal); // Шрифт с весом Normal

/** Структура `Font` представляет шрифт с определенным весом.
 *
 * Она содержит информацию о весе шрифта.
 *
 * # Параметры
 *
 * `weight`: Вес шрифта.
 */
#[derive(Debug, Clone)]
pub struct Font {
    /// Вес шрифта
    weight: Weight,
    inner: OnceLock<iced::Font>,
}

// Реализация методов для структуры Font
impl Font {
    /** Конструктор для создания нового шрифта с указанным весом.
     * # Параметры
     * `weight`: Вес шрифта, который будет установлен.
     */
    const fn new(weight: Weight) -> Self {
        Self {
            weight,
            inner: OnceLock::new(),
        }
    }

    /** Метод для установки имени шрифта.
     * # Параметры
     * `name`: Имя шрифта, которое будет установлено.
     */
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
    /** Преобразует экземпляр `Font` в `iced::Font`.
     * # Возвращает
     * Возвращает `iced::Font`, который соответствует данному экземпляру `Font`.
     */
    fn from(value: Font) -> Self {
        value
            .inner
            .get()
            .copied()
            .expect("Шрифт устанавливается при запуске")
    }
}

/** Функция для установки имен шрифтам.
 * Эта функция устанавливает имена для статических шрифтов `UI_FONT_MEDIUM` и `UI_FONT_NORMAL`.
 */
pub fn set() {
    UI_FONT_MEDIUM.set("Inter".to_string());
    UI_FONT_NORMAL.set("Inter".to_string());
}

/** Функция для загрузки шрифтов.
 *
 * Эта функция загружает шрифты из файловой системы и возвращает вектор загруженных шрифтов в
 * виде байтов.
 * # Возвращает
 * Возвращает вектор, содержащий загруженные шрифты в виде `Cow<'static, [u8]>`.
 */
pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!("../fonts/Inter-Medium.ttf")
            .as_slice()
            .into(),
    ]
}
