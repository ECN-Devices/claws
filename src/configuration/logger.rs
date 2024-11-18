use super::APPLICATION_NAME;

/** Инициализирует логгер для приложения.
 * Эта функция настраивает уровень логирования для приложения, устанавливая переменную окружения `RUST_LOG` в значение "claws". Затем она инициализирует `pretty_env_logger`, который обеспечивает форматированный вывод логов в консоль.
 */
pub fn init_logger() {
    std::env::set_var("RUST_LOG", APPLICATION_NAME.to_lowercase());
    pretty_env_logger::init();
}
