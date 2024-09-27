use log::info;
use serde::Serialize;
use std::{env::consts::OS, fmt::Debug, path::PathBuf};
use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncWriteExt, BufWriter},
};

const MAX_PROFILE_NAME: usize = 15;
const MAX_KEYVALUE: usize = 6;
const MAX_SWITCH_COUNT: usize = 4;
pub mod port;

pub async fn get_config_dir() -> PathBuf {
    let config_dir = match OS {
        "linux" => dirs::document_dir()
            .expect("Не могу найти папку Документы")
            .join("Lapa"),
        "windows" => dirs::document_dir()
            .expect("Не могу найти папку Документы")
            .join("Lapa"),
        _ => panic!("Система не поддерживается: {}.", OS),
    };

    config_dir
}

// pub async fn get_config_dir() -> PathBuf {
//     let config_dir = match OS {
//         "linux" => dirs::config_dir()
//             .expect("Не могу найти папку .config")
//             .join("Lapa"),
//         "windows" => dirs::document_dir()
//             .expect("Не могу найти папку Документы")
//             .join("Lapa"),
//         _ => panic!("Система не поддерживается: {}.", OS),
//     };

//     config_dir
// }

pub async fn get_config_file() -> PathBuf {
    let config_file_path = get_config_dir().await;
    config_file_path.join("lapa.toml")
}

pub async fn create_config_dir() {
    let config_dir_path = get_config_dir().await;
    let _ = fs::create_dir_all(config_dir_path).await;
}

pub async fn create_config_file() {
    let config_file_path = get_config_file().await;
    let _ = File::create(config_file_path).await;
}

pub async fn check_config_file() {
    let config_dir_path = get_config_dir().await;
    let config_file_path = get_config_file().await;

    match config_dir_path.exists() {
        true => {
            info!("Конфигурационная папку уже сущевствует");
            match config_file_path.exists() {
                true => info!("Конфигурационный файл уже сущесвтует"),
                false => {
                    info!("Создаю конфигурационный файл");
                    create_config_file().await
                }
            }
        }
        false => {
            info!("Создаю конфигурационную папку");
            create_config_dir().await;
            info!("Создаю конфигурационный файл");
            create_config_file().await;
        }
    };
}

#[derive(Debug, Serialize)]
struct Profile {
    name: String,
    port: String,
    buttons: [[u16; MAX_KEYVALUE]; MAX_SWITCH_COUNT],
    joystick_key_value: [u16; 4],
}

// #[derive(Clone, Copy, Debug, Serialize)]
// struct Button {
//     btn: [u16; MAX_KEYVALUE],
// }

// #[derive(Serialize)]
// struct DPad {
//     up: char,
//     left: char,
//     right: char,
//     down: char,
// }

impl Default for Profile {
    fn default() -> Self {
        Profile {
            name: "".to_string(),
            port: "/dev/ttyACM0".to_string(),
            buttons: [[0; MAX_KEYVALUE]; MAX_SWITCH_COUNT],
            joystick_key_value: [0; 4],
        }
    }
}

// impl Default for Button {
//     fn default() -> Self {
//         Button {
//             btn: [0; MAX_KEYVALUE],
//         }
//     }
// }

pub async fn update_config_file(file_path: PathBuf) -> tokio::io::Result<()> {
    let config_toml = Profile {
        ..Default::default()
    };

    println!("{:#?}", config_toml);

    let toml = toml::to_string(&config_toml).unwrap();

    let config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .await?;
    let mut buffer = BufWriter::new(config_file);

    buffer.write_all(toml.as_bytes()).await?;
    buffer.flush().await?;
    Ok(())
}

// pub async fn update_config_file_2(file_path: PathBuf) -> tokio::io::Result<()> {
//     let config_toml = Profile {
//         name: todo!(),
//         switch_key_value: todo!(),
//         joystick_key_value: todo!(),
//     };

//     // println!("{:#?}", config_toml);

//     // let toml = toml::to_string(&config_toml).unwrap();

//     // let config_file = OpenOptions::new()
//     //     .read(true)
//     //     .write(true)
//     //     .create(true)
//     //     .open(file_path)
//     //     .await?;
//     // let mut buffer = BufWriter::new(config_file);

//     // buffer.write_all(toml.as_bytes()).await?;
//     // buffer.flush().await?;
//     Ok(())
// }

pub async fn command_to_string(array: &[u16; 9]) -> String {
    let mut command_string = String::new();
    const ARRAY_LEN: usize = 9;

    for (i, &value) in array.iter().take(ARRAY_LEN).enumerate() {
        command_string.push_str(&value.to_string());

        if i + 1 != ARRAY_LEN {
            command_string.push(',')
        } else {
            command_string.push(';')
        }
    }

    command_string
}
