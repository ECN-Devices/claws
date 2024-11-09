use std::{sync::Arc, time::Duration};

use iced::{
    widget::{column, container, row, svg, tooltip, Button, Tooltip},
    Alignment, Element,
    Length::{self, Fill},
    Task, Theme,
};
use log::{debug, error};

use tokio::{runtime::Builder, sync::Mutex};

use crate::{
    configuration::{
        config::{check_config_file, get_config_file, update_config_file},
        port::Keypad,
        ARRAY_LEN,
    },
    screens::Screens,
};

// Определение структуры приложения
#[derive(Debug, Clone)]
pub struct Claws {
    pub screen: Screens,
    pub keypad: Keypad,
}

// Определение возможных действий в приложении
#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screens),
    ButtonClicked,
    UpdateConfigFile,
    ReadPort,
    WritePort,
    WriteAndReadPort,
    PrintAny,

    TaskReadPort(Result<String, serialport::Error>),
    TaskWritePort(Result<(), serialport::Error>),
}

impl Claws {
    pub fn new() -> (Self, Task<Message>) {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        let port_name = runtime.block_on(self::Keypad::get_keypad_port()).clone();

        let initial_screen = if port_name.is_empty() {
            Screens::ConnectedDeviceNotFound
        } else {
            Screens::default()
        };

        debug!(
            "Port name: {:?}, bytes: {:?}",
            port_name,
            port_name.as_bytes()
        );

        let keypad = if !port_name.is_empty() {
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
        } else {
            Keypad {
                port: None,
                is_open: false,
            }
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
        match message {
            // Обработка сообщения Message::ChangeScreen
            Message::ChangeScreen(new_screen) => {
                self.screen = new_screen.clone();
                // Определение текущего экран
                Task::none()
            }
            // Обработка сообщения Message::ButtonClicked
            Message::ButtonClicked => {
                println!("Кнопка нажата");
                Task::none()
            }
            // Обработка сообщения Message::UpdateConfigFile
            Message::UpdateConfigFile => {
                // Создание runtime для асинхронных операций
                let runtime = Builder::new_current_thread().enable_all().build().unwrap();

                // Асинхронная операция обновления конфигурационного файла
                let config_file = runtime.block_on(async {
                    // Проверка наличия конфигурационного файла
                    check_config_file().await?;
                    // Обновление конфигурационного файла
                    update_config_file(get_config_file().await).await
                });

                // Вывод обновленного конфигурационного файла
                debug!("{:?}", config_file);
                Task::none()
            }
            Message::WritePort => {
                let serial_port = match self.keypad.is_open {
                    true => self.keypad.port.clone().unwrap(),
                    false => todo!(),
                };
                let write_data_array: [u16; ARRAY_LEN] = [11, 0, 0, 0, 0, 0, 0, 0, 0];
                let write_port = self::Keypad::write_keypad_port(serial_port, write_data_array);

                Task::perform(write_port, Message::TaskWritePort)
                // Task::none()
            }
            Message::ReadPort => {
                let serial_port = match self.keypad.is_open {
                    true => self.keypad.port.clone().unwrap(),
                    false => todo!(),
                };
                let read_port = self::Keypad::read_keypad_port(serial_port);

                Task::perform(read_port, Message::TaskReadPort)
                // Task::none()
            }
            Message::WriteAndReadPort => {
                // let runtime = Builder::new_current_thread().enable_all().build().unwrap();
                // let write_data_array: [u16; ARRAY_LEN] = [11, 0, 0, 0, 0, 0, 0, 0, 0];
                // let write_port =
                //     write_keypad_port(self.keypad.serial_port.try_clone(), write_data_array);

                // let read_port = read_keypad_port(self.keypad.serial_port.try_clone());

                // // let (write_port_result, read_port_result) = tokio::join!(write_port, read_port);

                // // println!("{}", write)

                // let write_and_read_port = async {
                //     let _ =
                //         write_keypad_port(self.keypad.serial_port.try_clone(), write_data_array)
                //             .await;
                //     read_keypad_port(self.keypad.serial_port.try_clone()).await
                // };

                // runtime.block_on(async move { write_and_read_port.await });

                // Task::batch(vec![write_port, read_port]).map(Message::TaskWriteAndReadPort)
                Task::none()
            }
            Message::PrintAny => Task::none(),
            Message::TaskReadPort(_r) => Task::none(),
            Message::TaskWritePort(_r) => Task::none(),
        }
    }

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

        match self.keypad.is_open {
            true => {
                let screen = self::Screens::get_screen_content(self);
                container(row![sidebar, screen].spacing(20)).into()
            }
            false => {
                let screen = self::Screens::get_screen_content(self);
                container(row![screen]).into()
            }
        }
    }

    pub fn theme(&self) -> Theme {
        match dark_light::detect() {
            dark_light::Mode::Dark => Theme::Dark,
            dark_light::Mode::Light => Theme::Light,
            dark_light::Mode::Default => Theme::Dark,
        }
    }
}

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
