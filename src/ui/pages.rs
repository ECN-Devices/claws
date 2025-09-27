//! Страницы интерфейса и элементы управления для конфигурации профилей и настроек.

use super::Message;
use crate::{State, assets::APPLICATION_VERSION, data::profiles::Profile, ui::styles};
use iced::{
  Alignment, Element, Length, Theme,
  widget::{
    MouseArea, Row, button, center, column, container, horizontal_space, mouse_area, row, slider,
    svg, text, text_input, toggler, vertical_rule, vertical_space,
  },
};

/// Отступ между элементами
pub const SPACING: u16 = 10;
/// Внутренние отступы контейнеров
pub const PADDING: u16 = 10;
const HEADING_SIZE: u16 = 30;
const BUTTON_HEIGH: u16 = 100;
const BUTTON_WIDTH: u16 = 90;

macro_rules! mk_button {
  ($text:expr, $on_press:expr) => {
    button($text)
      .on_press($on_press)
      .style(styles::button::rounding)
  };
}

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
}

impl Pages {
  /// Возвращает имя текущего экрана
  fn name(&self) -> &str {
    match self {
      Self::Profiles => "Профили",
      Self::Settings => "Настройки",
      Self::Updater => "Обновление",
      Self::ConnectedDeviceNotFound => "Устройство не найдено",
    }
  }

  /**
  Генерирует содержимое экрана на основе текущего состояния
  # Аргументы
  * `state` - Ссылка на состояние приложения
  * `profile` - Активный профиль
  # Возвращает
  Элемент интерфейса для отображения
  */
  pub fn get_content<'a>(state: &'a State, profile: &'a Profile) -> Element<'a, Message> {
    let screen_name = text(state.pages.name())
      .size(HEADING_SIZE)
      .width(match state.pages {
        Pages::Profiles => Length::Shrink,
        _ => Length::Fill,
      });

    match state.pages {
      Self::Profiles => {
        let toggler = toggler(state.is_rom).on_toggle(|_| Message::WriteButtonIsRom);

        let ram_rom_button = column![
          mk_button_profile_row(state, &1),
          mk_button_profile_row(state, &2),
          mk_button_profile_row(state, &3),
          mk_button_profile_row(state, &4),
        ]
        .spacing(SPACING);

        let col_1 = column![
          mk_button(state, 1, profile),
          mk_button(state, 2, profile),
          mk_button(state, 3, profile),
          mk_button(state, 4, profile),
        ]
        .spacing(SPACING);

        let col_2 = column![
          mk_button(state, 5, profile),
          mk_button(state, 6, profile),
          mk_button(state, 7, profile),
          mk_button(state, 8, profile),
        ]
        .spacing(SPACING);

        let col_3 = column![
          mk_button(state, 9, profile),
          mk_button(state, 10, profile),
          mk_button(state, 11, profile),
          mk_button(state, 12, profile),
        ]
        .spacing(SPACING);

        let col_4 = column![
          mk_button(state, 13, profile),
          mk_button(state, 14, profile),
          mk_button(state, 15, profile),
          mk_button(state, 16, profile),
        ]
        .spacing(SPACING);

        let stick_pad = column![
          column![
            text!("Мёртвая зона: {}%", state.profile.stick.deadzone).size(25),
            slider(1..=100, state.profile.stick.deadzone, |deadzone| {
              Message::WriteDeadZone(deadzone)
            })
            .step(1),
          ]
          .spacing(SPACING)
          .align_x(Alignment::Center),
          mk_button_stick(state, 1, profile),
          row![
            mk_button_stick(state, 4, profile),
            button("").height(BUTTON_HEIGH).width(BUTTON_WIDTH).style(
              move |theme: &Theme, status| {
                styles::button::stick::active_write(theme, status, state, 0, state.button.is_stick)
              }
            ),
            mk_button_stick(state, 2, profile),
          ]
          .spacing(SPACING),
          mk_button_stick(state, 3, profile),
        ]
        .align_x(Alignment::Center)
        .spacing(SPACING)
        .width(317.);

        let import_profile = mk_button!(
          container("Импорт профиля").center_x(Length::Fill),
          Message::ProfileImport
        )
        .width(180);
        let export_profile = mk_button!(
          container("Экспорт профиля").center_x(Length::Fill),
          Message::ProfileExport
        )
        .width(180);

        let all_profiles = column![
          screen_name,
          row![text("ОЗУ"), toggler, text("ПЗУ")]
            .align_y(Alignment::Center)
            .spacing(SPACING),
          ram_rom_button,
          vertical_space(),
          import_profile,
          export_profile,
        ]
        .align_x(Alignment::Center)
        .spacing(SPACING)
        .padding(PADDING);

        let active_profile = mouse_area(
          column![
            container(
              text_input(&profile.name, &profile.name)
                .align_x(Alignment::Center)
                .size(30)
                .width(300)
                .on_input(Message::ProfileUpdateName)
                .style(styles::text_input::rounding)
            )
            .center_x(Length::Fill),
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
          .padding(PADDING),
        )
        .on_press(Message::DisallowWriteButtonCombination);

        row!(all_profiles, vertical_rule(2), active_profile,).into()
      }
      Self::Settings => {
        let reboot_to_bootloader = mk_button!(
          container("Перезагрузить в bootloder").center_x(Length::Fill),
          Message::RebootToBootloader
        )
        .width(Length::Fill);

        let stick_calibration = mk_button!(
          container("Калибровать стик").center_x(Length::Fill),
          Message::StickInitCalibration
        )
        .width(Length::Fill);

        let settings_layout = match state.stick_callibrate {
          true => match state.stick_callibrate_time {
            Some(time) => {
              column![
                container(text("Калибровка стика").size(HEADING_SIZE))
                  .style(styles::container::round_bordered_box_header)
                  .width(Length::Fill)
                  .padding(PADDING),
                container(text!("Вращайте стик {}", 6 - time.elapsed().as_secs()).size(20))
                  .align_x(Alignment::Center)
                  .style(styles::container::round_bordered_box)
                  .width(Length::Fill)
                  .padding(PADDING),
              ]
              .width(600)
            }
            None => match state.stick_show_calibrate_parameters {
              true => {
                column![
                  container(text("Параметры калибровки стика").size(HEADING_SIZE))
                    .style(styles::container::round_bordered_box_header)
                    .width(Length::Fill)
                    .padding(PADDING),
                  container(column![
                    text!("Центр по оси X: {}\nЦентр по оси Y: {}\nВнешняя мертвая зона: {}\nВнутренняя мертвая зона: {}",
                     state.stick_info.center_x,
                     state.stick_info.center_y,
                     state.stick_info.external_deadzone,
                     state.stick_info.internal_deadzone
                   ).size(20),
                   container(mk_button!("Готово", Message::StickEndCalibration)).align_right(Length::Fill)
                 ]).style(styles::container::round_bordered_box).padding(PADDING),
                ]
                .width(600)
              }
              false => {
                column![
                  container(text("Калибровка стика").size(HEADING_SIZE)).style(styles::container::round_bordered_box_header).width(Length::Fill).padding(PADDING),
                  container(column![
                    "После нажатия на кнопку 'Далее' начнется процесс калибровки стика, вам необходимо вращать стик в крайнем положении пока не закончиться обратный отсчет.",
                    container(mk_button!("Далее",Message::StickStartCalibration)).align_right(Length::Fill)
                  ]).style(styles::container::round_bordered_box).width(Length::Fill).padding(PADDING),
                ].width(600)
              }
            },
          },
          false => column![reboot_to_bootloader, stick_calibration].width(270)
            .align_x(Alignment::Center)
            .spacing(SPACING),
        };

        column![screen_name, center(settings_layout)]
          .spacing(SPACING)
          .padding(PADDING)
          .into()
      }
      Self::Updater => {
        let app_version = row![
          text("Версия приложения:"),
          horizontal_space(),
          text(APPLICATION_VERSION)
        ];

        let firmware_version = row![
          text("Версия прошивки:"),
          horizontal_space(),
          text(state.device_info.firmware_version)
        ];

        let about = center(
          column![app_version, firmware_version]
            .spacing(SPACING)
            .width(Length::Fixed(300.)),
        );

        container(column![screen_name, about])
          .padding(PADDING)
          .into()
      }
      Self::ConnectedDeviceNotFound => center(screen_name.center()).padding(PADDING).into(),
    }
  }
}

/// Иконки для навигационного меню
pub enum Icon {
  /// Иконка профилей
  Profiles,

  /// Иконка настроек
  Settings,

  /// Иконка обновления
  Update,

  Download,
}
impl Icon {
  /// Возвращает SVG-иконку в виде байтового массива
  pub fn icon(&self) -> &'static [u8] {
    match self {
      Self::Profiles => include_bytes!("../../assets/icons/profiles.svg"),
      Self::Settings => include_bytes!("../../assets/icons/settings.svg"),
      Self::Update => include_bytes!("../../assets/icons/updater.svg"),
      Self::Download => include_bytes!("../../assets/icons/download.svg"),
    }
  }
}

/**
Создает стандартизированную кнопку для клавиатурной панели
# Аргументы
* `id` - Номер клавиши (1..=16)
* `profile` - Активный профиль
# Возвращает
Готовый элемент кнопки с заданными параметрами
*/
fn mk_keypad_button<'a>(state: &'a State, id: usize, profile: &Profile) -> MouseArea<'a, Message> {
  let _id = id - 1;
  let _text = profile.get_button_label(_id);

  mouse_area(
    button(
      column![
        container(text(_text.clone()).size(15)).center(Length::Fill),
        text!("#{}", id)
          .size(10)
          .align_x(Alignment::End)
          .align_y(Alignment::End),
      ]
      .width(Length::Fill)
      .height(Length::Fill),
    )
    .on_press(Message::GetButtonSettings(id, false))
    .height(BUTTON_HEIGH)
    .width(BUTTON_WIDTH)
    .style(move |theme: &Theme, status| {
      styles::button::active_write(theme, status, state, id, state.button.is_stick)
    }),
  )
  .on_right_press(Message::ClearButtonCombination(id, false))
}

fn mk_button_stick<'a>(state: &'a State, id: usize, profile: &Profile) -> MouseArea<'a, Message> {
  let _id = id - 1;
  let _text = profile.get_stick_label(_id);

  mouse_area(
    button(
      column![
        container(text(_text.clone()).size(15)).center(Length::Fill),
        text!("#{}", id)
          .size(10)
          .align_x(Alignment::End)
          .align_y(Alignment::End),
      ]
      .width(Length::Fill)
      .height(Length::Fill),
    )
    .on_press(Message::GetButtonSettings(id, true))
    .height(BUTTON_HEIGH)
    .width(BUTTON_WIDTH)
    .style(move |theme: &Theme, status| {
      styles::button::stick::active_write(theme, status, state, id, state.button.is_stick)
    }),
  )
  .on_right_press(Message::ClearButtonCombination(id, true))
}

fn mk_button_profile_row<'a>(state: &'a State, num: &'a u8) -> Row<'a, Message> {
  let (profile_assignment, write_profile) = (
    match state.is_rom {
      true => "ПЗУ",
      false => "ОЗУ",
    },
    match state.is_rom {
      true => Message::ProfileActiveWriteToRom(*num),
      false => Message::ProfileActiveWriteToRam(*num),
    },
  );

  row![
    button(text!("{} {}", profile_assignment, num).center())
      .on_press(Message::ProfileLoadRamToActive(*num))
      .width(80)
      .height(35)
      .style(|theme: &Theme, status| styles::button::active_profile(theme, status, state, *num)),
    button(
      svg(svg::Handle::from_memory(Icon::Download.icon()))
        .height(Length::Fill)
        .width(Length::Fill),
    )
    .width(50)
    .height(35)
    .on_press(write_profile)
    .style(styles::button::rounding)
  ]
  .spacing(SPACING)
}
