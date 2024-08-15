use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text},
    Element, Length,
};
use iced_aw::sidebar::TabLabel;
use profile::{ProfileMessage, ProfileTab};
use settings::{SettingsMessage, SettingsTab};

mod profile;
mod settings;

pub const HEADER_SIZE: u16 = 32;
pub const TAB_PADDING: u16 = 16;

#[derive(Default)]
pub struct TabBarExample {
    pub active_tab: TabId,
    pub profile_tab: ProfileTab,
    pub settings_tab: SettingsTab,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(TabId),
    Profile(ProfileMessage),
    Settings(SettingsMessage),
}

#[derive(PartialEq, Hash, Debug, Clone, Eq, Default)]
pub enum TabId {
    #[default]
    Profile,
    Settings,
}

pub trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content())
            .align_x(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
