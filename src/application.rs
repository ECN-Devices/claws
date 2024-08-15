use std::path::PathBuf;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        column, container, horizontal_space, row, text, text_editor, Column, Container, Text,
    },
    Element,
    Length::{self, Fill},
    Task, Theme,
};
use iced_aw::{sidebar::TabLabel, widgets::SidebarWithContent};

// Импортируем шрифт UI_FONT_NORMAL из модуля fonts
use crate::{
    fonts::UI_FONT_NORMAL,
    tabs::{Message, Tab, TabBarExample, TabId, HEADER_SIZE, TAB_PADDING},
};

// Определяем структуру приложения
//      content: Поле текстового редактора
//      file: файл
// #[derive(Default)]
// pub struct Editor {
//     content: text_editor::Content,
//     file: Option<PathBuf>,
// }

// Определяем возможные действий в приложении
//      ActionPerformed: действия text_editor
// #[derive(Debug, Clone)]
// pub enum Message {
//     ActionPerformed(text_editor::Action),
// }

impl TabBarExample {
    // pub fn new() -> (Self, Task<Message>) {
    //     (
    //         Self {
    //             content: text_editor::Content::with_text(include_str!("main.rs")),
    //             file: None,
    //         },
    //         Task::none(),
    //     )
    // }

    pub fn title(&self) -> String {
        String::from("A cool text editor")
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Profile(message) => self.profile_tab.update(message),
            Message::Settings(message) => self.settings_tab.update(message),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // let editor = text_editor(&self.content)
        //     .on_action(Message::ActionPerformed)
        //     .height(Fill)
        //     .font(UI_FONT_NORMAL.clone());

        // let cursor_position = {
        //     let (line, column) = self.content.cursor_position();
        //     text(format!("{}:{}", line + 1, column + 1))
        // };

        // let file_name = text(if let Some(path) = &self.file {
        //     let path = path.display().to_string();

        //     if path.len() > 60 {
        //         format!("...{}", &path[path.len() - 40..])
        //     } else {
        //         path
        //     }
        // } else {
        //     String::from("New file")
        // });

        // let status_bar = row![file_name, horizontal_space(), cursor_position];

        // container(column![editor, status_bar].spacing(10))
        //     .padding(10)
        //     .into()

        SidebarWithContent::new(Message::TabSelected)
            .tab_icon_position(iced_aw::sidebar::Position::End)
            .push(
                TabId::Profile,
                self.profile_tab.tab_label(),
                self.profile_tab.view(),
            )
            .push(
                TabId::Settings,
                self.settings_tab.tab_label(),
                self.settings_tab.view(),
            )
            .set_active_tab(&self.active_tab)
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
