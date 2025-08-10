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
  Запрос: 0x73, 0x1, 0x101, 0x65

  Ответ: 0x73, 0x1, 0x101, 0x65
  */
  VoidRequest,
}

impl Value for Command {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::VoidRequest => vec![101],
    }
  }
}

pub fn empty(buffers: &mut Buffers) -> Result<()> {
  let time = SystemTime::now();
  let duration = Duration::from_secs(5);

  buffers.send().push(Command::VoidRequest.get());

  loop {
    if time.elapsed()? >= duration {
      break Err(KeypadError::NoResponse(Command::VoidRequest.get()).into());
    }

    match buffers
      .receive()
      .pull(&super::KeypadCommands::Empty(Command::VoidRequest))
    {
      Some(s) => {
        debug!("{s:?}");
        break Ok(());
      }
      None => continue,
    };
  }
}
