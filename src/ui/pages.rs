use super::Message;
use crate::{State, data::profiles::Profile};
use iced::{
  Alignment, Element, Length,
  widget::{
    Button, button, center, column, container, row, slider, text, toggler, vertical_rule,
    vertical_space,
  },
};

pub const SPACING: u16 = 10;
pub const PADDING: u16 = 10;
const HEADING_SIZE: u16 = 30;
const BUTTON_HEIGH: u16 = 120;
const BUTTON_WIDTH: u16 = 100;

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
  pub fn get_content(state: &State, profile: Profile) -> Element<'_, Message> {
    let screen_name = text(state.pages.name())
      .size(HEADING_SIZE)
      .width(match state.pages {
        Pages::Profiles => Length::Shrink,
        _ => Length::Fill,
      });

    match state.pages {
      Self::Profiles => {
        let toggler = toggler(state.is_rom)
          .label("ОЗУ/ПЗУ")
          .on_toggle(|_| Message::WriteButtonIsRom);

        let ram_rom_button = match state.is_rom {
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

        let col_1 = column![
          mk_button(1, &profile, false),
          mk_button(2, &profile, false),
          mk_button(3, &profile, false),
          mk_button(4, &profile, false),
        ]
        .spacing(SPACING);

        let col_2 = column![
          mk_button(5, &profile, false),
          mk_button(6, &profile, false),
          mk_button(7, &profile, false),
          mk_button(8, &profile, false),
        ]
        .spacing(SPACING);

        let col_3 = column![
          mk_button(9, &profile, false),
          mk_button(10, &profile, false),
          mk_button(11, &profile, false),
          mk_button(12, &profile, false),
        ]
        .spacing(SPACING);

        let col_4 = column![
          mk_button(13, &profile, false),
          mk_button(14, &profile, false),
          mk_button(15, &profile, false),
          mk_button(16, &profile, false),
        ]
        .spacing(SPACING);

        let stick_pad = column![
          column![
            text!("{}%", state.profile.stick.deadzone).size(25),
            slider(1..=100, state.profile.stick.deadzone, |deadzone| {
              Message::WriteDeadZone(deadzone)
            })
            .step(1),
          ]
          .spacing(SPACING)
          .align_x(Alignment::Center),
          row![
            mk_button(4, &profile, true),
            column![
              mk_button(1, &profile, true),
              button("").height(BUTTON_HEIGH).width(BUTTON_WIDTH),
              mk_button(3, &profile, true),
            ]
            .spacing(SPACING),
            mk_button(2, &profile, true),
          ]
          .spacing(SPACING)
          .align_y(Alignment::Center)
        ]
        .spacing(SPACING)
        .width(317.);

        let all_profiles = column![screen_name, toggler, ram_rom_button]
          .padding(PADDING)
          .spacing(SPACING)
          .align_x(Alignment::Center);

        let active_profile = column![
          container(text(profile.name).size(30)).center_x(Length::Fill),
          container(
            row![
              row![col_1, col_2, col_3, col_4].spacing(SPACING),
              column![stick_pad]
            ]
            .spacing(SPACING)
            .align_y(Alignment::End)
          )
          .center(Length::Fill)
        ]
        .padding(PADDING);

        let open_file_dialog = button("Импорт Профиля").on_press(Message::OpenFileDialog);

        let write_button_combination = match state.allow_input {
          true => button("Закончить запись")
            .width(Length::Fixed(300.))
            .on_press(Message::AllowWriteButtonCombination),
          false => button("Начать запись")
            .width(Length::Fixed(300.))
            .on_press(Message::AllowWriteButtonCombination),
        };

        let profile_settings = column![
          text!("Кнопка: #{}", state.button.id),
          container(text(&state.button.label).size(25))
            .align_x(Alignment::Center)
            .width(Length::Shrink),
          write_button_combination,
          button("Очистить").on_press(Message::ClearButtonCombination),
          button("Сохранить").on_press(Message::SaveButtonCombination(state.button.id)),
          vertical_space(),
          open_file_dialog,
        ]
        .width(Length::Fixed(300.))
        .align_x(Alignment::Center)
        .spacing(SPACING)
        .padding(PADDING);

        row!(
          all_profiles,
          vertical_rule(2),
          active_profile,
          vertical_rule(2),
          profile_settings
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

        let write_profile = button("Write Profile").on_press(Message::ProfileWrite);
        let update_profile = button("Update Profile").on_press(Message::ProfileReceive);

        let save_active_profile_to_file =
          button("Save Active Profile to File").on_press(Message::ProfileFileSave);

        column!(
          screen_name,
          center(
            column![
              reboot_to_bootloader,
              empty,
              write_profile,
              update_profile,
              save_active_profile_to_file
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
fn mk_button<'a>(id: usize, profile: &Profile, stick: bool) -> Button<'a, Message> {
  let _id = id - 1;
  let _text = match stick {
    true => profile.get_stick_label(_id),
    false => profile.get_button_label(_id),
  };

  button(
    column![
      container(text(_text.clone()).size(20)).center(Length::Fill),
      text!("#{}", id)
        .size(10)
        .align_x(Alignment::End)
        .align_y(Alignment::End),
    ]
    .width(Length::Fill)
    .height(Length::Fill),
  )
  .on_press(Message::GetButtonSettings(id, _text, stick))
  .height(BUTTON_HEIGH)
  .width(BUTTON_WIDTH)
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
