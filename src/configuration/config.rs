use log::{error, info};
use serde::Serialize;
use std::{env::consts::OS, fmt::Debug, path::PathBuf};
use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncWriteExt, BufWriter},
};

use super::APPLICATION_NAME;

/// `MAX_PROFILE_NAME` определяет максимальную длину имени профиля.
const _MAX_PROFILE_NAME: usize = 15;

/// `MAX_KEYVALUE` определяет максимальное количество значений для кнопок.
const MAX_KEYVALUE: usize = 6;

/// `MAX_SWITCH_COUNT` определяет максимальное количество переключателей.
const MAX_SWITCH_COUNT: usize = 4;

/** Получает путь к директории конфигурации в зависимости от операционной системы.
 *
 * Эта функция определяет, какая операционная система используется (Linux или Windows) и возвращает
 * путь к директории конфигурации приложения. Если директория не может быть найдена, функция
 * вызывает панику с соответствующим сообщением.
 * # Возвращаемое значение
 * Возвращает `PathBuf`, представляющий путь к директории конфигурации приложения.
 */
pub async fn get_config_dir() -> PathBuf {
    match OS {
        "linux" => dirs::config_dir()
            .expect("Не могу найти папку .config")
            .join(APPLICATION_NAME),
        "windows" => dirs::document_dir()
            .expect("Не могу найти папку Документы")
            .join(APPLICATION_NAME),
        _ => panic!("Система не поддерживается: {}.", OS),
    }
}

/** Получает путь к конфигурационному файлу `claws.toml`.
 *
 * Эта функция вызывает `get_config_dir()` для получения пути к директории конфигурации и
 * добавляет к нему имя конфигурационного файла `claws.toml`.
 *
 * # Возвращаемое значение
 * Возвращает `PathBuf`, представляющий путь к конфигурационному файлу.
 */
pub async fn get_config_file() -> PathBuf {
    let config_file_path = get_config_dir().await;
    config_file_path.join(APPLICATION_NAME.to_lowercase() + ".toml")
}

/** Проверяет существование конфигурационной директории и файла, и создает их при необходимости.
 *
 * Эта функция проверяет, существует ли директория конфигурации. Если она существует, проверяет,
 * существует ли конфигурационный файл. Если файл не существует, он будет создан. Если директория
 * не существует, она будет создана, а затем будет создан и конфигурационный файл.
 *
 * # Возвращаемое значение
 * Возвращает `Result<(), std::io::Error>`, где `Ok(())` указывает на успешное выполнение,
 * а `Err(e)` содержит информацию об ошибке, если что-то пошло не так.
 */
pub async fn check_config_file() -> Result<(), std::io::Error> {
    let config_dir_path = get_config_dir().await;
    let config_file_path = get_config_file().await;

    match config_dir_path.exists() {
        true => {
            info!("Конфигурационная папка уже сущевствует");
            match config_file_path.exists() {
                true => info!("Конфигурационный файл уже сущесвтует"),
                false => {
                    info!("Создаю конфигурационный файл");
                    create_config_file().await?
                }
            }
        }
        false => {
            info!("Создаю конфигурационную папку");
            create_config_dir().await?;
            info!("Создаю конфигурационный файл");
            create_config_file().await?;
        }
    }
    Ok(())
}

/** Создает директорию конфигурации.
 *
 * Эта функция создает директорию конфигурации, используя путь, полученный из `get_config_dir()`.
 * Если создание директории завершилось неудачей, функция возвращает ошибку.
 *
 * # Возвращаемое значение
 * Возвращает `Result<(), std::io::Error>`, где `Ok(())` указывает на успешное выполнение,
 * а `Err(e)` содержит информацию об ошибке, если что-то пошло не так.
 */
pub async fn create_config_dir() -> Result<(), std::io::Error> {
    let config_dir_path = get_config_dir().await;
    match fs::create_dir_all(config_dir_path).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Ошибка создания папки конфигурации: {}", e);
            Err(e)
        }
    }
}

/** Создает конфигурационный файл `claws.toml`.
 *
 * Эта функция создает конфигурационный файл по пути, полученному из `get_config_file()`.
 * Если создание файла завершилось неудачей, функция возвращает ошибку.
 *
 * # Возвращаемое значение
 * Возвращает `Result<(), std::io::Error>`, где `Ok(())` указывает на успешное выполнение,
 * а `Err(e)` содержит информацию об ошибке, если что-то пошло не так.
 */
pub async fn create_config_file() -> Result<(), std::io::Error> {
    let config_file_path = get_config_file().await;
    match File::create(config_file_path).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Ошибка создания конфигурационного файла: {}", e);
            Err(e)
        }
    }
}

/** Структура, представляющая профиль с кнопками и значениями джойстика.
 *
 * Эта структура содержит имя профиля, массив кнопок и массив значений джойстика.
 *
 * # Поля
 * - `name`: строка, представляющая имя профиля.
 * - `buttons`: двумерный массив, представляющий значения кнопок, где размерность определяется
 *    константой `MAX_KEYVALUE` и `MAX_SWITCH_COUNT`.
 * - `joystick_key_value`: массив, представляющий значения джойстика.
 */
#[derive(Debug, Serialize)]
struct Profile {
    name: String,
    buttons: [[u16; MAX_KEYVALUE]; MAX_SWITCH_COUNT],
    joystick_key_value: [u16; 4],
}

/** Реализует стандартные значения для структуры `Profile`.
 *
 * Эта реализация устанавливает имя профиля в пустую строку, а массивы кнопок и значений джойстика
 * инициализирует нулями.
 */
impl Default for Profile {
    fn default() -> Self {
        Profile {
            name: "".to_string(),
            buttons: [[0; MAX_KEYVALUE]; MAX_SWITCH_COUNT],
            joystick_key_value: [0; 4],
        }
    }
}

/** Обновляет конфигурационный файл по указанному пути.
 *
 * Эта функция создает новый профиль с значениями по умолчанию и записывает его в файл
 * по указанному пути. Если запись завершается неудачей, функция возвращает ошибку.
 *
 * # Параметры
 * - `file_path`: путь к конфигурационному файлу, который необходимо обновить.
 *
 * # Возвращаемое значение
 * Возвращает `Result<(), tokio::io::Error>`, где `Ok(())` указывает на успешное выполнение,
 * а `Err(e)` содержит информацию об ошибке, если что-то пошло не так.
 */
pub async fn update_config_file(file_path: PathBuf) -> tokio::io::Result<()> {
    let config_toml = Profile {
        ..Default::default()
    };

    // println!("{:#?}", config_toml);

    let toml = toml::to_string(&config_toml).unwrap();

    let config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .await?;
    let mut buffer = BufWriter::new(config_file);

    buffer.write_all(toml.as_bytes()).await?;
    buffer.flush().await?;
    Ok(())
}
