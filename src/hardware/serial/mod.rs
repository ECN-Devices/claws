use super::buffers::Buffers;
use crate::{
  errors::serial::KeypadError,
  hardware::{
    buffers::BuffersIO,
    commands::{KeypadCommands, Value, empty},
  },
  utils::{BYTE_END, BYTE_START},
};
use anyhow::{Result, bail};
use log::{debug, error};
use serialport::SerialPort;
use std::{
  sync::{Arc, Mutex},
  time::{Duration, SystemTime},
  vec,
};

pub mod buttons;
pub mod profile;
pub mod stick;

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
  fn get_port() -> Result<String>;

  /**
  Читает данные из последовательного порта
  # Аргументы
  * `port` - Ссылка на последовательный порт
  # Возвращает
  Прочитанные данные или ошибку последовательного порта
  */
  fn receive(port: &mut SerialIO, buffers: &mut Buffers) -> Result<()>;

  /**
  Записывает команду в последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт
  * `command` - Команда для отправки
  # Возвращает
  Результат операции или структуру Keypad с состоянием ошибки
  */
  fn send(port: &mut SerialIO, buffers: &mut Buffers) -> Result<()>;
}

impl DeviceIO for Keypad {
  fn get_port() -> Result<String> {
    let time = SystemTime::now();
    let mut buffers = Buffers::default();
    let ports = serialport::available_ports().map_err(|_| KeypadError::NoPortsFound)?;
    for port in ports {
      match &port.port_type {
        serialport::SerialPortType::UsbPort(usb_port_info) => {
          if usb_port_info.vid == 11914 && usb_port_info.pid == 32778
            || usb_port_info.vid == 9114 && usb_port_info.pid == 33012
          {
            let port = port.port_name;

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

            if cfg!(windows)
              && let Err(e) = serial_port.lock().unwrap().write_data_terminal_ready(true)
            {
              error!("Ошибка при установке DTR: {e}");
            }

            buffers.send().push(empty::Command::VoidRequest.get());
            Self::send(&mut serial_port, &mut buffers)?;

            loop {
              if time.elapsed()? >= Duration::from_secs(5) {
                return Ok("".to_string());
              }

              Self::receive(&mut serial_port, &mut buffers)?;
              match buffers
                .receive()
                .pull(&KeypadCommands::Empty(empty::Command::VoidRequest))
              {
                Some(_) => {
                  debug!("port name: {}", &port);
                  return Ok(port);
                }

                None => continue,
              };
            }
          }
        }
        _ => continue,
      };
    }
    bail!(KeypadError::NoPortsFound)
  }

  fn receive(port: &mut SerialIO, buffers: &mut Buffers) -> Result<()> {
    let mut data = [0; 1];
    let mut port_lock = port
      .lock()
      .map_err(|e| KeypadError::LockError(e.to_string()))?;

    // Проверяем, сколько байтов доступно для чтения
    let message = port_lock.bytes_to_read()?;
    if message == 0 {
      return Ok(());
    }

    port_lock.read_exact(&mut data)?;

    if let [start, ..] = data.as_slice()
      && *start == BYTE_START
    {
      // Чтение длины пакета
      port_lock.read_exact(&mut data)?;
      let pack_len = data[0] as usize;

      // Чтение пакета данных
      let mut buf = vec![0u8; pack_len];
      port_lock.read_exact(&mut buf)?;

      // Проверка конца сообщения
      port_lock.read_exact(&mut data)?;
      if let [end, ..] = data.as_slice()
        && *end == BYTE_END
      {
        // debug!("receive: {buf:?}");
        buffers.receive().push(buf);
        return Ok(());
      }
    }

    bail!(KeypadError::InvalidPacketFormat)
  }

  fn send(port: &mut SerialIO, buffers: &mut Buffers) -> Result<()> {
    let mut port_lock = port
      .lock()
      .map_err(|e| KeypadError::LockError(e.to_string()))?;
    let mut buf_lock = buffers.send();
    let buf_data = buf_lock.pull().ok_or(KeypadError::BufferEmpty)?;

    let mut buf = Vec::with_capacity(3 + buf_data.len());

    buf.extend(&[BYTE_START, buf_data.len() as u8]);
    buf.extend_from_slice(&buf_data);
    buf.push(BYTE_END);

    port_lock.write_all(&buf)?;
    port_lock.flush()?;

    // debug!("write: {buf:?}");
    Ok(())
  }
}
