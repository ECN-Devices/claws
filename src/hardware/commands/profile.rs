use anyhow::Result;
use log::debug;
use tokio::time::{Duration, Instant};

use crate::{
  errors::serial::KeypadError,
  hardware::{
    buffers::{Buffers, BuffersIO},
    commands::{DURATION, Value},
  },
};

/**
Команды для управления профилями устройства

Содержит команды для работы с профилями: запрос активного профиля,
установка имени, переключение между ОЗУ и ПЗУ, загрузка профилей.
*/
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
  LoadRamToActive(usize),

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
      Self::LoadRamToActive(num) => vec![15, *num as u8],
      Self::LoadFlashToRam => vec![16],
    }
  }
}

/**
Запрашивает номер активного профиля с устройства

Отправляет команду запроса и ожидает ответ с номером активного профиля
в течение 5 секунд.

# Аргументы
* `buffers` - Буферы для обмена данными с устройством

# Возвращает
Номер активного профиля (1-4) или ошибку при таймауте

# Ошибки
* `KeypadError::NoResponse` - если устройство не отвечает в течение 5 секунд
*/
pub async fn request_active_num(buffers: &mut Buffers) -> Result<u8> {
  let time = Instant::now();
  let duration = Duration::from_secs_f64(DURATION);

  buffers.send().push(Command::RequestActiveNum.get());

  loop {
    if time.elapsed() >= duration {
      break Err(KeypadError::NoResponse(Command::RequestActiveNum.get()).into());
    }

    match buffers
      .receive()
      .pull(&super::KeypadCommands::Profile(Command::RequestActiveNum))
    {
      Some(s) => {
        debug!("request_active_num: {s:?}");
        break Ok(*s.last().unwrap());
      }
      None => continue,
    };
  }
}

/**
Запрашивает имя активного профиля с устройства

Отправляет команду запроса и ожидает ответ с именем профиля
в течение 5 секунд.

# Аргументы
* `buffers` - Буферы для обмена данными с устройством

# Возвращает
Массив байтов с именем профиля (до 15 символов) или ошибку при таймауте

# Ошибки
* `KeypadError::NoResponse` - если устройство не отвечает в течение 5 секунд
*/
pub async fn request_name(buffers: &mut Buffers) -> Result<Vec<u8>> {
  let time = Instant::now();
  let duration = Duration::from_secs_f64(DURATION);

  buffers.send().push(Command::RequestName.get());

  loop {
    if time.elapsed() >= duration {
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
