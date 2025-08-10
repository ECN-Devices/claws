use super::Message;
use crate::{App, data::profiles::Profile};
use iced::{
  Alignment, Element, Length,
  widget::{Button, button, center, column, container, row, text, toggler, vertical_rule},
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
  pub fn get_content(claws: &App, profile: Profile) -> Element<'_, Message> {
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

        let stick_pad = row![
          mk_stick(4, &profile, Message::ButtonClicked),
          column![
            mk_stick(1, &profile, Message::ButtonClicked),
            button("").height(110).width(90),
            mk_stick(3, &profile, Message::ButtonClicked),
          ]
          .spacing(SPACING),
          mk_stick(2, &profile, Message::ButtonClicked),
        ]
        .spacing(SPACING)
        .align_y(Alignment::Center);

        let buttons_container = container(row![col_1, col_2, col_3, col_4].spacing(SPACING))
          .center_y(Length::Fill)
          .center_x(Length::Fill);

        let stick_container = container(column![stick_pad])
          .center_y(Length::Fill)
          .center_x(Length::Fill);

        let toggler = toggler(claws.is_rom)
          .label("ОЗУ/ПЗУ")
          .on_toggle(|_| Message::WriteButtonIsRom);

        let write_button = match claws.is_rom {
          true => column![
            button("ПЗУ 1")
              .on_press(Message::ProfileActiveWriteToRom(1))
              .width(80),
            button("ПЗУ 2")
              .on_press(Message::ProfileActiveWriteToRom(2))
              .width(80),
            button("ПЗУ 3")
              .on_press(Message::ProfileActiveWriteToRom(3))
              .width(80),
            button("ПЗУ 4")
              .on_press(Message::ProfileActiveWriteToRom(4))
              .width(80)
          ],
          false => column![
            button("ОЗУ 1")
              .on_press(Message::ProfileActiveWriteToRam(1))
              .width(80),
            button("ОЗУ 2")
              .on_press(Message::ProfileActiveWriteToRam(2))
              .width(80),
            button("ОЗУ 3")
              .on_press(Message::ProfileActiveWriteToRam(3))
              .width(80),
            button("ОЗУ 4")
              .on_press(Message::ProfileActiveWriteToRam(4))
              .width(80)
          ],
        }
        .spacing(SPACING);

        let all_profiles = column![screen_name, toggler, write_button]
          .padding(PADDING)
          .spacing(SPACING)
          .align_x(Alignment::Center);

        let active_profile = column![
          container(text(profile.name).size(30))
            .center_x(Length::Fill)
            .padding(PADDING),
          row![buttons_container, stick_container].padding(PADDING)
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

        let empty = button("Empty").on_press(Message::PortAvalaible);

        // let stick_cal = button("Stick Calibration").on_press(Message::PortSend);

        let write_profile = button("Write Profile").on_press(Message::ProfileWrite);

        column!(
          screen_name,
          center(
            column![
              reboot_to_bootloader,
              empty,
              // stick_cal,
              write_profile,
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

  button(text(button_text).size(20).center())
    .on_press(on_press)
    .height(110)
    .width(90)
}

fn mk_stick(id: u8, profile: &Profile, on_press: Message) -> Button<'static, Message> {
  let stick_id = (id - 1) as usize;
  let stick_text = profile.get_stick_label(stick_id);

  button(text(stick_text).size(20).center())
    .on_press(on_press)
    .height(110)
    .width(90)
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
