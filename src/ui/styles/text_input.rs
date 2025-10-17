use iced::{Border, Color, Theme, widget::text_input};

use crate::ui::styles::BORDER_RADIUS;

/**
Создает стиль поля ввода с прозрачной границей

Применяет прозрачную границу к полю ввода, используя базовый стиль темы.

# Аргументы
* `theme` - Текущая тема приложения
* `status` - Состояние поля ввода (фокус, ввод и т.д.)

# Возвращает
Стиль поля ввода с прозрачной границей
*/
pub fn rounding(theme: &Theme, status: text_input::Status) -> text_input::Style {
  text_input::Style {
    border: Border {
      color: Color::TRANSPARENT,
      radius: BORDER_RADIUS.into(),
      ..Default::default()
    },
    ..text_input::default(theme, status)
  }
}
