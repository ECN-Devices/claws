use super::buffers::Buffers;
use crate::{
  data::profiles::Profile,
  errors::serial::KeypadError,
  hardware::buffers::BuffersIO,
  utils::{BYTE_END, BYTE_START},
};
use log::{debug, error};
use serialport::SerialPort;
use std::{
  sync::{Arc, Mutex},
  vec,
};

pub mod profile;

type SerialIO = Arc<Mutex<Box<dyn SerialPort>>>;

/**
Содержит состояние подключения и последовательный порт.
Реализует основные операции для работы с устройством через последовательный порт.
*/
#[derive(Debug, Clone, Default)]
pub struct Keypad {
  pub is_open: bool,
  pub port: Option<SerialIO>,
}

/// Трейт для операций с последовательным портом
pub trait DeviceIO {
  /// Находит и возвращает имя порта, к которому подключено устройство
  fn get_port() -> Result<String, KeypadError>;

  fn processing_buf(buf: Result<Vec<u8>, serialport::Error>);

  /**
  Читает данные из последовательного порта
  # Аргументы
  * `port` - Ссылка на последовательный порт
  # Возвращает
  Прочитанные данные или ошибку последовательного порта
  */
  fn receive(port: &mut SerialIO) -> Result<Vec<u8>, KeypadError>;

  /**
  Записывает команду в последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт
  * `command` - Команда для отправки
  # Возвращает
  Результат операции или структуру Keypad с состоянием ошибки
  */
  fn send(port: &mut SerialIO, buffers: &mut Buffers) -> Result<(), KeypadError>;
}

impl DeviceIO for Keypad {
  fn get_port() -> Result<String, KeypadError> {
    let ports = serialport::available_ports().map_err(|_| KeypadError::NoPortsFound)?;
    for port in ports {
      match &port.port_type {
        serialport::SerialPortType::UsbPort(usb_port_info) => {
          if usb_port_info.vid == 11914 && usb_port_info.pid == 32778 {
            debug!("port name: {}", &port.port_name);
            return Ok(port.port_name);
          }
        }
        _ => continue,
      };
    }
    Err(KeypadError::NoPortsFound)
  }

  fn processing_buf(buf: Result<Vec<u8>, serialport::Error>) {
    let mut profile = Profile::default();
    match buf {
      Ok(buf) => match buf[0] {
        8 => {
          let button_data: [u8; 6] = buf[2..].try_into().unwrap();
          profile.buttons[buf[1] as usize] = button_data;
          // let button_data: [u8; 6] = buf.try_into().unwrap();
          // keypad_profile.buttons[button_num as usize - 1] = button_data
        }
        101 => (),
        _ => error!("Ошибка чтения"),
      },
      Err(e) => error!("erorr {e}"),
    }
    debug!("profile but {0:?}", profile.buttons)
  }

  fn receive(port: &mut SerialIO) -> Result<Vec<u8>, KeypadError> {
    let mut data = [0; 1];
    let mut port_lock = port
      .lock()
      .map_err(|e| KeypadError::LockError(e.to_string()))?;

    // Проверяем, сколько байтов доступно для чтения
    let message = port_lock.bytes_to_read()?;
    if message == 0 {
      return Ok(vec![]);
    }

    port_lock.read_exact(&mut data)?;

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
            debug!("buf: {buf:?}");
            return Ok(buf);
          }
        }
      }
    }

    Err(KeypadError::InvalidPacketFormat)
  }

  fn send(port: &mut SerialIO, buffers: &mut Buffers) -> Result<(), KeypadError> {
    let mut port_lock = port
      .lock()
      .map_err(|e| KeypadError::LockError(e.to_string()))?;
    let mut buf_lock = buffers.send();
    let buf_data = buf_lock.pull().ok_or(KeypadError::BufferEmpty)?;
    let buf_len = buf_lock.len();

    if buf_len > 0 {
      let mut buf = Vec::with_capacity(3 + buf_data.len());

      buf.extend(&[BYTE_START, buf_data.len() as u8]);
      buf.extend_from_slice(&buf_data);
      buf.push(BYTE_END);

      port_lock.write_all(&buf)?;
      port_lock.flush()?;

      debug!("write: {buf:?}");
    }
    Ok(())
  }
}
