use crate::{assets::APPLICATION_NAME, ui::Message};
use iced::Task;

pub mod profiles;
pub mod window;

pub trait Config {
  fn save(&self);
}

pub fn open_file_dialog() -> Task<Message> {
  let file_path = confy::get_configuration_file_path(APPLICATION_NAME, None).unwrap();
  let dir_path = file_path.parent().unwrap().to_path_buf();
  Task::future(
    rfd::AsyncFileDialog::new()
      .add_filter(
        // <-- (OPTIONAL) Add a filter to only allow PNG and JPEG formats.
        "Config Formats",
        &["toml"],
      )
      .set_directory(dir_path)
      .pick_file(), // <-- Launch the dialog window.
  )
  .then(|handle| match handle {
    // After obtaining a file handle from the dialog, we load the image.
    //
    // We use Task::perform to run load_image, as this may take a while to load.
    Some(_) => Task::none(),

    // The user has cancelled the operation, so we return a "Cancelled" message.
    None => Task::none(),
  })
}
