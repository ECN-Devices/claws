use std::path::PathBuf;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{text, text_editor, Column, Container, Text, TextEditor},
    Alignment, Element,
};

#[derive(Default)]
pub struct SettingsTab {
    pub content: text_editor::Content,
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
