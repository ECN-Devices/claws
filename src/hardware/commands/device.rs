use super::Value;
use crate::{
  errors::serial::KeypadError,
  hardware::buffers::{Buffers, BuffersIO},
};
use anyhow::Result;
use log::debug;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub enum Command {
  /**
  Запрос: 0x73, 0x1, 0x17, 0x65

  Ответ: 0x73, 0x9, 0x17, (название устройства),(количество доступных для настройки переключателей),(2 байта серийный номер),(2 байта год изготовления),(2 байта версия прошивки) , 0x65
  */
  RequestInfo,

  /**
  Запрос: 0x73, 0x5, 0x18, (2 байта серийный номер),(2 байта год изготовления) , 0x65

  Ответ: нет ответа
  */
  WriteInfo(u16, u16),
}

impl Value for Command {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestInfo => vec![17],
      Self::WriteInfo(serial_number, year) => {
        vec![
          18,
          (*serial_number >> 8) as u8,
          *serial_number as u8,
          (*year >> 8) as u8,
          *year as u8,
        ]
      }
    }
  }
}

pub fn request_info(buffers: &mut Buffers) -> Result<Vec<u8>> {
  let time = SystemTime::now();
  let duration = Duration::from_secs(5);

  buffers.send().push(Command::RequestInfo.get());

  loop {
    if time.elapsed()? >= duration {
      break Err(KeypadError::NoResponse(Command::RequestInfo.get()).into());
    }

    match buffers
      .receive()
      .pull(&super::KeypadCommands::Device(Command::RequestInfo))
    {
      Some(s) => {
        debug!("request_info: {s:?}");
        break Ok(s);
      }
      None => continue,
    };
  }
}
