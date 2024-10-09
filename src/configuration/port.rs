use std::time::Duration;

use tokio_serial::SerialPort;

use super::{command::command_to_string, ARRAY_LEN};

// pub fn get_keypad_port() {
//     // Порт Linux /dev/ttyACM0
//     // Порт Windows COM3
//     let ports = serialport::available_ports().expect("No ports found!");

//     let write_data_array: [u16; ARRAY_LEN] = [11, 0, 0, 0, 0, 0, 0, 0, 0];

//     for i in ports.iter() {
//         let port_name = &i.port_name;
//         let port = serialport::new(port_name, 115200)
//             .timeout(Duration::from_millis(10))
//             .open()
//             .expect("Failed to open port");

//         // let port_data = async {
//         //     write_keypad_port(port.try_clone(), write_data_array).await;
//         //     read_keypad_port(port.try_clone()).await
//         // };

//         println!("{:#?}", port)
//     }

//     // let port = serialport::new("/dev/ttyACM0", 115200)
//     //     .timeout(Duration::from_millis(10))
//     //     .open()
//     //     .expect("Failed to open port");

//     // "/dev/ttyACM0".to_string()
// }

pub async fn get_keypad_port() -> Vec<String> {
    let mut results = Vec::new();

    let ports = tokio_serial::available_ports().expect("No ports found!");
    let write_data_array: [u16; ARRAY_LEN] = [101, 0, 0, 0, 0, 0, 0, 0, 0];

    for port in ports.iter() {
        let port_name = &port.port_name;
        let port = match tokio_serial::new(port_name, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(port) => port,
            Err(err) => {
                eprintln!("Failed to open port {}: {}", port_name, err);
                continue;
            }
        };

        write_keypad_port(port.try_clone(), write_data_array).await;
        let read_result = read_keypad_port(port.try_clone()).await;

        results.push(format!("{}: {}", port_name, read_result));
    }

    println!("{:#?}", results);

    results
}

pub async fn write_keypad_port(
    port: Result<Box<dyn SerialPort>, tokio_serial::Error>,
    write_data_array: [u16; ARRAY_LEN],
) -> String {
    let write_data = command_to_string(&write_data_array).await;

    port.unwrap()
        .write_all(write_data.as_bytes())
        .expect("Write failed!");

    String::new()
}

pub async fn read_keypad_port(port: Result<Box<dyn SerialPort>, tokio_serial::Error>) -> String {
    let mut serial_buf = vec![0; 256];

    port.unwrap()
        .read(serial_buf.as_mut_slice())
        .expect("Found no data!");
    let buf = String::from_utf8(serial_buf).unwrap();
    let trimmed_buf = buf.trim_end_matches(|c: char| c.is_control()).to_string();

    println!("{:?}", trimmed_buf);

    trimmed_buf
}
