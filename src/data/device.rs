/**
Информация об устройстве, полученная по запросу через протокол

Содержит основные характеристики подключенного устройства,
включая версию прошивки, модель и серийные данные.
*/
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
  /**
  Парсит массив байтов ответа устройства в структуру `Device`

  # Аргументы
  * `arr` - Массив байтов от устройства с информацией об устройстве

  # Возвращает
  Структуру `Device` с распарсенными данными об устройстве

  # Формат данных
  Ожидается массив длиной минимум 9 байт:
  - arr[1]: name (идентификатор модели)
  - arr[2]: num_of_buttons (количество кнопок)
  - arr[3-4]: serial_num (серийный номер, big-endian)
  - arr[5-6]: year (год выпуска, big-endian)
  - arr[7-8]: firmware_version (версия прошивки, big-endian)
  */
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
