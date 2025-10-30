/*!
Модуль страниц интерфейса и элементов управления для конфигурации профилей и настроек.

Этот модуль предоставляет структуры и методы для создания пользовательского интерфейса
приложения управления профилями контроллера. Включает в себя экраны профилей, настроек,
обновления прошивки и обработки ошибок подключения.
*/

use iced::{
  Alignment, Element, Length, Theme,
  widget::{
    MouseArea, Row, Scrollable, button, center, column, container, horizontal_rule,
    horizontal_space, mouse_area, row,
    scrollable::{Direction, Scrollbar},
    slider, svg, text, text_input, toggler, vertical_rule,
  },
};

use crate::{
  State,
  assets::APPLICATION_VERSION,
  data::profiles::Profile,
  ui::{
    Message,
    styles::{
      self, BUTTON_HEIGH, BUTTON_HEIGH_PROFILE, BUTTON_WIDTH_PROFILE, HEADING_SIZE, PADDING,
      SPACING,
    },
  },
};

/**
Макрос для создания стандартизированных кнопок с закругленными углами

# Аргументы
* `$text` - Текст кнопки или элемент интерфейса
* `$on_press` - Сообщение, отправляемое при нажатии на кнопку

# Пример
```
mk_button!("Текст кнопки", Message::SomeAction)
```
*/
macro_rules! mk_button {
  ($text:expr, $on_press:expr) => {
    button($text)
      .height(BUTTON_HEIGH)
      .on_press($on_press)
      .style(styles::button::rounding)
  };
}

/**
Перечисление экранов приложения

Определяет все возможные состояния пользовательского интерфейса приложения
и управляет навигацией между ними.
*/
#[derive(Clone, Debug, Default)]
pub enum Pages {
  /**
  Экран управления профилями (экран по умолчанию)

  Позволяет пользователю просматривать, редактировать и управлять профилями настроек контроллера,
  включая настройку кнопок и калибровку стика.
  */
  #[default]
  Profiles,

  /**
  Экран настроек

  Содержит системные настройки приложения, включая калибровку стика
  и перезагрузку в режим bootloader.
  */
  Settings,

  /**
  Экран обновления прошивки

  Отображает информацию о версиях приложения и прошивки устройства.
  */
  Updater,

  /**
  Экран отображения ошибки подключения устройства

  Показывается когда целевое устройство не обнаружено или недоступно.
  */
  ConnectedDeviceNotFound,
}

impl Pages {
  /**
  Возвращает локализованное имя текущего экрана

  # Возвращает
  Строковый срез с именем экрана на русском языке

  # Пример
  ```
  let page = Pages::Profiles;
  assert_eq!(page.name(), "Профили");
  ```
  */
  fn name(&self) -> &str {
    match self {
      Self::Profiles => "Профили",
      Self::Settings => "Настройки",
      Self::Updater => "Обновление",
      Self::ConnectedDeviceNotFound => "Устройство не найдено",
    }
  }

  /**
  Генерирует содержимое экрана на основе текущего состояния приложения

  # Аргументы
  * `state` - Ссылка на текущее состояние приложения
  * `profile` - Ссылка на активный профиль настроек

  # Возвращает
  Элемент интерфейса Iced для отображения соответствующего экрана

  # Паникует
  Не паникует при нормальных условиях работы
  */
  pub fn get_content<'a>(state: &'a State, profile: &'a Profile) -> Element<'a, Message> {
    let screen_name = Self::create_screen_header(state);

    match state.pages {
      Self::Profiles => Self::profiles_screen(state, profile, screen_name),
      Self::Settings => Self::settings_screen(state, screen_name),
      Self::Updater => Self::updater_screen(state, screen_name),
      Self::ConnectedDeviceNotFound => Self::device_not_found_screen(screen_name),
    }
  }

  /**
  Создает заголовок экрана с соответствующим именем

  # Аргументы
  * `state` - Ссылка на состояние приложения для определения текущего экрана

  # Возвращает
  Текстовый элемент с названием экрана в стиле заголовка
  */
  fn create_screen_header(state: &State) -> Element<'_, Message> {
    text(state.pages.name())
      .size(HEADING_SIZE)
      .width(match state.pages {
        Pages::Profiles => Length::Shrink,
        Pages::ConnectedDeviceNotFound => Length::Shrink,
        _ => Length::Fill,
      })
      .into()
  }

  /**
  Создает интерфейс экрана управления профилями

  Разделен на две основные панели:
  - Левая панель: список профилей и управление ими
  - Правая панель: редактирование активного профиля

  # Аргументы
  * `state` - Состояние приложения
  * `profile` - Активный профиль для редактирования
  * `_screen_name` - Заголовок экрана (не используется, но требуется для консистентности)

  # Возвращает
  Элемент интерфейса с разделенными панелями управления профилями
  */
  fn profiles_screen<'a>(
    state: &'a State,
    profile: &'a Profile,
    _screen_name: Element<'a, Message>,
  ) -> Element<'a, Message> {
    // Левая панель - управление профилями
    let profiles_panel = Self::build_profiles_panel(state);

    // Правая панель - активный профиль
    let active_profile_panel = Self::build_active_profile_panel(state, profile);

    row![profiles_panel, vertical_rule(2), active_profile_panel].into()
  }

  /**
  Строит левую панель управления профилями

  Содержит:
  - Переключатель режимов ОЗУ/ПЗУ
  - Кнопки быстрого доступа к профилям 1-4
  - Кнопки импорта/экспорта профилей
  - Прокручиваемый список всех доступных профилей

  # Аргументы
  * `state` - Состояние приложения

  # Возвращает
  Вертикальную колонку с элементами управления профилями
  */
  fn build_profiles_panel(state: &State) -> Element<'_, Message> {
    let mode_toggle = row![
      text("ОЗУ"),
      toggler(state.is_rom).on_toggle(|_| Message::WriteButtonIsRom),
      text("ПЗУ")
    ]
    .align_y(Alignment::Center)
    .spacing(SPACING);

    let ram_rom_buttons =
      column((1..=4).map(|id| mk_button_profile_row(state, id).into())).spacing(SPACING);

    let profile_management = column![mk_button!(
      container("Создать профиль").center_x(Length::Fill),
      Message::ProfileNew
    )]
    .height(Length::Shrink)
    .spacing(SPACING);

    let profile_list = Self::build_profile_list(state);

    column![
      text("Профили").size(HEADING_SIZE),
      mode_toggle,
      ram_rom_buttons,
      horizontal_rule(2),
      profile_management,
      container(profile_list),
    ]
    .align_x(Alignment::Center)
    .spacing(SPACING)
    .padding(PADDING)
    .width(260)
    .into()
  }

  /**
  Создает прокручиваемый список всех доступных профилей

  # Аргументы
  * `state` - Состояние приложения, содержащее вектор профилей

  # Возвращает
  Прокручиваемый контейнер с кнопками выбора профилей
  */
  fn build_profile_list(state: &State) -> Element<'_, Message> {
    let profile_buttons = column(state.profiles_local_vec.iter().map(|(idx, profile)| {
      row![
        mk_button!(
          container(text(&profile.name)).center_x(Length::Fill),
          Message::ProfileLoadLocal(*idx)
        )
        .style(move |theme: &Theme, status| {
          styles::button::active_profile_id(theme, status, state, *idx)
        }),
        mk_button!(
          container(svg(svg::Handle::from_memory(include_bytes!(
            "../../assets/icons/trash.svg"
          ))))
          .center(Length::Fill),
          Message::ProfileRemove(*idx)
        )
        .width(BUTTON_HEIGH)
        .height(BUTTON_HEIGH)
      ]
      .spacing(SPACING)
      .into()
    }))
    .spacing(SPACING);

    Scrollable::new(profile_buttons)
      .direction(Direction::Vertical(Scrollbar::new()))
      .spacing(SPACING)
      .into()
  }

  /**
  Строит правую панель редактирования активного профиля

  Содержит:
  - Поле ввода имени профиля
  - Сетку кнопок клавиатуры (16 кнопок)
  - Элементы управления стиком

  # Аргументы
  * `state` - Состояние приложения
  * `profile` - Активный профиль для редактирования

  # Возвращает
  Область редактирования профиля с элементами управления
  */
  fn build_active_profile_panel<'a>(
    state: &'a State,
    profile: &'a Profile,
  ) -> Element<'a, Message> {
    let profile_name_input = match state.profile_on_keypad {
      true => container(
        text_input(&profile.name, &profile.name)
          .align_x(Alignment::Center)
          .size(25)
          .width(300)
          .style(styles::text_input::rounding),
      )
      .center_x(Length::Fill),
      false => container(
        text_input(&profile.name, &profile.name)
          .align_x(Alignment::Center)
          .size(25)
          .width(300)
          .on_input(Message::ProfileUpdateName)
          .style(styles::text_input::rounding),
      )
      .center_x(Length::Fill),
    };

    let keypad_grid = Self::build_keypad_grid(state, profile);
    let stick_controls = Self::build_stick_controls(state, profile);

    let controls_layout = row![keypad_grid, column![stick_controls].spacing(SPACING)]
      .spacing(SPACING)
      .align_y(Alignment::End);

    match state.profile_on_keypad {
      true => column![
        profile_name_input,
        container(controls_layout).center(Length::Fill)
      ]
      .padding(PADDING)
      .spacing(SPACING)
      .into(),
      false => mouse_area(
        column![
          profile_name_input,
          container(controls_layout).center(Length::Fill)
        ]
        .padding(PADDING)
        .spacing(SPACING),
      )
      .on_press(Message::DisallowWriteButtonCombination)
      .into(),
    }
  }

  /**
  Создает сетку из 16 кнопок клавиатуры, организованных в 4 колонки

  Распределение кнопок:
  - Колонка 1: кнопки 1-4
  - Колонка 2: кнопки 5-8
  - Колонка 3: кнопки 9-12
  - Колонка 4: кнопки 13-16

  # Аргументы
  * `state` - Состояние приложения
  * `profile` - Профиль для получения меток кнопок

  # Возвращает
  Горизонтальную строку с 4 колонками кнопок
  */
  fn build_keypad_grid<'a>(state: &'a State, profile: &'a Profile) -> Element<'a, Message> {
    let col_1 = column((1..=4).map(|id| mk_keypad_button(state, id, profile))).spacing(SPACING);

    let col_2 = column((5..=8).map(|id| mk_keypad_button(state, id, profile))).spacing(SPACING);

    let col_3 = column((9..=12).map(|id| mk_keypad_button(state, id, profile))).spacing(SPACING);

    let col_4 = column((13..=16).map(|id| mk_keypad_button(state, id, profile))).spacing(SPACING);

    row![col_1, col_2, col_3, col_4].spacing(SPACING).into()
  }

  /**
  Создает элементы управления стиком

  Включает:
  - Слайдер для настройки мертвой зоны стика
  - Кнопки направлений стика (вверх, вниз, влево, вправо)
  - Центральную неактивную кнопку

  # Аргументы
  * `state` - Состояние приложения
  * `profile` - Профиль для получения меток кнопок стика

  # Возвращает
  Вертикальную колонку с элементами управления стиком
  */
  fn build_stick_controls<'a>(state: &'a State, profile: &'a Profile) -> Element<'a, Message> {
    let deadzone_controls = match state.profile_on_keypad {
      true => column![
        text!("Мёртвая зона: {}%", state.profile.stick.deadzone).size(25),
        slider(1..=100, state.profile.stick.deadzone, |_| Message::None),
      ]
      .align_x(Alignment::Center),
      false => column![
        text!("Мёртвая зона: {}%", state.profile.stick.deadzone).size(25),
        slider(
          1..=100,
          state.profile.stick.deadzone,
          Message::WriteDeadZone
        )
        .step(1),
      ]
      .align_x(Alignment::Center),
    };

    let stick_buttons = column![
      deadzone_controls,
      mk_button_stick(state, 1, profile), // Вверх
      row![
        mk_button_stick(state, 4, profile), // Влево
        Self::create_center_stick_button(state),
        mk_button_stick(state, 2, profile), // Вправо
      ]
      .spacing(SPACING),
      mk_button_stick(state, 3, profile), // Вниз
    ]
    .align_x(Alignment::Center)
    .spacing(SPACING);

    column![stick_buttons].width(317.).spacing(SPACING).into()
  }

  /**
  Создает центральную неактивную кнопку для стика

  Используется как визуальный центр в компоновке кнопок направлений стика.

  # Аргументы
  * `state` - Состояние приложения для применения стилей

  # Возвращает
  Пустую кнопку с соответствующими стилями
  */
  fn create_center_stick_button(state: &State) -> Element<'_, Message> {
    button("")
      .height(BUTTON_HEIGH_PROFILE)
      .width(BUTTON_WIDTH_PROFILE)
      .style(move |theme: &Theme, status| {
        styles::button::stick::active_write(theme, status, state, 0, state.button.is_stick)
      })
      .into()
  }

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
  fn settings_screen<'a>(
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

  /**
  Создает интерфейс экрана обновления прошивки

  Отображает информацию о версиях приложения и прошивки устройства.

  # Аргументы
  * `state` - Состояние приложения с информацией об устройстве
  * `screen_name` - Заголовок экрана

  # Возвращает
  Элемент интерфейса экрана обновления
  */
  fn updater_screen<'a>(
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

  /**
  Создает интерфейс экрана ошибки подключения устройства

  Показывает сообщение о том, что устройство не найдено.

  # Аргументы
  * `screen_name` - Заголовок с сообщением об ошибке

  # Возвращает
  Центрированное сообщение об ошибке
  */
  fn device_not_found_screen(screen_name: Element<'_, Message>) -> Element<'_, Message> {
    center(screen_name).padding(PADDING).into()
  }
}

/**
Создает кнопку для клавиатурной панели с поддержкой правого клика

Каждая кнопка отображает:
- Назначенную клавишу (метку) в центре
- Номер кнопки в правом нижнем углу

Поддерживает:
- Левый клик: открытие настроек кнопки
- Правый клик: очистка комбинации кнопки

# Аргументы
* `state` - Состояние приложения для определения стилей
* `id` - Идентификатор кнопки (1-16)
* `profile` - Профиль для получения метки кнопки

# Возвращает
Интерактивную кнопку клавиатуры
*/
fn mk_keypad_button<'a>(state: &'a State, id: usize, profile: &Profile) -> Element<'a, Message> {
  let button_index = id - 1;
  let label = profile.get_button_label(button_index);

  match state.profile_on_keypad {
    true => button(
      column![
        container(text(label).size(15)).center(Length::Fill),
        text!("#{}", id)
          .size(10)
          .align_x(Alignment::End)
          .align_y(Alignment::End),
      ]
      .width(Length::Fill)
      .height(Length::Fill),
    )
    .on_press(Message::GetButtonSettings(id, false))
    .height(BUTTON_HEIGH_PROFILE)
    .width(BUTTON_WIDTH_PROFILE)
    .style(move |theme: &Theme, status| {
      styles::button::active_write(theme, status, state, id, state.button.is_stick)
    })
    .into(),
    false => mouse_area(
      button(
        column![
          container(text(label).size(15)).center(Length::Fill),
          text!("#{}", id)
            .size(10)
            .align_x(Alignment::End)
            .align_y(Alignment::End),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
      )
      .on_press(Message::GetButtonSettings(id, false))
      .height(BUTTON_HEIGH_PROFILE)
      .width(BUTTON_WIDTH_PROFILE)
      .style(move |theme: &Theme, status| {
        styles::button::active_write(theme, status, state, id, state.button.is_stick)
      }),
    )
    .on_right_press(Message::ClearButtonCombination(id, false))
    .into(),
  }

  // mouse_area(
  //   button(
  //     column![
  //       container(text(label).size(15)).center(Length::Fill),
  //       text!("#{}", id)
  //         .size(10)
  //         .align_x(Alignment::End)
  //         .align_y(Alignment::End),
  //     ]
  //     .width(Length::Fill)
  //     .height(Length::Fill),
  //   )
  //   .on_press(Message::GetButtonSettings(id, false))
  //   .height(BUTTON_HEIGH_PROFILE)
  //   .width(BUTTON_WIDTH_PROFILE)
  //   .style(move |theme: &Theme, status| {
  //     styles::button::active_write(theme, status, state, id, state.button.is_stick)
  //   }),
  // )
  // .on_right_press(Message::ClearButtonCombination(id, false))
}

/**
Создает кнопку направления стика с поддержкой правого клика

Аналогична кнопкам клавиатуры, но предназначена для направлений стика.

# Аргументы
* `state` - Состояние приложения для определения стилей
* `id` - Идентификатор направления (1-4)
* `profile` - Профиль для получения метки направления

# Возвращает
Интерактивную кнопку направления стика
*/
fn mk_button_stick<'a>(state: &'a State, id: usize, profile: &Profile) -> MouseArea<'a, Message> {
  let button_index = id - 1;
  let label = profile.get_stick_label(button_index);

  mouse_area(
    button(
      column![
        container(text(label).size(15)).center(Length::Fill),
        text!("#{}", id)
          .size(10)
          .align_x(Alignment::End)
          .align_y(Alignment::End),
      ]
      .width(Length::Fill)
      .height(Length::Fill),
    )
    .on_press(Message::GetButtonSettings(id, true))
    .height(BUTTON_HEIGH_PROFILE)
    .width(BUTTON_WIDTH_PROFILE)
    .style(move |theme: &Theme, status| {
      styles::button::stick::active_write(theme, status, state, id, state.button.is_stick)
    }),
  )
  .on_right_press(Message::ClearButtonCombination(id, true))
}

/**
Создает строку с кнопками управления профилем для быстрого доступа

Каждая строка содержит:
- Основную кнопку профиля с номером и типом (ОЗУ/ПЗУ)
- Кнопку загрузки/выгрузки профиля

# Аргументы
* `state` - Состояние приложения для определения режима (ОЗУ/ПЗУ)
* `num` - Номер профиля (1-4)

# Возвращает
Горизонтальную строку с двумя кнопками управления профилем
*/
fn mk_button_profile_row<'a>(state: &'a State, id: usize) -> Row<'a, Message> {
  let (profile_type, write_message) = if state.is_rom {
    ("ПЗУ", Message::ProfileActiveWriteToRom(id as u8))
  } else {
    ("ОЗУ", Message::ProfileActiveWriteToRam(id as u8))
  };

  let block = if let Some(pr_num) = state.active_profile_id
    && pr_num == id
  {
    column![button("").width(10).style(styles::button::rounding)]
  } else {
    column![button("").width(10).style(styles::button::transparent)]
  };

  row![
    block,
    button(text!("{} {}", profile_type, id).center())
      .on_press(Message::ProfileLoadKeypad(id))
      .width(80)
      .height(35)
      .style(move |theme: &Theme, status| {
        styles::button::active_profile(theme, status, state, id)
      }),
    button(
      svg(svg::Handle::from_memory(Icon::Download.icon()))
        .height(Length::Fill)
        .width(Length::Fill),
    )
    .width(50)
    .height(BUTTON_HEIGH)
    .on_press(write_message)
    .style(styles::button::rounding)
  ]
  .spacing(SPACING)
}

/**
Перечисление иконок для навигационного меню и элементов интерфейса

Каждая иконка представлена в формате SVG и загружается из ресурсов приложения.
*/
pub enum Icon {
  /// Иконка для раздела профилей
  Profiles,

  /// Иконка для раздела настроек
  Settings,

  /// Иконка для раздела обновления
  Update,

  /// Иконка загрузки/выгрузки
  Download,
}

impl Icon {
  /**
  Возвращает SVG-иконку в виде байтового массива

  Иконки загружаются из ресурсов приложения во время компиляции.

  # Возвращает
  Ссылку на статический байтовый массив с SVG-данными

  # Паникует
  Не паникует при нормальных условиях работы
  */
  pub fn icon(&self) -> &'static [u8] {
    match self {
      Self::Profiles => include_bytes!("../../assets/icons/profiles.svg"),
      Self::Settings => include_bytes!("../../assets/icons/settings.svg"),
      Self::Update => include_bytes!("../../assets/icons/updater.svg"),
      Self::Download => include_bytes!("../../assets/icons/download.svg"),
    }
  }
}
