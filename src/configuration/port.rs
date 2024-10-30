use std::{sync::Arc, time::Duration};

use regex::Regex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio_serial::{available_ports, new, SerialPortBuilderExt, SerialStream};

use super::{command::command_to_string, ARRAY_LEN};
use log::{debug, error};

async fn process_ports(ports: Vec<String>) -> String {
    let mut result = String::new();
    let write_data_array: [u16; ARRAY_LEN] = [101, 0, 0, 0, 0, 0, 0, 0, 0];
    for port_name in ports.iter() {
        debug!("Port: {}; bytes: {:?}", port_name, port_name.as_bytes());
        // Открываем порт
        let port = match new(port_name, 9600)
            .timeout(Duration::from_millis(10))
            .open_native_async()
        {
            Ok(port) => Arc::new(Mutex::new(port)), // Оборачиваем в Arc и AsyncMutex
            Err(err) => {
                error!("Ошибка открытия порта {}: {}", port_name, err);
                continue;
            }
        };

        // Пишем данные в порт
        match write_keypad_port(port.clone(), write_data_array).await {
            Ok(_) => {
                // Читаем данные из порта
                match read_keypad_port(port.clone()).await {
                    Ok(_) => result.push_str(port_name),
                    Err(err) => {
                        error!("Ошибка чтения из порта {}: {}", port_name, err);
                        continue;
                    }
                };
            }
            Err(err) => {
                error!("Ошибка записи в порт {}: {}", port_name, err);
                continue;
            }
        };
    }
    result
}

#[cfg(target_os = "linux")]
pub async fn get_keypad_port() -> String {
    let ports = available_ports().expect("No ports found!");
    let mut vec_ports_str = Vec::new();
    let regex = Regex::new(r"/dev/ttyACM\d+").unwrap();

    for port in ports.into_iter() {
        let port_name = port.port_name;
        //debug!("{}", port_name);
        vec_ports_str.push(port_name);
    }

    let filtered_ports: Vec<String> = vec_ports_str
        .into_iter()
        .filter(|port_name| regex.is_match(port_name))
        .collect();
    process_ports(filtered_ports).await
}

#[cfg(target_os = "windows")]
pub async fn get_keypad_port() -> String {
    let mut vec_ports_str: Vec<String> = Vec::new();
    let ports = available_ports().expect("No ports found!");

    for port in ports.into_iter() {
        let port_name = port.port_name;
        //debug!("{}", port_name);
        vec_ports_str.push(port_name);
    }

    let filtered_ports: Vec<String> = vec_ports_str
        .into_iter()
        .filter(|port| !port.contains("COM1"))
        .collect();
    process_ports(filtered_ports).await
}

pub async fn write_keypad_port(
    port: Arc<Mutex<SerialStream>>,
    write_data_array: [u16; ARRAY_LEN],
) -> Result<(), tokio_serial::Error> {
    let write_data = command_to_string(&write_data_array).await;
    let mut port_lock = port.lock().await; // Ожидаем получения блокировки

    if let Err(e) = port_lock.write_all(write_data.as_bytes()).await {
        error!("Failed to write to port: {}", e);
    }; // Используем await для асинхронного вызова

    debug!(
        "Write data: {}, bytes: {:?}",
        write_data,
        write_data.as_bytes()
    );

    Ok(())
}

pub async fn read_keypad_port(
    port: Arc<Mutex<SerialStream>>,
) -> Result<String, tokio_serial::Error> {
    let mut serial_buf = vec![0; 256];
    let mut port_lock = port.lock().await;

    match port_lock.read(serial_buf.as_mut_slice()).await {
        Ok(_m) => {
            let buf = String::from_utf8(serial_buf).unwrap();
            let trimmed_buf = buf.trim_end_matches(|c: char| c.is_control()).to_string();

            debug!("{:?}", trimmed_buf);
            Ok(trimmed_buf)
        }
        Err(err) => {
            // eprintln!("Ошибка чтения из порта: {}", err);
            Err(tokio_serial::Error::from(err))
        }
    }
}
