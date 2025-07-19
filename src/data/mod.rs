pub mod profiles;
pub mod window;

pub trait Config {
  fn save(&self);
}
