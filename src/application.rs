use iced::{
    widget::{column, container, row, svg, tooltip, Button, Tooltip},
    Alignment, Element,
    Length::{self, Fill},
    Task, Theme,
};

// Импортируем шрифт UI_FONT_NORMAL из модуля fonts
use crate::{
    configuration::{create_config_dir, create_config_file, get_config_dir, get_config_file},
    screens::{self, Screen},
};

// Определяем структуру приложения
#[derive(Default, Clone)]
pub struct Lapa {
    pub screen: Screen,
}

// Определяем возможные действий в приложении
#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screen),
    ButtonClicked,
    ButtonConfigDirPrint,
    ButtonConfigDirCreate,
    ButtonConfigFilePrint,
    ButtonConfigFileCreate,
}

impl Lapa {
    pub fn new() -> (Self, Task<Message>) {
        let initial_screen = Screen::Profile;

        (
            Self {
                screen: initial_screen,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        String::from("Lapa")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeScreen(new_screen) => {
                self.screen = new_screen.clone();

                match new_screen {
                    Screen::Profile => println!("change screen to Profile"),
                    Screen::Settings => println!("change screen to Settings"),
                    Screen::Updater => println!("change screen to Updater"),
                }
            }
            Message::ButtonClicked => println!("Кнопка нажата"),
            Message::ButtonConfigDirPrint => {
                let config_dir = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(get_config_dir());

                println!("{:#?}", config_dir)
            }
            Message::ButtonConfigDirCreate => {
                let config_dir = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(create_config_dir());

                println!("{:#?}", config_dir)
            }
            Message::ButtonConfigFilePrint => {
                let config_file = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(get_config_file());

                println!("{:#?}", config_file)
            }
            Message::ButtonConfigFileCreate => {
                let config_file = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(create_config_file());

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
