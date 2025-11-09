use iced::{
  Alignment, Element, Length, Theme,
  widget::{
    Scrollable, button, column, container, horizontal_rule, mouse_area, row,
    scrollable::{Direction, Scrollbar},
    slider, svg, text, text_input, toggler, vertical_rule,
  },
};

use crate::{
  State,
  data::profiles::Profile,
  mk_button,
  ui::{
    pages::{Icon, Pages},
    styles::{
      self, BUTTON_HEIGH, BUTTON_HEIGH_PROFILE, BUTTON_WIDTH_PROFILE, HEADING_SIZE, PADDING,
      RULE_WIDTH, SPACING,
    },
    update::Message,
  },
};

impl Pages {
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
  pub fn profiles_screen<'a>(
    state: &'a State,
    profile: &'a Profile,
    _screen_name: Element<'a, Message>,
  ) -> Element<'a, Message> {
    // Левая панель - управление профилями
    let profiles_panel = Self::build_profiles_panel(state);

    // Правая панель - активный профиль
    let active_profile_panel = Self::build_active_profile_panel(state, profile);

    row![
      profiles_panel,
      vertical_rule(RULE_WIDTH),
      active_profile_panel
    ]
    .into()
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
  pub fn build_profiles_panel(state: &State) -> Element<'_, Message> {
    let mode_toggle = row![
      text("ОЗУ"),
      toggler(state.is_rom).on_toggle(|_| Message::WriteButtonIsRom),
      text("ПЗУ")
    ]
    .align_y(Alignment::Center)
    .spacing(SPACING);

    let ram_rom_buttons =
      column((1..=4).map(|id| mk_button_profile_row(state, id))).spacing(SPACING);

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
      horizontal_rule(RULE_WIDTH),
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
    let profile_buttons = column(state.profiles_local_vec.iter().enumerate().map(
      |(idx, profile)| {
        row![
          mk_button!(
            container(text(&profile.name)).center_x(Length::Fill),
            Message::ProfileLoadLocal(idx)
          )
          .style(move |theme: &Theme, status| {
            styles::button::active_profile_id(theme, status, state, idx)
          }),
          mk_button!(
            container(svg(svg::Handle::from_memory(include_bytes!(
              "../../../assets/icons/trash.svg"
            ))))
            .center(Length::Fill),
            Message::ProfileRemove(idx)
          )
          .width(BUTTON_HEIGH)
          .height(BUTTON_HEIGH)
        ]
        .spacing(SPACING)
        .into()
      },
    ))
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
fn mk_button_stick<'a>(state: &'a State, id: usize, profile: &Profile) -> Element<'a, Message> {
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
  .into()
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
fn mk_button_profile_row<'a>(state: &'a State, id: usize) -> Element<'a, Message> {
  let (profile_type, write_message) = if state.is_rom {
    ("ПЗУ", Message::ProfileActiveWriteToRom(id as u8))
  } else {
    ("ОЗУ", Message::ProfileActiveWriteToRam(id as u8))
  };

  let block = if let Some(pr_num) = state.request_active_profile_id
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
  .into()
}
