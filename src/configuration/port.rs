use std::time::Duration;

use super::{command_to_string, ARRAY_LEN};

pub fn get_keypad_port() -> String {
    // Порт Linux /dev/ttyACM0
    // Порт Windows COM3
    // let ports = serialport::available_ports().expect("No ports found!");

    // let port = serialport::new("/dev/ttyACM0", 115200)
    //     .timeout(Duration::from_millis(10))
    //     .open()
    //     .expect("Failed to open port");
    "/dev/ttyACM0".to_string()
}

pub async fn write_keypad_port(port: String, write_data_array: [u16; ARRAY_LEN]) {
    // let pirt = get_keypad_port().await;
    let write_data = command_to_string(&write_data_array).await;

    let mut command = serialport::new(&port, 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    command.write(write_data.as_bytes()).expect("Write failed!");

    read_keypad_port(port).await;
}

pub async fn read_keypad_port(port: String) {
    let mut serial_buf: Vec<u8> = vec![0; 20];

    let mut port = serialport::new(port, 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    port.read(serial_buf.as_mut_slice())
        .expect("Found no data!");

    let ints: Vec<u8> = serial_buf.into_iter().filter(|&b| b != 59).collect();
    let buf_string = String::from_utf8(ints).unwrap();
    let parts: Vec<&str> = buf_string.split(',').collect();
    let buf_u16 = parts
        .into_iter()
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    println!("{:?}", buf_u16);
}
