use iced::{Border, Color, Theme, widget::text_input};

pub fn rounding(theme: &Theme, status: text_input::Status) -> text_input::Style {
  text_input::Style {
    border: Border {
      color: Color::TRANSPARENT,
      ..Default::default()
    },
    ..text_input::default(theme, status)
  }
}
