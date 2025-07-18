use crate::utils::{BYTE_END, BYTE_START};
use communication_protocol::{KeypadCommands, Value};
use log::{debug, error};
use serialport::SerialPort;
use std::{
  sync::{Arc, Mutex},
  vec,
};

pub mod communication_protocol;

/** Структура `Keypad` представляет клавиатуру, подключенную к приложению.
 * Она содержит информацию о порте, к которому подключен кейпад, и статус открытия порта.
 */
#[derive(Debug, Clone, Default)]
pub struct Keypad {
  pub port: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
  pub is_open: bool,
}

impl Keypad {
  pub fn get_port() -> String {
    let ports = serialport::available_ports().expect("No ports found!");

    for port in ports {
      let port_vid = match port.port_type {
        serialport::SerialPortType::UsbPort(usb_port_info) => usb_port_info.vid,
        serialport::SerialPortType::PciPort => continue,
        serialport::SerialPortType::BluetoothPort => continue,
        serialport::SerialPortType::Unknown => continue,
      };

      if port_vid == 11914 {
        if cfg!(debug_assertions) {
          debug!("port name: {}", port.port_name);
        }
        return port.port_name;
      }
    }
    "".to_string()
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
  ) -> Result<(), Keypad> {
    let mut port_lock = port.lock().unwrap();
    let write_data = Self::gen_command(command);

    if let Err(e) = port_lock.write_all(&write_data) {
      error!("Ошибка записив порт {:?}: {e}", port_lock.name());

      return Err(Keypad {
        port: None,
        is_open: false,
      });
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
    let message = match port_lock.bytes_to_read() {
      Ok(s) => s,
      Err(e) => {
        error!("Нет доступных байтов для чтения; Ошибка: {e}");
        return Ok(vec![]);
      }
    };
    if message == 0 {
      return Ok(vec![]);
    }

    if port_lock.read_exact(&mut data).is_ok() {
      //   let byte = data[0];

      //   // Проверка начала сообщения
      //   if byte == BYTE_START {
      //     // Чтение длины пакета
      //     port_lock.read_exact(&mut data)?;
      //     let pack_len = data[0] as usize;

      //     // Чтение пакета данных
      //     let mut buf = vec![0u8; pack_len];
      //     port_lock.read_exact(&mut buf)?;

      //     // Проверка конца сообщения
      //     port_lock.read_exact(&mut data)?;
      //     if data[0] == BYTE_END {
      //       if cfg!(debug_assertions) {
      //         debug!("buf: {buf:?}")
      //       }

      //       return Ok(buf);
      //     }
      //   }
      // }

      if let [start, ..] = data.as_slice() {
        if *start == BYTE_START {
          // Чтение длины пакета
          port_lock.read_exact(&mut data)?;
          let pack_len = data[0] as usize;

          // Чтение пакета данных
          let mut buf = vec![0u8; pack_len];
          port_lock.read_exact(&mut buf)?;

          // Проверка конца сообщения
          port_lock.read_exact(&mut data)?;
          if let [end, ..] = data.as_slice() {
            if *end == BYTE_END {
              if cfg!(debug_assertions) {
                debug!("buf: {buf:?}")
              }
              return Ok(buf);
            }
          }
        }
      }
    }
    Ok(vec![])
  }
}
