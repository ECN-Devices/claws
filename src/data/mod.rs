pub mod code;
pub mod device;
pub mod file_dialog;
pub mod profiles;
pub mod window;

pub trait Config {
  fn save(&self);
}
