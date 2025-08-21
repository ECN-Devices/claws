pub mod code;
pub mod file_dialog;
pub mod profiles;
pub mod window;

pub trait Config {
  fn save(&self);
}
