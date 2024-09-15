use iced::{
    border::Radius,
    widget::{
        button,
        container::{self, Catalog},
    },
    Background, Border, Color,
};

use container::Style;

pub struct Theme;

// #[derive(Default)]
// pub enum Container {
//     #[default]
//     Default,
//     BlackContainer,
// }

// impl Catalog for Theme {
//     type Class<'a> = Container;

//     fn default<'a>() -> Self::Class<'a> {
//         Container::Default
//     }

//     fn style(&self, class: &Self::Class<'_>) -> Style {
//         match class {
//             Container::Default => Style::default(),
//             Container::BlackContainer => Style {
//                 background: Some(Background::Color(Color::from_rgb8(49, 50, 68))),
//                 border: Border {
//                     radius: Radius::new(25),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         }
//     }
// }

// pub enum Button {}

// impl button::Catalog for Button {
//     type Class<'a>;

//     fn default<'a>() -> Self::Class<'a> {
//         todo!()
//     }

//     fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
//         todo!()
//     }
// }
