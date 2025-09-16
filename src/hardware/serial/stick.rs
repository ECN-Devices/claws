use serde::{Deserialize, Serialize};

/// Настройки стика: назначенные коды направлений и мёртвая зона
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Stick {
  /// Коды направлений: [Вверх, Вправо, Вниз, Влево]
  pub word: [u8; 4],

  /// Внутренняя (виртуальная) мёртвая зона, % от внешней
  pub deadzone: u8,
}
