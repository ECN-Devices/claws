use iced::{
    widget::{button, column, container, row, text, Button},
    Alignment::Center,
    Element,
    Length::{self, Shrink},
    Renderer, Theme,
};

use crate::application::{Claws, Message};

#[derive(Debug, Clone, Default)]
pub enum Screen {
    #[default]
    Profile,
    Settings,
    Updater,
}

pub fn get_screen_content(lapa: &Lapa) -> Element<'_, Message, Theme, Renderer> {
    match lapa.screen {
pub fn get_screen_content(claws: &Claws) -> Element<'_, Message, Theme, Renderer> {
    match claws.screen {
        Screen::Profile => {
            let screen_name = text("Профили")
                .size(30)
                .width(Length::Fill)
                .height(Length::Fixed(40.));

            let buttons = container(
                row![
                    create_keypad_button("btn1".to_string(), Message::ButtonClicked),
                    create_keypad_button("UpdateConfigFile".to_string(), Message::UpdateConfigFile),
                    create_keypad_button("WriteAndReadPort".to_string(), Message::WriteAndReadPort),
                    create_keypad_button("ReadPort".to_string(), Message::ReadPort)
                ]
                .spacing(10),
            );

            container(column![screen_name, buttons].spacing(10))
                .padding(10)
                .into()
        }
        Screen::Settings => {
            let screen_name = text("Настройки")
                .size(30)
                .width(Length::Fill)
                .height(Length::Fill);

            container(column![screen_name].spacing(10))
                .padding(10)
                .into()
        }
        Screen::Updater => {
            let screen_name = text("Обновление")
                .size(30)
                .width(Length::Fill)
                .height(Length::Fill);

            container(column![screen_name].spacing(10))
                .padding(10)
                .into()
        }
    }
}

fn create_keypad_button<'a>(button_text: String, on_press: Message) -> Button<'a, Message> {
    button(text(button_text).align_x(Center).align_y(Center))
        .on_press(on_press)
        .height(Shrink)
        .width(Shrink)
    // .height(100)
    // .width(80)
}
