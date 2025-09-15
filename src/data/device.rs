#[derive(Debug, Clone, Default)]
pub struct Device {
  pub firmware_version: u16,
  pub name: u8,
  pub num_of_buttons: u8,
  pub serial_num: u16,
  pub year: u16,
}

impl Device {
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
