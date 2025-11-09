use iced::{
  Element, Length,
  widget::{center, column, container, horizontal_space, row, text},
};

use crate::{
  State,
  assets::APPLICATION_VERSION,
  ui::{
    pages::Pages,
    styles::{self, HEADING_SIZE, PADDING, SPACING},
    update::Message,
  },
};

impl Pages {
  /**
  Создает интерфейс экрана обновления прошивки

  Отображает информацию о версиях приложения и прошивки устройства.

  # Аргументы
  * `state` - Состояние приложения с информацией об устройстве
  * `screen_name` - Заголовок экрана

  # Возвращает
  Элемент интерфейса экрана обновления
  */
  pub fn updater_screen<'a>(
    state: &'a State,
    screen_name: Element<'a, Message>,
  ) -> Element<'a, Message> {
    let version_info = column![
      row![
        text("Версия приложения:"),
        horizontal_space(),
        text(APPLICATION_VERSION)
      ],
      row![
        text("Версия прошивки:"),
        horizontal_space(),
        text(state.device_info.firmware_version)
      ],
    ]
    .spacing(SPACING);

    let version_panel = center(
      column![
        container(text("Версия").size(HEADING_SIZE))
          .style(styles::container::round_bordered_box_header)
          .padding(PADDING)
          .width(Length::Fill),
        container(version_info)
          .style(styles::container::round_bordered_box)
          .padding(PADDING)
          .width(Length::Fill),
      ]
      .width(Length::Fixed(300.)),
    );

    container(column![screen_name, version_panel])
      .padding(PADDING)
      .into()
  }
}
