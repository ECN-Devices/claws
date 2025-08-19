use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Stick {
  pub word: [u8; 4],
  pub deadzone: u8,
}
