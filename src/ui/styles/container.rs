use crate::ui::styles::BORDER_RADIUS;
use iced::{Background, Border, Color, Theme, border::Radius, widget::container::Style};

pub fn round_bordered_box_header(theme: &Theme) -> Style {
  match theme {
    Theme::Light => Style {
      background: Some(Background::Color(Color::parse("#d0d0d0").unwrap())),
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
      background: Some(Background::Color(Color::parse("#292929").unwrap())),
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

pub fn round_bordered_box(theme: &Theme) -> Style {
  match theme {
    Theme::Light => Style {
      background: Some(Background::Color(Color::parse("#ededed").unwrap())),
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
      background: Some(Background::Color(Color::parse("#333333").unwrap())),
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
