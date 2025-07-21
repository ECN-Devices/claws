use super::Message;
use crate::{
  App,
  data::profiles::Profile,
  hardware::commands::{KeypadCommands, empty, profile, stick},
};
use iced::{
  Element, Length,
  alignment::{Horizontal, Vertical},
  widget::{Button, button, center, column, container, row, text, vertical_rule},
};

pub const SPACING: u16 = 10;
pub const PADDING: u16 = 10;
const HEADING_SIZE: u16 = 30;

/// Перечисление экранов приложения
#[derive(Clone, Debug, Default)]
pub enum Pages {
  /// Экран управления профилями (экран по умолчанию)
  #[default]
  Profiles,

  /// Экран настроек
  Settings,

  /// Экран обновления прошивки
  Updater,

  /// Экран отображения ошибки подключения устройства
  ConnectedDeviceNotFound,

  /// Экран экспериментальных функций
  Experimental,
}

impl Pages {
  /// Возвращает имя текущего экрана
  fn name(&self) -> &str {
    match self {
      Self::Profiles => "Профили",
      Self::Settings => "Настройки",
      Self::Updater => "Обновление",
      Self::ConnectedDeviceNotFound => "Устройство не найдено",
      Self::Experimental => "Экспериментальные настройки",
    }
  }

  /**
  Генерирует содержимое экрана на основе текущего состояния
  # Аргументы
  * `claws` - Ссылка на главное приложение с текущим состоянием
  # Возвращает
  Элемент интерфейса для отображения
  */
  pub fn get_content(claws: &App, profile: Profile) -> Element<Message> {
    let screen_name = text(claws.pages.name())
      .size(HEADING_SIZE)
      .width(match claws.pages {
        Pages::Profiles => Length::Shrink,
        _ => Length::Fill,
      });

    match claws.pages {
      Self::Profiles => {
        let col_1 = column![
          mk_button(1, &profile, Message::ButtonClicked),
          mk_button(2, &profile, Message::ButtonClicked),
          mk_button(3, &profile, Message::ButtonClicked),
          mk_button(4, &profile, Message::ButtonClicked),
        ]
        .spacing(SPACING);

        let col_2 = column![
          mk_button(5, &profile, Message::ButtonClicked),
          mk_button(6, &profile, Message::ButtonClicked),
          mk_button(7, &profile, Message::ButtonClicked),
          mk_button(8, &profile, Message::ButtonClicked),
        ]
        .spacing(SPACING);

        let col_3 = column![
          mk_button(9, &profile, Message::ButtonClicked),
          mk_button(10, &profile, Message::ButtonClicked),
          mk_button(11, &profile, Message::ButtonClicked),
          mk_button(12, &profile, Message::ButtonClicked),
        ]
        .spacing(SPACING);

        let col_4 = column![
          mk_button(13, &profile, Message::ButtonClicked),
          mk_button(14, &profile, Message::ButtonClicked),
          mk_button(15, &profile, Message::ButtonClicked),
          mk_button(16, &profile, Message::ButtonClicked),
        ]
        .spacing(SPACING);

        let buttons_container = container(row![col_1, col_2, col_3, col_4].spacing(SPACING))
          .center_y(Length::Fill)
          .center_x(Length::Fill)
          .padding(PADDING);

        let all_profiles = column![screen_name].padding(PADDING);
        let active_profile = column![
          container(text(profile.name).size(30))
            .center_x(Length::Fill)
            .padding(PADDING),
          buttons_container
        ];
        let open_file_dialog = button("file").on_press(Message::OpenFileDialog);

        row!(
          all_profiles,
          vertical_rule(2),
          active_profile,
          vertical_rule(2),
          open_file_dialog
        )
        .into()
      }
      Self::Settings => container(screen_name).padding(PADDING).into(),
      Self::Updater => container(screen_name).padding(PADDING).into(),
      Self::ConnectedDeviceNotFound => center(screen_name.center()).padding(PADDING).into(),
      Self::Experimental => {
        let reboot_to_bootloader =
          button("Reboot to Bootloader").on_press(Message::RebootToBootloader);

        let empty = button("Empty").on_press(Message::WritePort(KeypadCommands::Empty(
          empty::Command::VoidRequest,
        )));

        let stick_cal =
          button("Stick Calibration").on_press(Message::WritePort(KeypadCommands::Stick(
            stick::Command::Calibration(stick::OptionsCalibration::Calibrate),
          )));

        let stick_request =
          button("Stick Request").on_press(Message::WritePort(KeypadCommands::Stick(
            stick::Command::Calibration(stick::OptionsCalibration::Request),
          )));

        let write_profile = button("Write Profile").on_press(Message::WriteProfile);
        let save_profile_flash = button("Save Profile to Flash").on_press(Message::WritePort(
          KeypadCommands::Profile(profile::Command::WriteActiveToFlash(1)),
        ));

        let save_profile_file = button("Save Profile to File").on_press(Message::SaveProfile);

        column!(
          screen_name,
          center(
            column![
              reboot_to_bootloader,
              empty,
              stick_cal,
              stick_request,
              write_profile,
              save_profile_flash,
              save_profile_file,
            ]
            .spacing(SPACING)
          )
        )
        .into()
      }
    }
  }
}

/**
Создает стандартизированную кнопку для клавиатурной панели
# Аргументы
* `button_text` - Текст на кнопке
* `on_press` - Сообщение при нажатии
# Возвращает
Готовый элемент кнопки с заданными параметрами
*/
fn mk_button(id: u8, profile: &Profile, on_press: Message) -> Button<'static, Message> {
  let button_id = (id - 1) as usize;
  let button_text = profile.get_button_label(button_id);

  button(text(button_text).size(20))
    .on_press(on_press)
    .height(110)
    .width(80)
}
/// Иконки для навигационного меню
pub enum Icon {
  /// Иконка профилей
  Profiles,

  /// Иконка настроек
  Settings,

  /// Иконка обновления
  Update,

  /// Иконка экспериментальных функций
  Experimental,
}
impl Icon {
  /// Возвращает SVG-иконку в виде байтового массива
  pub fn icon(&self) -> &'static [u8] {
    match self {
      Icon::Profiles => include_bytes!("../../assets/icons/profiles.svg"),
      Icon::Settings => include_bytes!("../../assets/icons/settings.svg"),
      Icon::Update => include_bytes!("../../assets/icons/updater.svg"),
      Icon::Experimental => include_bytes!("../../assets/icons/test.svg"),
    }
  }
}
