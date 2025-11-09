//! Работа с последовательным портом и протоколом обмена с кейпадом.

use std::{
  sync::{Arc, Mutex},
  time::{Duration, Instant},
};

use anyhow::{Result, bail};
use log::{debug, error};
use serialport::SerialPort;

use crate::{
  errors::serial::KeypadError,
  hardware::{
    buffers::Buffers,
    commands::{KeypadCommands, empty},
  },
  utils::{BYTE_END, BYTE_START},
};

pub mod buttons;
pub mod profile;
pub mod stick;

/// Тип-обёртка для потокобезопасного доступа к `SerialPort`
type SerialIO = Arc<Mutex<Box<dyn SerialPort>>>;

/**
Содержит состояние подключения и последовательный порт.
Реализует основные операции для работы с устройством через последовательный порт.
*/
#[derive(Debug, Clone, Default)]
pub struct Keypad {
  /// Флаг открытого подключения
  pub is_open: bool,

  /// Дескриптор порта, если подключение установлено
  pub port: Option<SerialIO>,
}

/// Трейт для операций с последовательным портом
pub trait DeviceIO {
  /// Создает вектор портов с определенными параметрами
  fn get_port_vec() -> Result<Vec<String>>;

  /// Возвращает имя порта, к которому подключено устройство
  fn get_port() -> Result<String>;

  /**
  Читает данные из последовательного порта и складывает их в буферы
  # Аргументы
  * `port` - Ссылка на последовательный порт
  * `buffers` - Буферы обмена
  */
  fn receive(port: &mut SerialIO, buffers: &mut Buffers) -> Result<()>;

  /**
  Записывает команду из буфера отправки в последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт
  * `buffers` - Буферы обмена
  */
  fn send(port: &mut SerialIO, buffers: &mut Buffers) -> Result<()>;
}

impl DeviceIO for Keypad {
  fn get_port_vec() -> Result<Vec<String>> {
    let ports = serialport::available_ports().map_err(|_| KeypadError::NoPortsFound)?;
    let result = ports
      .into_iter()
      .filter_map(|port| {
        if let serialport::SerialPortType::UsbPort(usb_port_info) = port.port_type
          && (usb_port_info.vid == 11914 || usb_port_info.vid == 9114)
        {
          return Some(port.port_name);
        }
        None
      })
      .collect();

    Ok(result)
  }

  fn get_port() -> Result<String> {
    let time = Instant::now();
    let mut buffers = Buffers::default();
    let ports = Self::get_port_vec()?;
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

      if cfg!(windows)
        && let Err(e) = serial_port.lock().unwrap().write_data_terminal_ready(true)
      {
        error!("Ошибка при установке DTR: {e}");
      }

      buffers.send().push(&empty::Command::VoidRequest);
      Self::send(&mut serial_port, &mut buffers)?;

      loop {
        if time.elapsed() >= Duration::from_secs(5) {
          return Err(KeypadError::NoPortsFound.into());
        }

        match Self::receive(&mut serial_port, &mut buffers) {
          Ok(_) => (),
          Err(_) => continue,
        };

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
