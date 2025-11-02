use std::hint::unreachable_unchecked;

use iced::{Background, Border, Theme, widget::container::Style};

use crate::ui::styles::BORDER_RADIUS;

pub fn rounded(theme: &Theme, radius: f32) -> Style {
  let background = match theme {
    Theme::Light => Some(Background::Color(
      theme
        .extended_palette()
        .background
        .strong
        .color
        .scale_alpha(0.95),
    )),
    Theme::Dark => Some(Background::Color(
      theme
        .extended_palette()
        .background
        .weak
        .color
        .scale_alpha(0.05),
    )),
    // SAFETY:  The application's theme cannot be Light or Dark because they are explicitly specified in the theme() function in src/ui/mod.rs
    _ => unsafe { unreachable_unchecked() },
  };

  Style {
    background,
    border: Border {
      radius: radius.into(),
      ..Default::default()
    },
    ..Default::default()
  }
}

/**
Создает стиль контейнера для содержимого с закругленными нижними углами

Используется для основного содержимого секций с закругленными нижними углами
и соответствующим цветом фона для светлой и темной темы.

# Аргументы
* `theme` - Текущая тема приложения (светлая/темная)

# Возвращает
Стиль контейнера с закругленными нижними углами
*/
pub fn rounded_inside(theme: &Theme) -> Style {
  let background = match theme {
    Theme::Light => Some(Background::Color(
      theme
        .extended_palette()
        .background
        .weak
        .color
        .scale_alpha(0.9),
    )),
    Theme::Dark => Some(Background::Color(
      theme
        .extended_palette()
        .background
        .strong
        .color
        .scale_alpha(0.1),
    )),
    // SAFETY:  The application's theme cannot be Light or Dark because they are explicitly specified in the theme() function in src/ui/mod.rs
    _ => unsafe { unreachable_unchecked() },
  };

  Style {
    background,
    border: Border {
      radius: BORDER_RADIUS.into(),
      ..Default::default()
    },
    ..Default::default()
  }
}
