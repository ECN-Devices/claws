use log::{error, info};
use serde::Serialize;
use std::{env::consts::OS, fmt::Debug, path::PathBuf};
use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncWriteExt, BufWriter},
};

const _MAX_PROFILE_NAME: usize = 15;
const MAX_KEYVALUE: usize = 6;
const MAX_SWITCH_COUNT: usize = 4;

pub async fn get_config_dir() -> PathBuf {
    match OS {
        "linux" => dirs::config_dir()
            .expect("Не могу найти папку .config")
            .join("Claws"),
        "windows" => dirs::document_dir()
            .expect("Не могу найти папку Документы")
            .join("Claws"),
        _ => panic!("Система не поддерживается: {}.", OS),
    }
}

pub async fn get_config_file() -> PathBuf {
    let config_file_path = get_config_dir().await;
    config_file_path.join("claws.toml")
}

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

#[derive(Debug, Serialize)]
struct Profile {
    name: String,
    buttons: [[u16; MAX_KEYVALUE]; MAX_SWITCH_COUNT],
    joystick_key_value: [u16; 4],
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            name: "".to_string(),
            buttons: [[0; MAX_KEYVALUE]; MAX_SWITCH_COUNT],
            joystick_key_value: [0; 4],
        }
    }
}

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
