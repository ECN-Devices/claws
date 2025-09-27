/// Информация об устройстве, полученная по запросу через протокол
#[derive(Debug, Clone, Default)]
pub struct Stick {
  pub center_x: u16,
  pub center_y: u16,
  pub external_deadzone: u16,
  pub internal_deadzone: u8,
}

impl Stick {
  pub async fn parse(arr: &[u8]) -> Self {
    Self {
      center_x: u16::from_be_bytes([arr[2], arr[3]]),
      center_y: u16::from_be_bytes([arr[4], arr[5]]),
      external_deadzone: u16::from_be_bytes([arr[6], arr[7]]),
      internal_deadzone: arr[8],
    }
  }
}
