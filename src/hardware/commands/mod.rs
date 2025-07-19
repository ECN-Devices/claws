pub mod device;
pub mod empty;
pub mod profile;
pub mod stick;
pub mod switch;

#[derive(Debug, Clone)]
pub enum KeypadCommands {
  Device(device::Command),
  Empty(empty::Command),
  Profile(profile::Command),
  Stick(stick::Command),
  Swtich(switch::Command),
}

pub trait Value {
  fn get(&self) -> Vec<u8>;
}

impl Value for KeypadCommands {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::Device(device) => device.get(),
      Self::Empty(empty) => empty.get(),
      Self::Profile(profile) => profile.get(),
      Self::Stick(stick) => stick.get(),
      Self::Swtich(switch) => switch.get(),
    }
  }
}
