use serde::Serialize;
use std::{env::consts::OS, path::PathBuf};
use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncWriteExt, BufWriter},
};

#[derive(Serialize)]
struct Config {
    version: f32,
    button: Button,
}

#[derive(Serialize)]
struct Button {
    btn1: String,
    btn2: String,
    dpad: DPad,
}

#[derive(Serialize)]
struct DPad {
    up: char,
    left: char,
    right: char,
    down: char,
}

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

pub async fn update_config_file(file_path: PathBuf) -> tokio::io::Result<()> {
    let config_toml = Config {
        version: 1.0,
        button: Button {
            btn1: "Esc".to_string(),
            btn2: "W+2".to_string(),
            dpad: DPad {
                up: 'w',
                left: 'a',
                right: 'd',
                down: 's',
            },
        },
    };

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

pub async fn check_config_file() {}

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
