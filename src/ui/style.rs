pub mod button {
  use crate::State;
  use iced::{Color, Theme, widget::button};

  pub fn active_profile(
    theme: &Theme,
    status: button::Status,
    state: &State,
    number: u8,
  ) -> button::Style {
    match state.active_profile_num {
      Some(i) => match i == number {
        true => button::Style {
          background: Some(iced::Background::Color(Color::parse("#778fe6").unwrap())),
          ..Default::default()
        },
        false => button::primary(theme, status),
      },
      None => button::primary(theme, status),
    }
  }
}
