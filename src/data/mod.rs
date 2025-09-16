pub mod code;
pub mod device;
pub mod file_dialog;
pub mod profiles;
pub mod window;

/// Трейт конфигурации для сущностей, которые могут сохранять своё состояние.
pub trait Config {
  /// Сохраняет текущее состояние конфигурации в постоянное хранилище.
  fn save(&self);
}
