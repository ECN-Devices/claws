use std::path::PathBuf;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{text, text_editor, Column, Container, Text, TextEditor},
    Alignment, Element,
};
use iced_aw::sidebar::TabLabel;

use super::{Message, Tab};

#[derive(Default)]
pub struct ProfileTab {
    content: text_editor::Content,
    file: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum ProfileMessage {
    ActionPerformed(text_editor::Action),
}

impl ProfileTab {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::with_text(include_str!("../main.rs")),
            file: None,
        }
    }

    pub fn update(&mut self, message: ProfileMessage) {
        match message {
            ProfileMessage::ActionPerformed(action) => self.content.perform(action),
        }
    }

    // fn cursor_position(&self) -> Text {
    //     let cursor_position = {
    //         let (line, column) = self.content.cursor_position();
    //         text(format!("{}:{}", line + 1, column + 1))
    //     };

    //     cursor_position
    // }
}

impl Tab for ProfileTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Profile")
    }

    fn tab_label(&self) -> iced_aw::sidebar::TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> iced::Element<'_, Self::Message> {
        let content: Element<'_, ProfileMessage> = Container::new(
            Column::new()
                .align_x(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    TextEditor::new(&self.content)
                        .on_action(ProfileMessage::ActionPerformed)
                        .padding(10)
                        .size(32),
                ),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();

        content.map(Message::Profile)
    }
}
