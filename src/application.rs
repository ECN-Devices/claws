use iced::{
    widget::{column, container, row, svg, tooltip, Button, Tooltip},
    Alignment, Element,
    Length::{self, Fill},
    Task, Theme,
};
use tokio::runtime::Builder;

use crate::{
    configuration::{
        check_config_file, create_config_dir, create_config_file, get_config_dir, get_config_file,
        update_config_file,
    },
    screens::{self, Screen},
};

// Определение структуры приложения
#[derive(Default, Clone)]
pub struct Lapa {
    pub screen: Screen,
}

// Определение возможных действий в приложении
#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screen),
    ButtonClicked,
    UpdateConfigFile,
}

impl Lapa {
    pub fn new() -> (Self, Task<Message>) {
        let initial_screen = Screen::Profile; // Установка стартового экрана
        (
            Self {
                screen: initial_screen,
            },
            Task::none(),
        )
    }

    // Определение названия приложения
    pub fn title(&self) -> String {
        String::from("Lapa")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Обработка сообщения Message::ChangeScreen
            Message::ChangeScreen(new_screen) => {
                // Определение текущего экран
                self.screen = new_screen.clone();

                match new_screen {
                    Screen::Profile => println!("change screen to Profile"),
                    Screen::Settings => println!("change screen to Settings"),
                    Screen::Updater => println!("change screen to Updater"),
                }
            }
            // Обработка сообщения Message::ButtonClicked
            Message::ButtonClicked => println!("Кнопка нажата"),
            // Обработка сообщения Message::UpdateConfigFile
            Message::UpdateConfigFile => {
                // Создание runtime для асинхронных операций
                let runtime = Builder::new_current_thread().enable_all().build().unwrap();

                // Асинхронная операция обновления конфигурационного файла
                let config_file = runtime.block_on(async {
                    // Проверка наличия конфигурационного файла
                    check_config_file().await;
                    // Обновление конфигурационного файла
                    update_config_file(get_config_file().await).await
                });

                // Вывод обновленного конфигурационного файла
                println!("{:#?}", config_file)
            }
        }
        Task::none()
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
                )
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
        let theme = match dark_light::detect() {
            dark_light::Mode::Dark => Theme::Nord,
            dark_light::Mode::Light => Theme::Nord,
            dark_light::Mode::Default => Theme::Nord,
        };

        theme
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
