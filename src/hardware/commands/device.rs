use super::Value;

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
