use std::path::PathBuf;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{text, text_editor, Column, Container, Text, TextEditor},
    Alignment, Element,
};
use iced_aw::sidebar::TabLabel;

use super::{Message, Tab};

#[derive(Default)]
pub struct SettingsTab {
    content: text_editor::Content,
    file: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ActionPerformed(text_editor::Action),
}

impl SettingsTab {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
            file: None,
        }
    }

    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::ActionPerformed(action) => self.content.perform(action),
        }
    }

    fn cursor_position(&self) -> Text {
        let cursor_position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{}:{}", line + 1, column + 1))
        };

        cursor_position
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Profile")
    }

    fn tab_label(&self) -> iced_aw::sidebar::TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> iced::Element<'_, Self::Message> {
        let content: Element<'_, SettingsMessage> = Container::new(
            Column::new()
                .align_x(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    TextEditor::new(&self.content)
                        .on_action(SettingsMessage::ActionPerformed)
                        .padding(10)
                        .size(32),
                ),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();

        content.map(Message::Settings)
    }
}
