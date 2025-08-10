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
  Запрос: 0x73, 0x1, 0x1, 0x65

  Ответ: 0x73, 0x5, 0x1, (2 байта координаты x), (2 байта координаты y) 0x65
  */
  RequestPositionXY,

  /**
  Запрос: 0x73, 0x1, 0x3, 0x65

  Ответ: 0x73, 0x5, 0x3, (код "Вверх"), (код "Вправо"), (код "Вниз"), (код "Влево"), 0x65
  */
  RequestPositionASCII,

  /**
  Запрос: 0x73, 0x2 или 0x3, 0x4, (Код параметра), (1-2 байта значения параметра), 0x65

  Ответ: нет ответа
  Коды параметров:

    1 - центр оси x (Значение АЦП) (2 байта) (Не рекомендуется менять вручную)
    2 - центр оси y (Значение АЦП) (2 байта) (Не рекомендуется менять вручную)
    3 - радиус физической мертвой зоны (Значение АЦП) (2 байта) (Не рекомендуется менять вручную)
    4 - виртуальная внутренняя мертвая зона (Процент от физической мертвой зоны от 1 до 100) (1 байт)
  */
  SetParameters(u8),

  /**
  Запрос: 0x73, 0x3, 0x5, (код положения стика), (ascii код), 0x65

  Ответ: нет ответа

  Коды положения стика:

      1 - вверх
      2 - вправо
      3 - вниз
      4 - влево
  */
  SetPositionASCII(u8, u8),

  /**
  Запрос: 0x73, 0x2, 0x6, (код опции), 0x65

  Ответ опции 1: 0x73, 0x9, 0x6, (код опции), (2 байта центр x), (2 байта центр y), (2 байта внешняя мёртвая зона), (внутренняя мёртвая зона)

  Ответ опции 2: нет ответа

  Коды опций:

      1 - запрос параметров стика
      2 - автоматическая калибровка параметров стика

  **WARN**: Внешняя мертвая зона (или физическая мертвая зона) измеряется в единицах АЦП от центра до крайнего положения стика. Внутренняя мертвая зона (или виртуальная мертвая зона) измеряется в процентах от внешней мертвой зоны, не изменяется при калибровке, задается пользователем.
  */
  Calibration(OptionsCalibration),
}
#[derive(Debug, Clone)]
pub enum OptionsCalibration {
  Request,
  Calibrate,
}

impl Value for Command {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestPositionXY => vec![1],
      Self::RequestPositionASCII => vec![3],
      Self::SetParameters(percent) => vec![4, *percent],
      Self::SetPositionASCII(position, ascii_code) => vec![5, *position, *ascii_code],
      Self::Calibration(option) => vec![6, option.get()],
    }
  }
}
impl OptionsCalibration {
  pub fn get(&self) -> u8 {
    match self {
      Self::Request => 1,
      Self::Calibrate => 2,
    }
  }
}

pub fn request_position_ascii(buffers: &mut Buffers) -> Result<[u8; 4]> {
  let time = SystemTime::now();
  let duration = Duration::from_secs(5);

  let mut stick_code = [0u8; 4];

  buffers.send().push(Command::RequestPositionASCII.get());

  loop {
    if time.elapsed()? >= duration {
      break Err(KeypadError::NoResponse(Command::RequestPositionASCII.get()).into());
    }

    match buffers
      .receive()
      .pull(&super::KeypadCommands::Stick(Command::RequestPositionASCII))
    {
      Some(s) => {
        debug!("request_position_ascii: {s:?}");
        stick_code.copy_from_slice(&s[1..]);
        break Ok(stick_code);
      }
      None => continue,
    };
  }
}
