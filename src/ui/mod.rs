//! Пользовательский интерфейс на базе Iced: состояние, сообщения и представления.

use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

use iced::Task;
use log::error;

use crate::{
  State,
  data::{device::Device, profiles::Profile, stick::Stick, window::Window},
  hardware::{
    buffers::Buffers,
    serial::{DeviceIO, Keypad, buttons::KeypadButton},
  },
  ui::{pages::Pages, update::Message},
};

pub mod pages;
pub mod styles;
pub mod subscription;
pub mod update;
pub mod view;

impl State {
  /**
  Создает новый экземпляр приложения с инициализацией
  # Возвращает
  Кортеж из:
  - Экземпляр приложения
  - Инициализационная задача
  */
  pub fn new() -> (Self, Task<Message>) {
    let port_name = match Keypad::get_port() {
      Ok(s) => s,
      Err(err) => {
        error!("{err}");
        String::new()
      }
    };

    let keypad = match !port_name.is_empty() {
      true => {
        let serial_port = Arc::new(Mutex::new(
          serialport::new(&port_name, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Ошибка открытия порта"),
        ));

        if cfg!(windows)
          && let Err(e) = serial_port.lock().unwrap().write_data_terminal_ready(true)
        {
          error!("Ошибка при установке DTR: {e}");
        }

        Keypad {
          is_open: true,
          port: Some(serial_port),
        }
      }
      false => Keypad {
        is_open: false,
        port: None,
      },
    };

    let profile =
      Task::done(Message::ProfileReceiveKeypadVec).chain(Task::done(Message::ProfilesListLoad));

    let device_info_task = match keypad.is_open {
      true => Task::done(Message::GetDeviceInfo),
      false => Task::none(),
    };

    let pages = match port_name.is_empty() {
      true => {
        if cfg!(debug_assertions) {
          Pages::default()
        } else {
          Pages::ConnectedDeviceNotFound
        }
      }
      false => Pages::default(),
    };

    (
      Self {
        active_profile_id: None,
        allow_write: false,
        buffers: Buffers::default(),
        button: KeypadButton::default(),
        device_info: Device::default(),
        is_first_start: true,
        is_rom: false,
        keypad,
        local_profile_id: None,
        pages,
        profile: Profile::default(),
        profile_on_keypad: true,
        profile_write: false,
        profiles_keypad_vec: Vec::with_capacity(4),
        profiles_local_vec: Vec::new(),
        request_active_profile_id: None,
        stick_callibrate: false,
        stick_callibrate_time: None,
        stick_info: Stick::default(),
        stick_show_calibrate_parameters: false,
        time_write: None,
        window_settings: Window::load(),
      },
      Task::batch(vec![profile, device_info_task]),
    )
  }
}
