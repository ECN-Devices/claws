use super::Message;
use crate::App;
use iced::{
  Alignment, Element, Length,
  widget::{Button, button, center, column, container, row, text},
};

const BUTTON_SPACING: u16 = 30;
const BUTTON_PADDING: u16 = 10;
const HEADING_SIZE: u16 = 30;

#[derive(Clone, Debug, Default)]
pub enum Pages {
  #[default]
  Profiles,
  Settings,
  Updater,
  ConnectedDeviceNotFound,
  Experimental,
}

impl Pages {
  /// Возвращает имя текущего экрана в виде строки.
  fn name(&self) -> &str {
    match self {
      Self::Profiles => "Профили",
      Self::Settings => "Настройки",
      Self::Updater => "Обновление",
      Self::ConnectedDeviceNotFound => "Устройство не найдено",
      Self::Experimental => "Экспериментальные настройки",
    }
  }
  /** Генерирует содержимое экрана в зависимости от текущего состояния приложения.
   * # Параметры
   * `claws`: Ссылка на экземпляр `Claws`, который содержит текущее состояние приложения.
   * # Возвращает
   * Возвращает элемент типа `Element`, который представляет содержимое текущего экрана.
   */
  pub fn get_content(claws: &App) -> Element<Message> {
    match claws.pages {
      Self::Profiles => {
        let screen_name = text(claws.pages.name())
          .size(HEADING_SIZE)
          .width(Length::Fill);

        let buttons_1 = column![
          create_keypad_button("#1", Message::ButtonClicked),
          create_keypad_button("#2", Message::ButtonClicked),
          create_keypad_button("#3", Message::ButtonClicked),
          create_keypad_button("#4", Message::ButtonClicked),
        ]
        .spacing(BUTTON_SPACING)
        .padding(BUTTON_PADDING);

        let buttons_2 = column![
          create_keypad_button("#5", Message::ButtonClicked),
          create_keypad_button("#6", Message::ButtonClicked),
          create_keypad_button("#7", Message::ButtonClicked),
          create_keypad_button("#8", Message::ButtonClicked),
        ]
        .spacing(BUTTON_SPACING)
        .padding(BUTTON_PADDING);

        let buttons_3 = column![
          create_keypad_button("#9", Message::ButtonClicked),
          create_keypad_button("#10", Message::ButtonClicked),
          create_keypad_button("#11", Message::ButtonClicked),
          create_keypad_button("#12", Message::ButtonClicked),
        ]
        .spacing(BUTTON_SPACING)
        .padding(BUTTON_PADDING);

        let buttons_4 = column![
          create_keypad_button("#13", Message::ButtonClicked),
          create_keypad_button("#14", Message::ButtonClicked),
          create_keypad_button("#15", Message::ButtonClicked),
          create_keypad_button("#16", Message::ButtonClicked),
        ]
        .spacing(BUTTON_SPACING)
        .padding(BUTTON_PADDING);

        let buttons_container = row![buttons_1, buttons_2, buttons_3, buttons_4];

        column!(screen_name, center(buttons_container))
          .padding(10)
          .into()
      }
      Self::Settings => {
        let screen_name = text(claws.pages.name())
          .size(HEADING_SIZE)
          .width(Length::Fill)
          .height(Length::Shrink);

        container(screen_name).padding(10).into()
      }
      Self::Updater => {
        let screen_name = text(claws.pages.name())
          .size(HEADING_SIZE)
          .width(Length::Fill)
          .height(Length::Fill);

        container(screen_name).padding(10).into()
      }
      Self::ConnectedDeviceNotFound => {
        let screen_name = text(claws.pages.name())
          .size(HEADING_SIZE)
          .width(Length::Fill)
          .height(Length::Fill)
          .center();

        container(screen_name).padding(10).into()
      }
      Self::Experimental => {
        let screen_name = text(claws.pages.name())
          .size(HEADING_SIZE)
          .width(Length::Fill)
          .height(Length::Fixed(40.));

        let reboot_to_bootloader =
          button("Reboot to Bootloader").on_press(Message::RebootToBootloader);
        let empty = button("Empty").on_press(Message::WritePort(
          crate::hardware::communication_protocol::KeypadCommands::Empty(
            crate::hardware::communication_protocol::CommandEmpty::VoidRequest,
          ),
        ));

        column!(screen_name, center(column![reboot_to_bootloader, empty]))
          .padding(10)
          .into()
      }
    }
  }
}

/** Создает кнопку для клавиатуры.
 * # Параметры
 * `button_text`: Текст, который будет отображаться на кнопке.
 * `on_press`: Сообщение, которое будет отправлено при нажатии на кнопку.
 * # Возвращает
 * Возвращает экземпляр `Button`, который можно использовать в пользовательском интерфейсе.
 */
fn create_keypad_button(button_text: &str, on_press: Message) -> Button<Message> {
  button(
    text(button_text)
      .size(10)
      .align_x(Alignment::End)
      .align_y(Alignment::End),
  )
  .on_press(on_press)
  .height(110)
  .width(80)
}

pub enum Icon {
  Profiles,
  Settings,
  Update,
  Experimental,
}

impl Icon {
  pub fn icon(&self) -> &'static [u8] {
    match self {
      Icon::Profiles => include_bytes!("../../assets/icons/profiles.svg"),
      Icon::Settings => include_bytes!("../../assets/icons/settings.svg"),
      Icon::Update => include_bytes!("../../assets/icons/updater.svg"),
      Icon::Experimental => include_bytes!("../../assets/icons/test.svg"),
    }
  }
}
