use crate::{assets::APPLICATION_NAME, ui::Message};
use iced::Task;
use std::path::PathBuf;

use super::profiles::Profile;

pub trait FileDialog {
  fn open_load_file_dialog() -> Task<Message>;
  fn load_file_handle(handle: rfd::FileHandle) -> PathBuf;
}
impl FileDialog for Profile {
  fn open_load_file_dialog() -> Task<Message> {
    let file_path = confy::get_configuration_file_path(APPLICATION_NAME, None).unwrap();
    let dir_path = file_path.parent().unwrap().to_path_buf();
    Task::future(
      rfd::AsyncFileDialog::new()
        .add_filter("Config Formats", &["toml"])
        .set_directory(dir_path)
        .pick_file(),
    )
    .then(|handle| match handle {
      Some(handle) => {
        let profile = Profile::load_file(Self::load_file_handle(handle));
        Task::done(Message::ProfileFileWrite(profile))
      }
      None => Task::none(),
    })
  }
  fn load_file_handle(handle: rfd::FileHandle) -> PathBuf {
    handle.path().to_path_buf()
  }
}
