use super::APPLICATION_NAME;

/** Инициализирует логгер для приложения.
 * Эта функция настраивает уровень логирования для приложения, устанавливая переменную окружения `RUST_LOG` в значение "claws". Затем она инициализирует `pretty_env_logger`, который обеспечивает форматированный вывод логов в консоль.
 */
pub fn init_logger() {
    pretty_env_logger::env_logger::Builder::from_default_env()
        .filter(Some(APPLICATION_NAME), log::LevelFilter::Info)
        .init();
}
