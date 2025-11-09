use iced::{
  Alignment, Element, Length,
  widget::{button, center, column, container, text},
};

use crate::{
  State, mk_button,
  ui::{
    pages::Pages,
    styles::{self, BUTTON_HEIGH, HEADING_SIZE, PADDING, SPACING},
    update::Message,
  },
};

impl Pages {
  /**
  Создает интерфейс экрана настроек

  В зависимости от состояния калибровки отображает либо обычные настройки,
  либо интерфейс калибровки стика.

  # Аргументы
  * `state` - Состояние приложения
  * `screen_name` - Заголовок экрана настроек

  # Возвращает
  Элемент интерфейса экрана настроек
  */
  pub fn settings_screen<'a>(
    state: &'a State,
    screen_name: Element<'a, Message>,
  ) -> Element<'a, Message> {
    let settings_content = match state.stick_callibrate {
      true => Self::build_stick_calibration_ui(state),
      false => Self::build_regular_settings_ui(),
    };

    column![screen_name, center(settings_content)]
      .spacing(SPACING)
      .padding(PADDING)
      .into()
  }

  /**
  Создает интерфейс обычных настроек (когда калибровка не активна)

  Содержит кнопки для:
  - Перезагрузки в bootloader
  - Запуска калибровки стика

  # Возвращает
  Вертикальную колонку с кнопками системных настроек
  */
  fn build_regular_settings_ui<'a>() -> Element<'a, Message> {
    let reboot_button = mk_button!(
      container("Перезагрузить в bootloader").center_x(Length::Fill),
      Message::RebootToBootloader
    )
    .width(Length::Fill);

    let calibration_button = mk_button!(
      container("Калибровать стик").center_x(Length::Fill),
      Message::StickInitCalibration
    )
    .width(Length::Fill);

    let profile_import = mk_button!(
      container("Импорт профиля").center_x(Length::Fill),
      Message::ProfileImport
    )
    .width(Length::Fill);

    let profile_export = mk_button!(
      container("Экспорт профилей").center_x(Length::Fill),
      Message::ProfilesExport
    )
    .width(Length::Fill);

    column![
      reboot_button,
      calibration_button,
      profile_import,
      profile_export
    ]
    .width(270)
    .align_x(Alignment::Center)
    .spacing(SPACING)
    .into()
  }

  /**
  Создает интерфейс калибровки стика

  В зависимости от этапа калибровки отображает:
  - Инструкции перед началом
  - Обратный отсчет во время калибровки
  - Результаты после завершения

  # Аргументы
  * `state` - Состояние приложения с информацией о калибровке

  # Возвращает
  Соответствующий интерфейс для текущего этапа калибровки
  */
  fn build_stick_calibration_ui(state: &State) -> Element<'_, Message> {
    match state.stick_callibrate_time {
      Some(time) => Self::build_calibration_countdown(time),
      None => match state.stick_show_calibrate_parameters {
        true => Self::build_calibration_results(state),
        false => Self::build_calibration_instructions(),
      },
    }
  }

  /**
  Создает интерфейс обратного отсчета во время калибровки

  Показывает оставшееся время вращения стика пользователем.

  # Аргументы
  * `time` - Время начала калибровки для расчета оставшегося времени

  # Возвращает
  Интерфейс с обратным отсчетом
  */
  fn build_calibration_countdown<'a>(time: std::time::Instant) -> Element<'a, Message> {
    let seconds_remaining = 6 - time.elapsed().as_secs();

    column![
      Self::create_calibration_header("Калибровка стика"),
      Self::create_calibration_box(text!("Вращайте стик {}", seconds_remaining).size(20)),
    ]
    .width(600)
    .into()
  }

  /**
  Создает интерфейс отображения результатов калибровки

  Показывает рассчитанные параметры калибровки и кнопку завершения.

  # Аргументы
  * `state` - Состояние приложения с параметрами калибровки

  # Возвращает
  Интерфейс с результатами калибровки
  */
  fn build_calibration_results(state: &State) -> Element<'_, Message> {
    let parameters_text = text(format!(
            "Центр по оси X: {}\nЦентр по оси Y: {}\nВнешняя мертвая зона: {}\nВнутренняя мертвая зона: {}",
            state.stick_info.center_x,
            state.stick_info.center_y,
            state.stick_info.external_deadzone,
            state.stick_info.internal_deadzone
        ))
        .size(20);

    let done_button =
      container(mk_button!("Готово", Message::StickEndCalibration)).align_right(Length::Fill);

    column![
      Self::create_calibration_header("Параметры калибровки стика"),
      Self::create_calibration_box(column![parameters_text, done_button].spacing(SPACING)),
    ]
    .width(600)
    .into()
  }

  /**
  Создает интерфейс с инструкциями перед началом калибровки

  Объясняет пользователю процесс калибровки и предоставляет кнопку для начала.

  # Возвращает
  Интерфейс с инструкциями калибровки
  */
  fn build_calibration_instructions<'a>() -> Element<'a, Message> {
    let next_button =
      container(mk_button!("Далее", Message::StickStartCalibration)).align_right(Length::Fill);

    column![
      Self::create_calibration_header("Калибровка стика"),
      Self::create_calibration_box(
        column![
          text("После нажатия на кнопку 'Далее' начнется процесс калибровки стика, вам необходимо вращать стик в крайнем положении пока не закончиться обратный отсчет."),
          next_button
        ].spacing(SPACING)
       ),
    ]
    .width(600)
    .into()
  }

  /**
  Создает стандартизированный заголовок для экранов калибровки

  # Аргументы
  * `title` - Текст заголовка

  # Возвращает
  Стилизованный контейнер с заголовком
  */
  fn create_calibration_header(title: &str) -> Element<'_, Message> {
    container(text(title).size(HEADING_SIZE))
      .style(styles::container::round_bordered_box_header)
      .width(Length::Fill)
      .padding(PADDING)
      .into()
  }

  /**
  Создает стандартизированный контейнер для содержимого калибровки

  # Аргументы
  * `content` - Внутреннее содержимое контейнера

  # Возвращает
  Стилизованный контейнер с содержимым
  */
  fn create_calibration_box<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
      .center_x(Length::Fill)
      .style(styles::container::round_bordered_box)
      .width(Length::Fill)
      .padding(PADDING)
      .into()
  }
}
