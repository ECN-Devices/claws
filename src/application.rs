use std::{sync::Arc, time::Duration};

use iced::{
    widget::{column, container, row, svg, tooltip, Button, Tooltip},
    Alignment, Element,
    Length::{self, Fill},
    Subscription, Task,
};
use log::debug;

#[cfg(target_os = "windows")]
use log::error;

use tokio::{runtime::Builder, sync::Mutex};

use crate::{
    configuration::{
        config::{check_config_file, get_config_file, update_config_file},
        keypad_port_commands::{KeypadCommands, SwitchCommands, Value},
        port::{get_buffer, Keypad},
    },
    screens::Screens,
};

/// Структура приложения
#[derive(Debug, Clone)]
pub struct Claws {
    pub screen: Screens,
    pub keypad: Keypad,
}

/** Определение возможных действий в приложении
 *
 * Эти сообщения определяют возможные действия и взаимодействия в приложении.
 */
#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screens),
    ButtonClicked,
    UpdateConfigFile,
    ReadPort,
    WritePort(KeypadCommands),
    RequestingAsciiSwitchCodes,

    PrintBuffer,
    TaskPrintBuffer(()),

    TaskRequestingAsciiSwitchCodes(Result<String, serialport::Error>),
    TaskReadPort(Result<String, serialport::Error>),
    TaskWritePort(Result<(), serialport::Error>),
}

impl Claws {
    /// Эта функция инициализирует экран и порт.
    pub fn new() -> (Self, Task<Message>) {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        let port_name = runtime.block_on(self::Keypad::get_keypad_port()).clone();

        let initial_screen = match port_name.is_empty() {
            true => {
                #[cfg(debug_assertions)]
                {
                    Screens::default()
                }

                #[cfg(not(debug_assertions))]
                Screens::ConnectedDeviceNotFound
            }
            false => Screens::default(),
        };

        debug!(
            "Port name: {:?}, bytes: {:?}",
            port_name,
            port_name.as_bytes()
        );

        let keypad = match !port_name.is_empty() {
            true => {
                let serial_port = Arc::new(Mutex::new(
                    serialport::new(port_name, 115_200)
                        .timeout(Duration::from_millis(10))
                        .open()
                        .expect("Failed to open port"),
                ));

                #[cfg(target_os = "windows")]
                {
                    let mut port_lock = runtime.block_on(async { serial_port.lock().await });
                    if let Err(e) = port_lock.write_data_terminal_ready(true) {
                        error!("Ошибка при установке DTR: {}", e);
                    }
                }

                Keypad {
                    port: Some(serial_port),
                    is_open: true,
                }
            }
            false => Keypad {
                port: None,
                is_open: false,
            },
        };

        (
            Self {
                screen: initial_screen,
                keypad,
            },
            Task::none(),
        )
    }

    // Определение названия приложения
    pub fn title(&self) -> String {
        String::from("Claws")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        match message {
            Message::ChangeScreen(new_screen) => {
                // Определение текущего экран
                self.screen = new_screen.clone();
                Task::none()
            }
            Message::ButtonClicked => {
                println!("Кнопка нажата");
                Task::none()
            }
            Message::UpdateConfigFile => {
                // Асинхронная операция обновления конфигурационного файла
                let config_file = runtime.block_on(async {
                    check_config_file().await?;
                    update_config_file(get_config_file().await).await
                });

                // Вывод обновленного конфигурационного файла
                debug!("{:?}", config_file);
                Task::none()
            }
            Message::WritePort(data) => {
                // Проверяю открыт ли порт
                match self.keypad.is_open {
                    true => {
                        let serial_port = self.keypad.port.clone().unwrap();
                        let write_data_array = data.get_value();
                        let write_port =
                            self::Keypad::write_keypad_port(serial_port, write_data_array);
                        Task::perform(write_port, Message::TaskWritePort)
                    }
                    false => Task::none(),
                }
            }
            Message::ReadPort => {
                // Проверяю открыт ли порт
                match self.keypad.is_open {
                    true => {
                        let serial_port = self.keypad.port.clone().unwrap();
                        let read_port = self::Keypad::read_keypad_port(serial_port);
                        Task::perform(read_port, Message::TaskReadPort)
                    }
                    false => Task::none(),
                }
            }
            Message::RequestingAsciiSwitchCodes => {
                match self.keypad.is_open {
                    true => {
                        let serial_port = self.keypad.port.clone().unwrap();
                        for button_number in 1..=4 {
                            let write_data_array =
                                SwitchCommands::RequestingAsciiSwitchCodes(button_number)
                                    .get_value();

                            let _ = runtime.block_on(async {
                                self::Keypad::write_keypad_port(
                                    serial_port.clone(),
                                    write_data_array,
                                )
                                .await
                            });
                        }
                        Task::none()
                    }
                    false => Task::none(),
                }

                //let write_data_array = DeviceCommands::RequestingDeviceInformation.value();

                //let write_data_array = SwitchCommands::RequestingAsciiSwitchCodes.value();
                //let write_port = self::Keypad::write_keypad_port(serial_port, write_data_array);

                //Task::perform(write_read_port, Message::TaskRequestingAsciiSwitchCodes)
            }
            Message::TaskRequestingAsciiSwitchCodes(_r) => Task::none(),
            Message::TaskReadPort(_r) => Task::none(),
            Message::TaskWritePort(_r) => Task::none(),

            Message::PrintBuffer => {
                let buffer = get_buffer();

                Task::perform(buffer, Message::TaskPrintBuffer)
            }
            Message::TaskPrintBuffer(_r) => Task::none(),
        }
    }

    /** Отображает текущий интерфейс приложения.
     *
     * Эта функция создает и возвращает элемент интерфейса на основе текущего состояния приложения.
     */
    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = container(
            column![
                // Profiles
                create_button_with_svg_and_text(
                    "Профили",
                    include_bytes!("../icons/profiles.svg"),
                    Message::ChangeScreen(Screens::Profile)
                ),
                create_button_with_svg_and_text(
                    "Настройки",
                    include_bytes!("../icons/settings.svg"),
                    Message::ChangeScreen(Screens::Settings)
                ),
                // Updater
                create_button_with_svg_and_text(
                    "Обновление",
                    include_bytes!("../icons/updater.svg"),
                    Message::ChangeScreen(Screens::Updater)
                ),
                // DebugTest
                create_button_with_svg_and_text(
                    "Экспериментальные настройки",
                    include_bytes!("../icons/test.svg"),
                    Message::ChangeScreen(Screens::ExperimentalTab)
                ),
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        )
        .align_y(Alignment::Center)
        .width(100)
        .height(Length::Fill);

        let screen = self::Screens::get_screen_content(self);
        let container_content = match self.keypad.is_open {
            true => row![sidebar, screen],
            false => {
                #[cfg(debug_assertions)]
                {
                    row![sidebar, screen]
                }

                #[cfg(not(debug_assertions))]
                {
                    row![screen]
                }
            }
        }
        .spacing(20);

        container(container_content).into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.keypad.is_open {
            true => iced::time::every(Duration::from_millis(10)).map(|_| Message::ReadPort),
            false => Subscription::none(),
        }
    }
}

/** Создает кнопку с изображением и текстом.
 *
 * Эта функция возвращает кнопку с заданным текстом и изображением, которая вызывает указанное действие при нажатии.
 */
fn create_button_with_svg_and_text<'a>(
    button_text: &'a str,
    svg_path: &'static [u8],
    on_press: Message,
) -> Tooltip<'a, Message> {
    let button = Button::new(
        column![svg(svg::Handle::from_memory(svg_path))
            .height(Fill)
            .width(Fill),]
        .spacing(10)
        .align_x(Alignment::Center),
    )
    .on_press(on_press)
    .width(Length::Fixed(50.))
    .height(Length::Fixed(50.));

    tooltip(button, button_text, tooltip::Position::Right)
}
