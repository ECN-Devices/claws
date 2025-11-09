/*!
Модуль страниц интерфейса и элементов управления для конфигурации профилей и настроек.

Этот модуль предоставляет структуры и методы для создания пользовательского интерфейса
приложения управления профилями контроллера. Включает в себя экраны профилей, настроек,
обновления прошивки и обработки ошибок подключения.
*/

use iced::{Element, Length, widget::text};

use crate::{
  State,
  data::profiles::Profile,
  ui::{Message, styles::HEADING_SIZE},
};

pub mod connected_device_not_found;
pub mod profiles;
pub mod settings;
pub mod updater;

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
#[macro_export]
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
      Self::Profiles => include_bytes!("../../../assets/icons/profiles.svg"),
      Self::Settings => include_bytes!("../../../assets/icons/settings.svg"),
      Self::Update => include_bytes!("../../../assets/icons/updater.svg"),
      Self::Download => include_bytes!("../../../assets/icons/download.svg"),
    }
  }
}
