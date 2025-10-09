use crate::{State, ui::styles::BORDER_RADIUS};
use iced::{Border, Color, Theme, widget::button};

/**
Создает стиль кнопки с закругленными углами

Применяет стандартные закругления к кнопке, используя базовый стиль темы.

# Аргументы
* `theme` - Текущая тема приложения
* `status` - Состояние кнопки (нажата, наведена и т.д.)

# Возвращает
Стиль кнопки с закругленными углами
*/
pub fn rounding(theme: &Theme, status: button::Status) -> button::Style {
  button::Style {
    border: Border {
      radius: BORDER_RADIUS.into(),
      ..Default::default()
    },
    ..button::primary(theme, status)
  }
}

/**
Создает стиль кнопки активного профиля с подсветкой

Выделяет активный профиль синим цветом, остальные профили
отображаются в стандартном стиле.

# Аргументы
* `theme` - Текущая тема приложения
* `status` - Состояние кнопки (нажата, наведена и т.д.)
* `state` - Состояние приложения для определения активного профиля
* `number` - Номер профиля (1-4)

# Возвращает
Стиль кнопки с подсветкой для активного профиля
*/
pub fn active_profile(
  theme: &Theme,
  status: button::Status,
  state: &State,
  number: u8,
) -> button::Style {
  match state.active_profile_num {
    Some(i) => match i == number {
      true => button::Style {
        background: Some(iced::Background::Color(Color::parse("#778fe6").unwrap())),
        border: Border {
          radius: BORDER_RADIUS.into(),
          ..Default::default()
        },
        ..button::primary(theme, status)
      },
      false => button::Style {
        border: Border {
          radius: BORDER_RADIUS.into(),
          ..Default::default()
        },
        ..button::primary(theme, status)
      },
    },
    None => button::primary(theme, status),
  }
}

/**
Создает стиль кнопки в режиме записи комбинации

Выделяет кнопку синим цветом с увеличенными закруглениями,
когда она находится в режиме записи комбинации клавиш.

# Аргументы
* `theme` - Текущая тема приложения
* `status` - Состояние кнопки (нажата, наведена и т.д.)
* `state` - Состояние приложения для определения режима записи
* `id` - Идентификатор кнопки
* `is_stick` - Признак, что это кнопка стика

# Возвращает
Стиль кнопки с подсветкой для режима записи
*/
pub fn active_write(
  theme: &Theme,
  status: button::Status,
  state: &State,
  id: usize,
  is_stick: bool,
) -> button::Style {
  match (state.button.id == id, state.allow_write, is_stick) {
    (true, true, false) => button::Style {
      background: Some(iced::Background::Color(Color::parse("#778fe6").unwrap())),
      border: Border {
        radius: (BORDER_RADIUS * 2.).into(),
        ..Default::default()
      },
      ..button::primary(theme, status)
    },
    _ => button::Style {
      border: Border {
        radius: (BORDER_RADIUS * 2.).into(),
        ..Default::default()
      },
      ..button::primary(theme, status)
    },
  }
}

/// Модуль стилей для кнопок стика
pub mod stick {
  use crate::{State, ui::styles::BORDER_RADIUS};
  use iced::{Border, Color, Theme, widget::button};

  /**
  Создает стиль кнопки направления стика в режиме записи

  Аналогично `active_write`, но специально для кнопок направлений стика.
  Выделяет кнопку синим цветом при записи комбинации.

  # Аргументы
  * `theme` - Текущая тема приложения
  * `status` - Состояние кнопки (нажата, наведена и т.д.)
  * `state` - Состояние приложения для определения режима записи
  * `id` - Идентификатор направления стика
  * `is_stick` - Должно быть true для кнопок стика

  # Возвращает
  Стиль кнопки стика с подсветкой для режима записи
  */
  pub fn active_write(
    theme: &Theme,
    status: button::Status,
    state: &State,
    id: usize,
    is_stick: bool,
  ) -> button::Style {
    match (state.button.id == id, state.allow_write, is_stick) {
      (true, true, true) => button::Style {
        background: Some(iced::Background::Color(Color::parse("#778fe6").unwrap())),
        border: Border {
          radius: (BORDER_RADIUS * 2.).into(),
          ..Default::default()
        },
        ..button::primary(theme, status)
      },
      _ => button::Style {
        border: Border {
          radius: (BORDER_RADIUS * 2.).into(),
          ..Default::default()
        },
        ..button::primary(theme, status)
      },
    }
  }
}
