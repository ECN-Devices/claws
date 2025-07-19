use super::Value;

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
