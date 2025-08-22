#[derive(Debug, Clone, Default)]
pub struct KeypadButton {
  pub id: usize,
  pub label: String,
  pub vec_str: Vec<String>,
  pub code: Vec<u8>,
  pub is_stick: bool,
}
