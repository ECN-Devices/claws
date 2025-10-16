//! Ошибки взаимодействия с последовательным портом и протоколом кейпада.

/// Перечень возможных ошибок при работе с Serial и буферами
#[derive(Debug, thiserror::Error)]
pub enum KeypadError {
  #[error("Send buffer is empty")]
  BufferEmpty,

  #[error("Invalid packet format")]
  InvalidPacketFormat,

  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Mutex lock error: {0}")]
  LockError(String),

  #[error("No serial ports found")]
  NoPortsFound,

  #[error("No Response")]
  NoResponse(Vec<u8>),

  #[error("Serial port error: {0}")]
  SerialError(#[from] serialport::Error),
}
