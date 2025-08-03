#[derive(Debug, thiserror::Error)]
pub enum KeypadError {
  #[error("No serial ports found")]
  NoPortsFound,

  #[error("Send buffer is empty")]
  BufferEmpty,

  #[error("Invalid packet format")]
  InvalidPacketFormat,

  #[error("Serial port error: {0}")]
  SerialError(#[from] serialport::Error),

  #[error("Mutex lock error: {0}")]
  LockError(String),

  #[error("No Response")]
  NoResponse(Vec<u8>),

  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),
}
