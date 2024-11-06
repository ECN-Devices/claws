use iced::Alignment;
use iced::{
    widget::{button, column, container, row, text, Button},
    Element, Length, Pixels, Renderer, Theme,
};

use crate::application::{Claws, Message};

const BUTTON_SPACING: u16 = 30;
const BUTTON_PADDING: u16 = 10;

const HEADING_SIZE: u16 = 30;

#[derive(Debug, Clone, Default)]
pub enum Screen {
    #[default]
    Profile,
    Settings,
    Updater,
    ConnectedDeviceNotFound,
    ExperimentalTab,
}

impl Screen {
    fn name(&self) -> String {
        match self {
            Screen::Profile => "Профили".to_string(),
            Screen::Settings => "Настройки".to_string(),
            Screen::Updater => "Обновление".to_string(),
            Screen::ConnectedDeviceNotFound => "Устройство не найдено".to_string(),
            Screen::ExperimentalTab => "Экспериментальные настройки".to_string(),
        }
    }
}

pub fn get_screen_content(claws: &Claws) -> Element<'_, Message, Theme, Renderer> {
    match claws.screen {
        Screen::Profile => {
            let screen_name = text(claws.screen.name())
                .size(HEADING_SIZE)
                .width(Length::Fill)
                .height(Length::Fixed(40.));

            let buttons_row_top = column![
                create_keypad_button("btn1".to_string(), Message::ButtonClicked),
                create_keypad_button("btn2".to_string(), Message::ButtonClicked),
                create_keypad_button("btn3".to_string(), Message::ButtonClicked),
                create_keypad_button("btn4".to_string(), Message::ButtonClicked),
            ]
            .spacing(Pixels::from(BUTTON_SPACING))
            .padding(BUTTON_PADDING);

            let buttons_row_middle = column![
                create_keypad_button("btn5".to_string(), Message::ButtonClicked),
                create_keypad_button("btn6".to_string(), Message::ButtonClicked),
                create_keypad_button("btn7".to_string(), Message::ButtonClicked),
                create_keypad_button("btn8".to_string(), Message::ButtonClicked),
            ]
            .spacing(Pixels::from(BUTTON_SPACING))
            .padding(BUTTON_PADDING);

            let buttons_row_bottom = column![
                create_keypad_button("btn9".to_string(), Message::ButtonClicked),
                create_keypad_button("btn10".to_string(), Message::ButtonClicked),
                create_keypad_button("btn11".to_string(), Message::ButtonClicked),
                create_keypad_button("btn12".to_string(), Message::ButtonClicked),
            ]
            .spacing(Pixels::from(BUTTON_SPACING))
            .padding(BUTTON_PADDING);

            let buttons_row_underground = column![
                create_keypad_button("btn13".to_string(), Message::ButtonClicked),
                create_keypad_button("btn14".to_string(), Message::ButtonClicked),
                create_keypad_button("btn15".to_string(), Message::ButtonClicked),
                create_keypad_button("btn16".to_string(), Message::ButtonClicked),
            ]
            .spacing(Pixels::from(BUTTON_SPACING))
            .padding(BUTTON_PADDING);

            let buttons_container = row![
                buttons_row_top,
                buttons_row_middle,
                buttons_row_bottom,
                buttons_row_underground
            ];

            container(column![screen_name, buttons_container].spacing(10))
                .padding(10)
                .into()
        }
        Screen::Settings => {
            let screen_name = text(claws.screen.name())
                .size(HEADING_SIZE)
                .width(Length::Fill)
                .height(Length::Fill);

            container(column![screen_name].spacing(10))
                .padding(10)
                .into()
        }
        Screen::Updater => {
            let screen_name = text(claws.screen.name())
                .size(HEADING_SIZE)
                .width(Length::Fill)
                .height(Length::Fill);

            container(column![screen_name].spacing(10))
                .padding(10)
                .into()
        }
        Screen::ConnectedDeviceNotFound => {
            let text_on_screen = text(claws.screen.name())
                .size(HEADING_SIZE)
                .width(Length::Fill)
                .height(Length::Fill)
                .center();

            container(column![text_on_screen]).padding(10).into()
        }
        Screen::ExperimentalTab => {
            let screen_name = text(claws.screen.name())
                .size(HEADING_SIZE)
                .width(Length::Fill)
                .height(Length::Fixed(40.));

            let buttons = container(column![row![
                create_keypad_button("UpdateConfigFile".to_string(), Message::UpdateConfigFile),
                create_keypad_button("WritePort".to_string(), Message::WritePort),
                create_keypad_button("ReadPort".to_string(), Message::ReadPort),
                create_keypad_button("WriteAndReadPort".to_string(), Message::WriteAndReadPort),
                create_keypad_button("PrintAny".to_string(), Message::PrintAny),
            ]
            .spacing(10),]);

            container(column![screen_name, buttons].spacing(10))
                .padding(10)
                .into()
        }
    }
}

fn create_keypad_button<'a>(button_text: String, on_press: Message) -> Button<'a, Message> {
    button(
        text(button_text)
            .size(10)
            .align_x(Alignment::End)
            .align_y(Alignment::End),
    )
    .on_press(on_press)
    .height(110)
    .width(80)
}
