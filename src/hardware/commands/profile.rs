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
  Запрос: 0x73, 0x1, 0x10, 0x65

  Ответ: 0x73, 0x2, 0x10, (номер профиля), 0x65
  */
  RequestActiveNum,

  /**
  Запрос: 0x73, 0x1, 0x11, 0x65

  Ответ: 0x73, 0x16, 0x11, (15 байт названия) , 0x65

  Название должно иметь длину 15 байт. Если название короче 15 символов, не использованные байты заполнятются нулями
  */
  RequestName,

  /**
  Запрос: 0x73, 0x16, 0x12, (15 байт названия) , 0x65

  Ответ: нет ответа

  Название должно иметь длину 15 байт. Если название короче 15 символов, не использованные байты заполняются нулями
  */
  SetName([u8; 15]),

  /**
  Запрос: 0x73, 0x2, 0x13, (номер профиля ОЗУ) , 0x65

  Ответ: нет ответа

  Кейпад имеет 4 профиля
  */
  WriteActiveToRam(u8),

  /**
  Запрос: 0x73, 0x2, 0x14, (номер профиля ПЗУ) , 0x65

  Ответ: нет ответа
  */
  WriteActiveToFlash(u8),

  /**
  Запрос: 0x73, 0x2, 0x15, (номер профиля ram) , 0x65

  Ответ: нет ответа

  Кейпад имеет 4 профиля
  */
  LoadRamToActive(u8),

  /**
  Запрос: 0x73, 0x1, 0x16, 0x65

  Загружает все профили из flash в ram. В следствии этого все профили в ram, а также активный профиль, будут перезаписаны.
  */
  LoadFlashToRam,
}

impl Value for Command {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestActiveNum => vec![10],
      Self::RequestName => vec![11],
      Self::SetName(profile_name) => [12].iter().chain(profile_name.iter()).copied().collect(),
      Self::WriteActiveToRam(num) => vec![13, *num],
      Self::WriteActiveToFlash(num) => vec![14, *num],
      Self::LoadRamToActive(num) => vec![15, *num],
      Self::LoadFlashToRam => vec![16],
    }
  }
}

pub fn request_name(buffers: &mut Buffers) -> Result<Vec<u8>> {
  let time = SystemTime::now();
  let duration = Duration::from_secs(5);

  buffers.send().push(Command::RequestName.get());

  loop {
    if time.elapsed()? >= duration {
      break Err(KeypadError::NoResponse(Command::RequestName.get()).into());
    }

    match buffers
      .receive()
      .pull(&super::KeypadCommands::Profile(Command::RequestName))
    {
      Some(s) => {
        debug!("request_name: {s:?}");
        break Ok(s[1..].to_vec());
      }
      None => continue,
    };
  }
}
