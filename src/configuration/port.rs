use std::time::Duration;

use regex::Regex;
use tokio_serial::{available_ports, new, SerialPort};

use super::{command::command_to_string, ARRAY_LEN};

#[cfg(target_os = "linux")]
pub async fn get_keypad_port() -> String {
    let mut result = String::new();
    let mut vec_ports_str = Vec::new();
    let ports = available_ports().expect("No ports found!");
    let write_data_array: [u16; ARRAY_LEN] = [101, 0, 0, 0, 0, 0, 0, 0, 0];
    let response_data_array: [u16; ARRAY_LEN] = [101, 0, 0, 0, 0, 0, 0, 0, 1];
    let regex = Regex::new(r"/dev/ttyACM\d+").unwrap();

    for port in ports.into_iter() {
        let port_name = port.port_name;
        vec_ports_str.push(port_name);
    }
    let vec_ports_str: Vec<&str> = vec_ports_str.iter().map(|s| s.as_str()).collect();
    let filtered_ports: Vec<String> = vec_ports_str
        .into_iter()
        .filter(|port_str| regex.is_match(port_str))
        .map(|s| s.to_string())
        .collect();

    for port_name in filtered_ports.iter() {
        let port = match new(port_name, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(port) => port,
            Err(_err) => {
                // eprintln!("Ошибка открытия порта {}: {}", port_name, err);
                continue;
            }
        };

        match write_keypad_port(port.try_clone(), write_data_array).await {
            Ok(_) => {
                match read_keypad_port(port.try_clone()).await {
                    Ok(_) => {
                        result.push_str(port_name);
                    }
                    Err(_err) => {
                        // println!("Ошибка чтения из порта {}: {}", port_name, err);
                        continue;
                    }
                };
            }
            Err(_err) => {
                // eprintln!("Ошибка записи в порт {}: {}", port_name, err);
                continue;
            }
        };

        // println!("{:#?}", port_name)
    }
    // println!("{:#?}", result);
    result
}

#[cfg(target_os = "windows")]
pub async fn get_keypad_port() -> String {
    let mut result = String::new();
    let mut vec_ports_str: Vec<String> = Vec::new();
    let ports = available_ports().expect("No ports found!");
    let write_data_array: [u16; ARRAY_LEN] = [101, 0, 0, 0, 0, 0, 0, 0, 0];

    for port in ports.into_iter() {
        let port_name = port.port_name;
        vec_ports_str.push(port_name);
    }

    let filtered_ports: Vec<String> = vec_ports_str
        .into_iter()
        .filter(|port| !port.contains("COM1"))
        .collect();

    for port_name in filtered_ports.iter() {
        let port = match new(port_name, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(port) => port,
            Err(_err) => {
                // eprintln!("Ошибка открытия порта {}: {}", port_name, err);
                continue;
            }
        };

        match write_keypad_port(port.try_clone(), write_data_array).await {
            Ok(_) => {
                let _read_result = match read_keypad_port(port.try_clone()).await {
                    Ok(_) => result.push_str(port_name),
                    Err(_err) => {
                        // println!("Ошибка чтения из порта {}: {}", port_name, err);
                        continue;
                    }
                };
            }
            Err(_err) => {
                // eprintln!("Ошибка записи в порт {}: {}", port_name, err);
                continue;
            }
        };

        // println!("{:#?}", port_name)
    }
    // println!("{:#?}", result);
    result
}

pub async fn write_keypad_port(
    port: Result<Box<dyn SerialPort>, tokio_serial::Error>,
    write_data_array: [u16; ARRAY_LEN],
) -> Result<String, tokio_serial::Error> {
    let write_data = command_to_string(&write_data_array).await;

    port?
        .write_all(write_data.as_bytes())
        .expect("Write failed!");

    Ok(String::new())
}

pub async fn read_keypad_port(
    port: Result<Box<dyn SerialPort>, tokio_serial::Error>,
) -> Result<String, tokio_serial::Error> {
    let mut serial_buf = vec![0; 256];

    match port?.read(serial_buf.as_mut_slice()) {
        Ok(_m) => {
            let buf = String::from_utf8(serial_buf).unwrap();
            let trimmed_buf = buf.trim_end_matches(|c: char| c.is_control()).to_string();
            let ok_trimmed_buf = Ok(trimmed_buf);

            println!("{:?}", ok_trimmed_buf);
            ok_trimmed_buf
        }
        Err(err) => {
            // eprintln!("Ошибка чтения из порта: {}", err);
            Err(tokio_serial::Error::from(err))
        }
    }
}
