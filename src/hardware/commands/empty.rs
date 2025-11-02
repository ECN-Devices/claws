use crate::hardware::commands::Value;

/**
Пустые команды для проверки связи с устройством

Содержит команды, которые не изменяют состояние устройства,
но позволяют проверить его доступность и работоспособность.
*/
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

/**
Отправляет пустую команду для проверки связи с устройством

Используется для проверки доступности устройства и тестирования
связи. Ожидает эхо-ответ в течение 5 секунд.

# Аргументы
* `buffers` - Буферы для обмена данными с устройством

# Возвращает
`Ok(())` при успешном получении эхо-ответа или ошибку при таймауте

# Ошибки
* `KeypadError::NoResponse` - если устройство не отвечает в течение 5 секунд
*/
#[cfg(false)]
pub fn empty(buffers: &mut Buffers) -> Result<()> {
  let time = Instant::now();
  let duration = Duration::from_secs_f64(DURATION);

  buffers.send().push(Command::VoidRequest.get());

  loop {
    if time.elapsed() >= duration {
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
