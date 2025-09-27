use crate::{State, ui::styles::BORDER_RADIUS};
use iced::{Border, Color, Theme, widget::button};

pub fn rounding(theme: &Theme, status: button::Status) -> button::Style {
  button::Style {
    border: Border {
      radius: BORDER_RADIUS.into(),
      ..Default::default()
    },
    ..button::primary(theme, status)
  }
}

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
        border: Border {
          radius: BORDER_RADIUS.into(),
          ..Default::default()
        },
        ..button::primary(theme, status)
      },
      false => button::Style {
        border: Border {
          radius: BORDER_RADIUS.into(),
          ..Default::default()
        },
        ..button::primary(theme, status)
      },
    },
    None => button::primary(theme, status),
  }
}

pub fn active_write(
  theme: &Theme,
  status: button::Status,
  state: &State,
  id: usize,
  is_stick: bool,
) -> button::Style {
  match (state.button.id == id, state.allow_write, is_stick) {
    (true, true, false) => button::Style {
      background: Some(iced::Background::Color(Color::parse("#778fe6").unwrap())),
      border: Border {
        radius: (BORDER_RADIUS * 2.).into(),
        ..Default::default()
      },
      ..button::primary(theme, status)
    },
    _ => button::Style {
      border: Border {
        radius: (BORDER_RADIUS * 2.).into(),
        ..Default::default()
      },
      ..button::primary(theme, status)
    },
  }
}

pub mod stick {
  use crate::{State, ui::styles::BORDER_RADIUS};
  use iced::{Border, Color, Theme, widget::button};

  pub fn active_write(
    theme: &Theme,
    status: button::Status,
    state: &State,
    id: usize,
    is_stick: bool,
  ) -> button::Style {
    match (state.button.id == id, state.allow_write, is_stick) {
      (true, true, true) => button::Style {
        background: Some(iced::Background::Color(Color::parse("#778fe6").unwrap())),
        border: Border {
          radius: (BORDER_RADIUS * 2.).into(),
          ..Default::default()
        },
        ..button::primary(theme, status)
      },
      _ => button::Style {
        border: Border {
          radius: (BORDER_RADIUS * 2.).into(),
          ..Default::default()
        },
        ..button::primary(theme, status)
      },
    }
  }
}
