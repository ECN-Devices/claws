//! Диалоги открытия/чтения профилей из файловой системы.

use crate::{assets::APPLICATION_NAME, ui::Message};
use iced::Task;
use std::path::Path;

use super::profiles::Profile;

impl Profile {
  /// Открывает диалог выбора TOML-файла профиля и инициирует его загрузку
  pub fn open_load_file_dialog() -> Task<Message> {
    let file_path = confy::get_configuration_file_path(APPLICATION_NAME, None).unwrap();
    let dir_path = file_path.parent().unwrap().to_path_buf();
    Task::future(
      rfd::AsyncFileDialog::new()
        .add_filter("Config Formats", &["toml"])
        .set_directory(dir_path)
        .pick_file(),
    )
    .then(|handle| match handle {
      Some(ref handle) => {
        let profile = Profile::load_file(Self::load_file_handle(handle));
        Task::done(Message::ProfileFileWrite(profile))
      }
      None => Task::none(),
    })
  }

  /// Вспомогательный метод: извлекает путь из результата диалога
  fn load_file_handle(handle: &rfd::FileHandle) -> &Path {
    handle.path()
  }
}
