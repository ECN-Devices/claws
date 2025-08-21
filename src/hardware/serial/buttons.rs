#[derive(Debug, Clone, Default)]
pub struct KeypadButton {
  pub id: usize,
  pub label: String,
  pub vec_str: Vec<String>,
  pub code: Vec<u8>,
}

impl KeypadButton {
  pub fn reduce_label(element: &str) -> String {
    match element {
      "Control" => "Ctrl".to_string(),
      "Delete" => "Del".to_string(),
      "Escape" => "Esc".to_string(),
      "ArrowUp" => "↑".to_string(),
      "ArrowRight" => "→".to_string(),
      "ArrowDown" => "↓".to_string(),
      "ArrowLeft" => "←".to_string(),
      _ => element.to_string(),
    }
  }
}
