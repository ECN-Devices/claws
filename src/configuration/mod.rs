use std::{env::consts::OS, path::PathBuf};
use tokio::fs::{self, File};

pub async fn get_config_dir() -> PathBuf {
    let config_dir = match OS {
        "linux" => dirs::config_dir()
            .expect("Не могу найти папку .config")
            .join("Lapa"),
        "windows" => dirs::document_dir()
            .expect("Не могу найти папку Документы")
            .join("Lapa"),
        _ => panic!("Система не поддерживается: {}.", OS),
    };

    config_dir
}

pub async fn get_config_file() -> PathBuf {
    let config_file_path = get_config_dir().await;
    config_file_path.join("lapa.toml")
}

pub async fn create_config_dir() {
    let config_dir_path = get_config_dir().await;

    match config_dir_path.exists() {
        true => println!("Директория конфигурации уже существует."),
        false => {
            let _ = fs::create_dir_all(config_dir_path).await;
            println!("Директория конфигурации создана.")
        }
    }
}

pub async fn create_config_file() {
    let config_file_path = get_config_file().await;

    match config_file_path.exists() {
        true => println!("Файл конфигурации уже существует."),
        false => {
            let _ = File::create(config_file_path).await;
            println!("Файл конфигурации создан.");
        }
    }
}

// fn get_config_file_path() -> String {
//     #[cfg(debug_assertions)]
//     return format!("{}/configurations_debug.json", get_config_dir());

//     #[cfg(not(debug_assertions))]
//     return format!("{}/configurations.json", get_config_dir());
// }

pub async fn command_to_string(array: &[i16; 9], len: usize) -> String {
    let mut command_string = String::new();

    for (i, &value) in array.iter().take(len).enumerate() {
        command_string.push_str(&value.to_string());

        if i + 1 != len {
            command_string.push(',')
        } else {
            command_string.push(';')
        }
    }

    command_string
}
