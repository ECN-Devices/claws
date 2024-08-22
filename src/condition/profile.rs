use std::path::PathBuf;

use iced::{widget::text_editor, Task};

use crate::application::Message;

#[derive(Default)]
pub struct Profile {
    content: text_editor::Content,
    file: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum ProfileMessage {
    ActionPerformed(text_editor::Action),
}

impl Profile {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                content: text_editor::Content::with_text(include_str!("../main.rs")),
                file: None,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: ProfileMessage) {
        match message {
            ProfileMessage::ActionPerformed(action) => self.content.perform(action),
        }
    }
}
