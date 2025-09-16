/// Информация об устройстве, полученная по запросу через протокол
#[derive(Debug, Clone, Default)]
pub struct Device {
  /// Версия прошивки
  pub firmware_version: u16,
  /// Идентификатор модели/имя устройства
  pub name: u8,
  /// Количество кнопок на устройстве
  pub num_of_buttons: u8,
  /// Серийный номер
  pub serial_num: u16,
  /// Год выпуска
  pub year: u16,
}

impl Device {
  /// Парсит ответ устройства в структуру `Device`
  pub async fn parse(arr: &[u8]) -> Self {
    Self {
      firmware_version: u16::from_be_bytes([arr[7], arr[8]]),
      name: arr[1],
      num_of_buttons: arr[2],
      serial_num: u16::from_be_bytes([arr[3], arr[4]]),
      year: u16::from_be_bytes([arr[5], arr[6]]),
    }
  }
}
