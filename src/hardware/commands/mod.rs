//! Набор команд протокола взаимодействия с устройством.

pub mod device;
pub mod empty;
pub mod profile;
pub mod stick;
pub mod switch;

const DURATION: f64 = 0.3;

/// Корневое перечисление всех команд протокола
#[derive(Debug, Clone)]
pub enum KeypadCommands {
  Device(device::Command),
  Empty(empty::Command),
  Profile(profile::Command),
  Stick(stick::Command),
  Switch(switch::Command),
}

/// Унифицированный интерфейс получения байтовой последовательности команды
pub trait Value {
  /// Возвращает полезную нагрузку команды (без обрамляющих байтов протокола)
  fn get(&self) -> Vec<u8>;
}

impl Value for KeypadCommands {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::Device(device) => device.get(),
      Self::Empty(empty) => empty.get(),
      Self::Profile(profile) => profile.get(),
      Self::Stick(stick) => stick.get(),
      Self::Switch(switch) => switch.get(),
    }
  }
}
