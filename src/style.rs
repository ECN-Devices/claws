// use iced::{
//     border::Radius,
//     widget::button::{self, Status},
//     Border, Color,
// };

// pub enum CustomTheme {
//     Light,
//     Dark,
// }

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

// #[derive(Default)]
// pub enum ButtonClass {
//     #[default]
//     Default,
//     Mod,
// }

// impl button::Catalog for CustomTheme {
//     type Class<'a> = ButtonClass;

//     fn default<'a>() -> Self::Class<'a> {
//         ButtonClass::Default
//     }

//     fn style(&self, class: &Self::Class<'_>, status: Status) -> button::Style {
//         button::Style {
//             background: match status {
//                 Status::Hovered => match class {
//                     ButtonClass::Default => Some(iced::Background::Color(Color::default())),
//                     ButtonClass::Mod => {
//                         Some(iced::Background::Color(Color::from_rgb8(250, 189, 47)))
//                     }
//                 },
//                 Status::Pressed => match class {
//                     ButtonClass::Default => Some(iced::Background::Color(Color::default())),
//                     ButtonClass::Mod => {
//                         Some(iced::Background::Color(Color::from_rgb8(250, 189, 47)))
//                     }
//                 },
//                 _ => Some(iced::Background::Color(Color::default())),
//             },
//             border: Border {
//                 color: Color::from_rgb8(50, 30, 10),
//                 width: 0.5,
//                 radius: Radius::new(8),
//             },
//             ..Default::default()
//         }
//     }
// }

// background: match status {
//     Status::Hovered => Some(iced::Background::Color(Color::from_rgb8(250, 189, 47)),
//     Status::Pressed => todo!(),
//     _ => todo!(),

// impl button::Catalog for Theme {
//     type Class<'a> = ButtonClass;

//     fn default<'a>() -> Self::Class<'a> {
//         ButtonClass::default()
//     }

//     fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
//         let pallete = self.palette();
//         button::Style {
//             background: {
//                 match status {
//                     button::Status::Active => Some(Background::Color(Color::default())),
//                     button::Status::Hovered => Some(Background::Color(Color::default())),
//                     _ => Some(Background::Color(Color::default())),
//                 }
//             },
//             ..Default::default()
//         }
//     }
// }
