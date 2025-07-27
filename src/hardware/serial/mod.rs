use super::commands::{KeypadCommands, Value, empty};
use crate::{
  data::profiles::Profile,
  utils::{BYTE_END, BYTE_START},
};
use log::{debug, error};
use serialport::SerialPort;
use std::{
  sync::{Arc, Mutex},
  thread::sleep,
  time::Duration,
  vec,
};

/**
Содержит состояние подключения и последовательный порт.
Реализует основные операции для работы с устройством через последовательный порт.
*/
#[derive(Debug, Clone, Default)]
pub struct Keypad {
  pub is_open: bool,
  pub port: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
}

/// Трейт для операций с последовательным портом
pub trait DeviceIO {
  /// Находит и возвращает имя порта, к которому подключено устройство
  fn get_port() -> String;

  fn processing_buf(buf: Result<Vec<u8>, serialport::Error>);

  /**
  Читает данные из последовательного порта
  # Аргументы
  * `port` - Ссылка на последовательный порт
  # Возвращает
  Прочитанные данные или ошибку последовательного порта
  */
  fn receive(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) -> Result<Vec<u8>, serialport::Error>;

  /**
  Записывает команду в последовательный порт
  # Аргументы
  * `port` - Ссылка на последовательный порт
  * `command` - Команда для отправки
  # Возвращает
  Результат операции или структуру Keypad с состоянием ошибки
  */
  fn send(
    port: &mut Arc<Mutex<Box<dyn SerialPort>>>,
    command: &KeypadCommands,
  ) -> Result<(), Keypad>;
}
impl DeviceIO for Keypad {
  fn get_port() -> String {
    let ports = serialport::available_ports().expect("No ports found!");
    let command = KeypadCommands::Empty(empty::Command::VoidRequest).get();

    for port in ports {
      match &port.port_type {
        serialport::SerialPortType::UsbPort(usb_port_info) => {
          if usb_port_info.vid == 11914 {
            let port = port.port_name;
            debug!("port name: {port}");
            let mut serial_port = match serialport::new(&port, 115_200)
              .timeout(Duration::from_millis(10))
              .open()
            {
              Ok(port) => Arc::new(Mutex::new(port)),
              Err(_) => {
                continue;
              }
            };

            if cfg!(windows) {
              if let Err(e) = serial_port.lock().unwrap().write_data_terminal_ready(true) {
                error!("Ошибка при установке DTR: {e}");
              }
            }

            Self::send(
              &mut serial_port,
              &KeypadCommands::Empty(empty::Command::VoidRequest),
            )
            .unwrap();

            sleep(Duration::from_millis(10));

            match Self::receive(&mut serial_port) {
              Ok(data) => {
                debug!("port: {port}, data: {data:?}, command: {command:?}");
                if data == command {
                  return port;
                }
              }
              Err(e) => error!("Ошибка чтения порта {port}: {e}"),
            };
          }
        }
        _ => continue,
      };
    }
    String::new()
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

  fn receive(port: &mut Arc<Mutex<Box<dyn SerialPort>>>) -> Result<Vec<u8>, serialport::Error> {
    let mut data = [0; 1];
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
      //         debug!("buf: {buf:?}");

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
              debug!("buf: {buf:?}");
              return Ok(buf);
            }
          }
        }
      }
    }
    Ok(vec![])
  }

  fn send(
    port: &mut Arc<Mutex<Box<dyn SerialPort>>>,
    command: &KeypadCommands,
  ) -> Result<(), Keypad> {
    let mut port_lock = port.lock().unwrap();
    let buf = Self::generate_command(command);

    if let Err(e) = port_lock.write_all(&buf) {
      error!("Ошибка записив порт {:?}: {e}", port_lock.name());

      return Err(Keypad {
        port: None,
        is_open: false,
      });
    };

    port_lock.flush().unwrap();

    debug!("write: {buf:?}");

    Ok(())
  }
}

/// Трейт для работы с протоколом обмена данными
pub trait ProtocolHandler {
  /**
  Генерирует байтовую последовательность команды согласно протоколу
  # Аргументы
  * `command` - Команда для преобразования
  # Возвращает
  Вектор байтов, готовый для отправки через последовательный порт
  */
  fn generate_command(command: &KeypadCommands) -> Vec<u8>;
}
impl ProtocolHandler for Keypad {
  fn generate_command(command: &KeypadCommands) -> Vec<u8> {
    let command = command.get();
    let mut result = Vec::with_capacity(3 + command.len());
    result.extend(&[BYTE_START, command.len() as u8]);
    result.extend_from_slice(&command);
    result.push(BYTE_END);
    result
  }
}
