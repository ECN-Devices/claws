use std::path::PathBuf;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        column, container, horizontal_space, row, text, text_editor, Button, Column, Container,
        Scrollable, Text,
    },
    Element,
    Length::Fill,
    Task, Theme,
};

// Импортируем шрифт UI_FONT_NORMAL из модуля fonts
use crate::fonts::UI_FONT_NORMAL;

// Определяем структуру приложения
//      content: Поле текстового редактора
//      file: файл
#[derive(Default)]
pub struct Lapa {
    content: text_editor::Content,
}

// Определяем возможные действий в приложении
// ActionPerformed: действия text_editor
#[derive(Debug, Clone)]
pub enum Message {
    ActionPerformed(text_editor::Action),
}

impl Lapa {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                content: text_editor::Content::with_text(include_str!("main.rs")),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        String::from("A cool text editor")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ActionPerformed(action) => self.content.perform(action),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let editor = text_editor(&self.content)
            .on_action(Message::ActionPerformed)
            .height(Fill)
            .font(UI_FONT_NORMAL.clone());

        let cursor_position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{}:{}", line + 1, column + 1))
        };

        let status_bar = row![horizontal_space(), cursor_position];

        container(column![editor, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    pub fn theme(&self) -> Theme {
        let theme = match dark_light::detect() {
            dark_light::Mode::Dark => Theme::Dark,
            dark_light::Mode::Light => Theme::Light,
            dark_light::Mode::Default => Theme::Light,
        };

        theme
    }
}
