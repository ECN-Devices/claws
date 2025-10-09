/**
Параметры калибровки стика, полученные от устройства

Содержит информацию о центре стика и мертвых зонах,
необходимую для корректной работы аналогового стика.
*/
#[derive(Debug, Clone, Default)]
pub struct Stick {
  /// Координата X центра стика в единицах АЦП
  pub center_x: u16,

  /// Координата Y центра стика в единицах АЦП
  pub center_y: u16,

  /// Внешняя (физическая) мертвая зона в единицах АЦП
  /// Определяет радиус от центра до крайнего положения стика
  pub external_deadzone: u16,

  /// Внутренняя (виртуальная) мертвая зона в процентах
  /// от внешней мертвой зоны (1-100%)
  pub internal_deadzone: u8,
}

impl Stick {
  /**
  Парсит массив байтов ответа устройства в структуру параметров стика

  # Аргументы
  * `arr` - Массив байтов от устройства с параметрами калибровки

  # Возвращает
  Структуру `Stick` с распарсенными параметрами калибровки

  # Формат данных
  Ожидается массив длиной минимум 9 байт:
  - arr[2-3]: center_x (big-endian)
  - arr[4-5]: center_y (big-endian)
  - arr[6-7]: external_deadzone (big-endian)
  - arr[8]: internal_deadzone
  */
  pub async fn parse(arr: &[u8]) -> Self {
    Self {
      center_x: u16::from_be_bytes([arr[2], arr[3]]),
      center_y: u16::from_be_bytes([arr[4], arr[5]]),
      external_deadzone: u16::from_be_bytes([arr[6], arr[7]]),
      internal_deadzone: arr[8],
    }
  }
}
