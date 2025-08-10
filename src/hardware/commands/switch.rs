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
  Запрос: 0x73, 0x2, 0x7, (номер переключателя), 0x65

  Ответ: 0x73, 0x3, 0x7, (код переключателя),(состояние переключателя), 0x65

  Переключатели нумеруются по порядку сверху вниз, слева на право

  Состояние переключателя:

      0 - не нажат
      1 - нажат

  Кейпад имеет 16 переключателей
  */
  RequestCondition(u8),

  /**
  Запрос: 0x73, 0x2, 0x8, (номер переключателя), 0x65

  Ответ: 0x73, 0x8, 0x8, (номер переключателя), (код 1), (код 2), (код 3),(код 4),(код 5),(код 6), 0x65

  Кейпад имеет 16 переключателей
  */
  RequestCodeASCII(u8),

  /**
  Запрос: 0x73, 0x8, 0x9, (номер переключателя), (код 1), (код 2), (код 3),(код 4),(код 5),(код 6), 0x65

  Ответ: нет ответа

  Кейпад имеет 16 переключателей
  */
  SetCodeASCII(u8, [u8; 6]),
}

impl Value for Command {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestCondition(num) => vec![7, *num],
      Self::RequestCodeASCII(num) => vec![8, *num],
      Self::SetCodeASCII(num, buttons_code) => {
        let mut result = vec![9, *num];
        for code in buttons_code {
          result.push(*code)
        }
        result
      }
    }
  }
}

pub fn request_condition(buffers: &mut Buffers) -> Result<()> {
  let time = SystemTime::now();
  let duration = Duration::from_secs(5);

  let switch_col = 16;
  (1..=switch_col).for_each(|i: u8| {
    buffers.send().push(Command::RequestCondition(i).get());
  });

  loop {
    if time.elapsed()? >= duration {
      break Err(KeypadError::NoResponse(Command::RequestCondition(1).get()).into());
    }

    (1..=switch_col).for_each(|i| {
      if let Some(s) = buffers
        .receive()
        .pull(&super::KeypadCommands::Swtich(Command::RequestCondition(i)))
      {
        debug!("request_condition: {s:?}");
      }
    });
  }
}

pub fn request_code_ascii(buffers: &mut Buffers) -> Result<[[u8; 6]; 16]> {
  let time = SystemTime::now();
  let duration = Duration::from_secs(5);

  let switch_col = 16;

  let mut switch_code = [[0u8; 6]; 16];
  let mut received = [false; 16];

  (1..=switch_col).for_each(|i: u8| {
    buffers.send().push(Command::RequestCodeASCII(i).get());
  });

  loop {
    if time.elapsed()? >= duration {
      return Err(KeypadError::NoResponse(Command::RequestCodeASCII(1).get()).into());
    }

    (1..=switch_col).for_each(|i| {
      if let Some(s) = buffers
        .receive()
        .pull(&super::KeypadCommands::Swtich(Command::RequestCodeASCII(i)))
      {
        debug!("request_code_ascii: {s:?}");
        let switch_num = s[1] as usize;
        switch_code[switch_num - 1].copy_from_slice(&s[2..]);
        received[switch_num - 1] = true;
      }
    });

    if received.iter().all(|&r| r) {
      break Ok(switch_code);
    }
  }
}
