use iced::Alignment;
use iced::{
    widget::{button, column, container, row, text, Button},
    Element, Length, Pixels, Renderer, Theme,
};

use crate::application::{Claws, Message};
use crate::configuration::keypad_port_commands::{DeviceCommands, KeypadCommands, ProfileCommands};

/// `BUTTON_SPACING` определяет расстояние между кнопками в пикселях.
const BUTTON_SPACING: u16 = 30;

/// `BUTTON_PADDING` определяет отступ вокруг кнопок в пикселях.
const BUTTON_PADDING: u16 = 10;

/// `HEADING_SIZE` определяет размер заголовка в пикселях.
const HEADING_SIZE: u16 = 30;

/** Перечисление `Screens` представляет различные экраны приложения.
 * Каждый вариант перечисления соответствует отдельному экрану, который может быть
 * отображен в пользовательском интерфейсе. По умолчанию выбран экран `Profile`.
 */
#[derive(Debug, Clone, Default)]
pub enum Screens {
    #[default]
    Profile,
    Settings,
    Updater,
    ConnectedDeviceNotFound,
    ExperimentalTab,
}

impl Screens {
    /// Возвращает имя текущего экрана в виде строки.
    fn name(&self) -> &str {
        match self {
            Screens::Profile => "Профили",
            Screens::Settings => "Настройки",
            Screens::Updater => "Обновление",
            Screens::ConnectedDeviceNotFound => "Устройство не найдено",
            Screens::ExperimentalTab => "Экспериментальные настройки",
        }
    }
    /** Генерирует содержимое экрана в зависимости от текущего состояния приложения.
     * # Параметры
     * `claws`: Ссылка на экземпляр `Claws`, который содержит текущее состояние приложения.
     * # Возвращает
     * Возвращает элемент типа `Element`, который представляет содержимое текущего экрана.
     */
    pub fn get_screen_content(claws: &Claws) -> Element<'_, Message, Theme, Renderer> {
        match claws.screen {
            Screens::Profile => {
                let screen_name = text(claws.screen.name())
                    .size(HEADING_SIZE)
                    .width(Length::Fill)
                    .height(Length::Fixed(40.));

                let buttons_top = column![
                    create_keypad_button("#1", Message::ButtonClicked),
                    create_keypad_button("#2", Message::ButtonClicked),
                    create_keypad_button("#3", Message::ButtonClicked),
                    create_keypad_button("#4", Message::ButtonClicked),
                ]
                .spacing(Pixels::from(BUTTON_SPACING))
                .padding(BUTTON_PADDING);

                let buttons_middle = column![
                    create_keypad_button("#5", Message::ButtonClicked),
                    create_keypad_button("#6", Message::ButtonClicked),
                    create_keypad_button("#7", Message::ButtonClicked),
                    create_keypad_button("#8", Message::ButtonClicked),
                ]
                .spacing(Pixels::from(BUTTON_SPACING))
                .padding(BUTTON_PADDING);

                let buttons_bottom = column![
                    create_keypad_button("#9", Message::ButtonClicked),
                    create_keypad_button("#10", Message::ButtonClicked),
                    create_keypad_button("#11", Message::ButtonClicked),
                    create_keypad_button("#12", Message::ButtonClicked),
                ]
                .spacing(Pixels::from(BUTTON_SPACING))
                .padding(BUTTON_PADDING);

                let buttons_underground = column![
                    create_keypad_button("#13", Message::ButtonClicked),
                    create_keypad_button("#14", Message::ButtonClicked),
                    create_keypad_button("#15", Message::ButtonClicked),
                    create_keypad_button("#16", Message::ButtonClicked),
                ]
                .spacing(Pixels::from(BUTTON_SPACING))
                .padding(BUTTON_PADDING);

                let buttons_container = row![
                    buttons_top,
                    buttons_middle,
                    buttons_bottom,
                    buttons_underground
                ];

                container(column![screen_name, buttons_container].spacing(10))
                    .padding(10)
                    .into()
            }
            Screens::Settings => {
                let screen_name = text(claws.screen.name())
                    .size(HEADING_SIZE)
                    .width(Length::Fill)
                    .height(Length::Fill);

                container(column![screen_name].spacing(10))
                    .padding(10)
                    .into()
            }
            Screens::Updater => {
                let screen_name = text(claws.screen.name())
                    .size(HEADING_SIZE)
                    .width(Length::Fill)
                    .height(Length::Fill);

                container(column![screen_name].spacing(10))
                    .padding(10)
                    .into()
            }
            Screens::ConnectedDeviceNotFound => {
                let text_on_screen = text(claws.screen.name())
                    .size(HEADING_SIZE)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center();

                container(column![text_on_screen]).padding(10).into()
            }
            Screens::ExperimentalTab => {
                let screen_name = text(claws.screen.name())
                    .size(HEADING_SIZE)
                    .width(Length::Fill)
                    .height(Length::Fixed(40.));

                let buttons = container(
                    row![
                        create_keypad_button("UpdateConfigFile", Message::UpdateConfigFile),
                        create_keypad_button(
                            "WritePortProfile",
                            Message::WritePort(KeypadCommands::ProfileCommands(
                                ProfileCommands::RequestForAProfileName
                            ))
                        ),
                        create_keypad_button(
                            "WritePortDevice",
                            Message::WritePort(KeypadCommands::DeviceCommands(
                                DeviceCommands::RequestingDeviceInformation
                            ))
                        ),
                        create_keypad_button("PrintBuffer", Message::PrintBuffer),
                        create_keypad_button(
                            "TaskRequestingAsciiSwitchCodes",
                            Message::RequestingAsciiSwitchCodes
                        )
                    ]
                    .spacing(10),
                );

                container(column![screen_name, buttons].spacing(10))
                    .padding(10)
                    .into()
            }
        }
    }
}

/** Создает кнопку для клавиатуры.
 * # Параметры
 * `button_text`: Текст, который будет отображаться на кнопке.
 * `on_press`: Сообщение, которое будет отправлено при нажатии на кнопку.
 * # Возвращает
 * Возвращает экземпляр `Button`, который можно использовать в пользовательском интерфейсе.
 */
fn create_keypad_button(button_text: &str, on_press: Message) -> Button<'_, Message> {
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
