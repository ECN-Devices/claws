// #![windows_subsystem = "windows"]

use iced::{
    executor,
    widget::{column, container, text, text_editor},
    Application, Command, Element,
    Length::Fill,
    Settings, Theme,
};

mod fonts;

fn main() -> iced::Result {
    pretty_env_logger::init();

    fonts::set();

    Editor::run(Settings {
        default_font: fonts::JETBRAINS_MONO_MEDIUM.clone().into(),
        fonts: fonts::load(),
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    ActionPerformed(text_editor::Action),
}

impl Application for Editor {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: text_editor::Content::with_text(include_str!("main.rs")),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool text editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ActionPerformed(action) => self.content.perform(action),
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let editor = text_editor(&self.content)
            .on_action(Message::ActionPerformed)
            .height(Fill);

        let cursor_position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{}:{}", line + 1, column + 1))
        };

        container(column![editor, cursor_position])
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
