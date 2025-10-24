use iced::{Background, Border, Theme, border::Radius, color, widget::container::Style};

use crate::ui::styles::BORDER_RADIUS;

/**
Создает стиль контейнера для заголовка с закругленными верхними углами

Используется для заголовков секций с закругленными верхними углами
и соответствующим цветом фона для светлой и темной темы.

# Аргументы
* `theme` - Текущая тема приложения (светлая/темная)

# Возвращает
Стиль контейнера с закругленными верхними углами
*/
pub fn round_bordered_box_header(theme: &Theme) -> Style {
  match theme {
    Theme::Light => Style {
      background: Some(Background::Color(color!(0xd0d0d0))),
      border: Border {
        radius: Radius {
          top_left: BORDER_RADIUS,
          top_right: BORDER_RADIUS,
          ..Default::default()
        },
        ..Default::default()
      },
      ..Default::default()
    },
    Theme::Dark => Style {
      background: Some(Background::Color(color!(0x292929))),
      border: Border {
        radius: Radius {
          top_left: BORDER_RADIUS,
          top_right: BORDER_RADIUS,
          ..Default::default()
        },
        ..Default::default()
      },
      ..Default::default()
    },
    _ => Style::default(),
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
pub fn round_bordered_box(theme: &Theme) -> Style {
  match theme {
    Theme::Light => Style {
      background: Some(Background::Color(color!(0xededed))),
      border: Border {
        radius: Radius {
          bottom_right: BORDER_RADIUS,
          bottom_left: BORDER_RADIUS,
          ..Default::default()
        },
        ..Default::default()
      },
      ..Default::default()
    },
    Theme::Dark => Style {
      background: Some(Background::Color(color!(0x333333))),
      border: Border {
        radius: Radius {
          bottom_right: BORDER_RADIUS,
          bottom_left: BORDER_RADIUS,
          ..Default::default()
        },
        ..Default::default()
      },
      ..Default::default()
    },
    _ => Style::default(),
  }
}
