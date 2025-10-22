/*!
Диалоги открытия/чтения профилей из файловой системы.

Этот модуль предоставляет функциональность для работы с файловыми диалогами,
позволяя пользователю выбирать и загружать профили из TOML-файлов.
*/

use std::path::Path;

use iced::Task;

use crate::{assets::APPLICATION_NAME, data::profiles::Profile, ui::Message};

impl Profile {
  /**
  Открывает асинхронный диалог выбора TOML-файла профиля и инициирует его загрузку

  Диалог открывается в директории конфигурации приложения и фильтрует
  только файлы с расширением .ron.

  # Возвращает
  Асинхронную задачу, которая при завершении отправит сообщение
  `Message::ProfileFileWrite` с загруженным профилем
  */
  pub fn open_load_file_dialog() -> Task<Message> {
    let file_path = confy::get_configuration_file_path(APPLICATION_NAME, None).unwrap();
    let dir_path = file_path.parent().unwrap().to_path_buf();
    Task::future(
      rfd::AsyncFileDialog::new()
        .add_filter("Config Formats", &["ron"])
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

  /**
  Вспомогательный метод: извлекает путь из результата диалога выбора файла

  # Аргументы
  * `handle` - Обработчик файла из диалога выбора

  # Возвращает
  Ссылку на путь выбранного файла
  */
  fn load_file_handle(handle: &rfd::FileHandle) -> &Path {
    handle.path()
  }
}
