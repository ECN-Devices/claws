use crate::assets::APPLICATION_NAME;
use log::LevelFilter;

/** Инициализирует pretty_env_logger с максимальным уровнем для модуля приложения.
 *
 * Устанавливает фильтр логов для целевого таргета `APPLICATION_NAME` на `LevelFilter::max()`
 * и инициализирует логгер из переменных окружения.
 *
 * Логгер будет выводить все сообщения уровня DEBUG и выше для модуля приложения,
 * что полезно для отладки и диагностики проблем.
 */
pub fn init_logger() {
  pretty_env_logger::env_logger::Builder::from_default_env()
    .filter(
      Some(APPLICATION_NAME.to_lowercase().as_str()),
      LevelFilter::max(),
    )
    .init();
}
