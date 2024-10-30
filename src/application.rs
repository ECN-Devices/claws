use std::{sync::Arc, time::Duration};

use iced::{
    widget::{column, container, row, svg, tooltip, Button, Tooltip},
    Alignment, Element,
    Length::{self, Fill},
    Task, Theme,
};
use log::debug;

use tokio::{runtime::Builder, sync::Mutex};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

use crate::{
    configuration::{
        config::{check_config_file, get_config_file, update_config_file},
        port::{get_keypad_port, read_keypad_port, write_keypad_port},
        ARRAY_LEN,
    },
    screens::{self, Screen},
};

// Определение структуры приложения
#[derive(Debug, Clone)]
pub struct Claws {
    pub screen: Screen,
    pub keypad: Keypad,
}

#[derive(Debug)]
pub struct Keypad {
    pub serial_port: Arc<Mutex<SerialStream>>,
}

impl Clone for Keypad {
    fn clone(&self) -> Self {
        Keypad {
            serial_port: self.serial_port.clone(),
        }
    }
}

// Определение возможных действий в приложении
#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screen),
    ButtonClicked,
    UpdateConfigFile,
    ReadPort,
    WritePort,
    WriteAndReadPort,
    PrintAny,

    TaskReadPort(Result<String, tokio_serial::Error>),
    TaskWritePort(Result<(), tokio_serial::Error>),
}

impl Claws {
    pub fn new() -> (Self, Task<Message>) {
        let initial_screen = Screen::default(); // Установка стартового экрана
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        let port_name = runtime.block_on(get_keypad_port());
        let prln_port_name = port_name.clone();
        //let prln_port_name = "COM3".to_string();

        debug!(
            "Port name: {:?}, bytes: {:?}",
            prln_port_name,
            prln_port_name.as_bytes()
        );
        let serial_port: Arc<Mutex<SerialStream>> = Arc::new(Mutex::new(
            tokio_serial::new(prln_port_name, 9600)
                .timeout(Duration::from_millis(10))
                .open_native_async()
                .expect("Failed to open port"),
        ));
        //debug!(
        //    "Port name: {:?}, bytes: {:?}",
        //    prln_port_name,
        //    prln_port_name.as_bytes()
        //);

        let keypad = Keypad { serial_port };

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
                let _ = runtime.block_on(async {
                    // Проверка наличия конфигурационного файла
                    check_config_file().await;
                    // Обновление конфигурационного файла
                    update_config_file(get_config_file().await).await
                });

                // Вывод обновленного конфигурационного файла
                //println!("{:#?}", config_file);
                Task::none()
            }
            Message::WritePort => {
                let serial_port = self.keypad.serial_port.clone();
                let write_data_array: [u16; ARRAY_LEN] = [11, 0, 0, 0, 0, 0, 0, 0, 0];
                let write_port = write_keypad_port(serial_port, write_data_array);

                Task::perform(write_port, Message::TaskWritePort)
                // Task::none()
            }
            Message::ReadPort => {
                let serial_port = self.keypad.serial_port.clone();
                let read_port = read_keypad_port(serial_port);

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
            Message::TaskReadPort(_) => Task::none(),
            Message::TaskWritePort(_) => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = container(
            column![
                // Profiles
                create_button_with_svg_and_text(
                    "Профили",
                    include_bytes!("../icons/profiles.svg"),
                    Message::ChangeScreen(Screen::Profile)
                ),
                create_button_with_svg_and_text(
                    "Настройки",
                    include_bytes!("../icons/settings.svg"),
                    Message::ChangeScreen(Screen::Settings)
                ),
                // Updater
                create_button_with_svg_and_text(
                    "Обновление",
                    include_bytes!("../icons/updater.svg"),
                    Message::ChangeScreen(Screen::Updater)
                ),
                // DebugTest
                create_button_with_svg_and_text(
                    "Тест нововведений",
                    include_bytes!("../icons/test.svg"),
                    Message::ChangeScreen(Screen::DebugTest)
                ),
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        )
        .align_y(Alignment::Center)
        .width(100)
        .height(Length::Fill);

        let screen = screens::get_screen_content(self);

        container(row![sidebar, screen].spacing(20)).into()
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
