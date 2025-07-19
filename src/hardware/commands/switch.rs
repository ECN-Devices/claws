use super::Value;

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
  RequestCondition(OptionsRequestCondition),

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
#[derive(Debug, Clone)]
pub enum OptionsRequestCondition {
  Pressed,
  NotPressed,
}

impl Value for Command {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestCondition(num) => vec![7, num.get()],
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
impl OptionsRequestCondition {
  pub fn get(&self) -> u8 {
    match self {
      OptionsRequestCondition::Pressed => 1,
      OptionsRequestCondition::NotPressed => 0,
    }
  }
}
