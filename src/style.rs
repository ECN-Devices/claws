use iced::{Background, Theme, border, color, widget::button};

pub struct Style;

impl Style {
    pub fn button(_theme: &Theme, _status: button::Status) -> button::Style {
        match _status {
            button::Status::Active | button::Status::Disabled => button::Style {
                background: Some(Background::Color(color!(0x7e7366))),
                text_color: color!(0xeee5cf),
                border: border::rounded(3),
                ..Default::default()
            },
            button::Status::Hovered | button::Status::Pressed => button::Style {
                background: Some(Background::Color(color!(0x994f3f))),
                text_color: color!(0xffffff),
                border: border::rounded(3),
                ..Default::default()
            },
        }
    }
}
