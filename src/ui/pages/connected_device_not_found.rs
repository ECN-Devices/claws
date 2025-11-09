use iced::{Element, widget::center};

use crate::ui::{pages::Pages, styles::PADDING, update::Message};

impl Pages {
  /**
  Создает интерфейс экрана ошибки подключения устройства

  Показывает сообщение о том, что устройство не найдено.

  # Аргументы
  * `screen_name` - Заголовок с сообщением об ошибке

  # Возвращает
  Центрированное сообщение об ошибке
  */
  pub fn device_not_found_screen(screen_name: Element<'_, Message>) -> Element<'_, Message> {
    center(screen_name).padding(PADDING).into()
  }
}
