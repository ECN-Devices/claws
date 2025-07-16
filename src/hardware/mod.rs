use crate::utils::{BYTE_END, BYTE_START};
use communication_protocol::{CommandEmpty, KeypadCommands, Value};
use log::{debug, error};
use regex::Regex;
use serialport::SerialPort;
use std::{
  sync::{Arc, Mutex},
  thread::sleep,
  time::Duration,
  vec,
};

pub mod communication_protocol;

/** Структура `Keypad` представляет клавиатуру, подключенную к приложению.
 * Она содержит информацию о порте, к которому подключена клавиатура, и статус открытия порта.
 */
#[derive(Debug, Clone, Default)]
pub struct Keypad {
  pub port: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
  pub is_open: bool,
}

impl Keypad {
  #[cfg(target_os = "linux")]
  fn get_vec_ports() -> Vec<String> {
    let re = Regex::new(r"/dev/ttyACM\d+").unwrap();
    let mut ports_vec: Vec<String> = Vec::new();
    let ports = serialport::available_ports().expect("No ports found!");

    for port in ports {
      ports_vec.push(port.port_name);
    }

    ports_vec
      .into_iter()
      .filter(|port_name| re.is_match(port_name))
      .collect()
  }

  #[cfg(target_os = "windows")]
  fn get_vec_ports() -> Vec<String> {
    let mut ports_vec: Vec<String> = Vec::new();
    let ports = serialport::available_ports().expect("No ports found!");

    for port in ports {
      ports_vec.push(port.port_name);
    }

    ports_vec
      .into_iter()
      .filter(|port_name| !port_name.contains("COM1"))
      .collect()
  }

  fn gen_command(command: &KeypadCommands) -> Vec<u8> {
    let mut result = vec![BYTE_START];
    let command_len = command.get().len() as u8;
    result.push(command_len);
    for byte in command.get() {
      result.push(byte);
    }
    result.push(BYTE_END);
    result
  }

  pub fn write_port(
    port: &mut Arc<Mutex<Box<dyn SerialPort>>>,
    command: &KeypadCommands,
  ) -> Result<(), serialport::Error> {
    let mut port_lock = port.lock().unwrap();
    let write_data = Self::gen_command(command);

    if let Err(e) = port_lock.write_all(&write_data) {
      error!("Failed to write to port: {e}");
    };

    port_lock.flush().unwrap();

    if cfg!(debug_assertions) {
      debug!("write: {write_data:?}")
    }

    Ok(())
  }

  pub fn read_port(
    port: &mut Arc<Mutex<Box<dyn SerialPort>>>,
  ) -> Result<Vec<u8>, serialport::Error> {
    let mut data = vec![0; 1];
    let mut port_lock = port.lock().unwrap();

    // Проверяем, сколько байтов доступно для чтения
    let message = port_lock.bytes_to_read().unwrap();
    if message == 0 {
      return Ok(vec![]);
    }

    if port_lock.read_exact(&mut data).is_ok() {
      let byte = data[0];

      // Проверка начала сообщения
      if byte == BYTE_START {
        // Чтение длины пакета
        port_lock.read_exact(&mut data)?;
        let pack_len = data[0] as usize;

        // Чтение пакета данных
        let mut buf = vec![0u8; pack_len];
        port_lock.read_exact(&mut buf)?;

        // Проверка конца сообщения
        port_lock.read_exact(&mut data)?;
        if data[0] == BYTE_END {
          if cfg!(debug_assertions) {
            debug!("buf: {buf:?}")
          }
          return Ok(buf);
        }
      }
    }
    Ok(vec![])
  }

  pub fn get_port() -> String {
    let command = KeypadCommands::Empty(CommandEmpty::VoidRequest).get();
    let mut result = String::new();
    let ports = Self::get_vec_ports();
    for port in ports {
      let mut serial_port = match serialport::new(&port, 115_200)
        .timeout(Duration::from_millis(10))
        .open()
      {
        Ok(port) => Arc::new(Mutex::new(port)),
        Err(e) => {
          error!("Ошибка открытия порта {port}: {e}");
          continue;
        }
      };

      if cfg!(target_os = "windows") {
        if let Err(e) = serial_port.lock().unwrap().write_data_terminal_ready(true) {
          error!("Ошибка при установке DTR: {e}");
        }
      }

      Self::write_port(
        &mut serial_port,
        &KeypadCommands::Empty(CommandEmpty::VoidRequest),
      )
      .unwrap();

      sleep(Duration::from_millis(100));

      match Self::read_port(&mut serial_port) {
        Ok(data) => {
          if cfg!(debug_assertions) {
            debug!("port: {port}, data: {data:?}, command: {command:?}")
          };
          if data == command {
            result.push_str(port.as_str());
          }
        }
        Err(e) => error!("Ошибка чтения порта {port}: {e}"),
      };
    }
    result
  }
}
