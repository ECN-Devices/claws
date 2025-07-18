use crate::{
  App,
  data::ConfigWindow,
  hardware::{Keypad, communication_protocol::KeypadCommands},
};
use iced::{
  Element, Event, Subscription, Task, event,
  widget::{Container, column, row},
  window,
};
use log::{debug, error};
use pages::Pages;
use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

pub mod pages;

pub const WINDOW_WIDTH: f32 = 800.;
pub const WINDOW_HEIGH: f32 = 600.;

/** Определение возможных действий в приложении
 *
 * Эти сообщения определяют возможные действия и взаимодействия в приложении.
 */
#[derive(Debug, Clone)]
pub enum Message {
  ReadPort,
  WritePort(KeypadCommands),
  ChangePage(Pages),
  ButtonClicked,
  WindowResized(f32, f32),
  WindowMoved(f32, f32),
  WindowSettingsSave,
}

impl App {
  pub fn new() -> (Self, Task<Message>) {
    let port = Keypad::get_port();
    let keypad = match !port.is_empty() {
      true => {
        let serial_port = Arc::new(Mutex::new(
          serialport::new(&port, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port"),
        ));

        if cfg!(target_os = "windows") {
          if let Err(e) = serial_port.lock().unwrap().write_data_terminal_ready(true) {
            error!("Ошибка при установке DTR: {e}");
          }
        }

        Keypad {
          port: Some(serial_port),
          is_open: true,
        }
      }
      false => Keypad {
        port: None,
        is_open: false,
      },
    };

    let pages = match port.is_empty() {
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
        keypad,
        pages,
        window_settings: ConfigWindow::load(),
      },
      Task::none(),
    )
  }

  pub fn title(&self) -> String {
    String::from("Claws")
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::ReadPort => {
        if !self.keypad.is_open {
          return Task::none();
        }
        let mut port = self.keypad.port.clone().unwrap();
        Keypad::read_port(&mut port).unwrap();
        Task::none()
      }
      Message::WritePort(command) => {
        if !self.keypad.is_open {
          return Task::none();
        }
        let mut port = self.keypad.port.clone().unwrap();
        Keypad::write_port(&mut port, &command).unwrap();
        Task::none()
      }
      Message::ChangePage(page) => {
        self.pages = page;
        Task::none()
      }
      Message::ButtonClicked => {
        println!("click");
        Task::none()
      }
      Message::WindowResized(width, height) => {
        self.window_settings.width = width;
        self.window_settings.height = height;
        Task::none()
      }
      Message::WindowMoved(x, y) => {
        self.window_settings.x = x;
        self.window_settings.y = y;
        Task::none()
      }
      Message::WindowSettingsSave => {
        self.window_settings.save();
        Task::none()
      }
    }
  }

  pub fn view(&self) -> Element<Message> {
    let page = Pages::get_content(self);

    let content = match self.keypad.is_open {
      true => row![page],
      false => {
        #[cfg(debug_assertions)]
        {
          row![page]
        }

        #[cfg(not(debug_assertions))]
        {
          row![page]
        }
      }
    };

    Container::new(column![content]).into()
  }

  pub fn subscription(&self) -> Subscription<Message> {
    let port_subscription = match self.keypad.is_open {
      true => iced::time::every(Duration::from_millis(10)).map(|_| Message::ReadPort),
      false => Subscription::none(),
    };

    let window = event::listen_with(|event, _status, _id| match event {
      Event::Window(event) => match event {
        #[cfg(target_os = "windows")]
        window::Event::Moved(point) => {
          if cfg!(debug_assertions) {
            debug!("subscription: event: window: moved: {point:#?}");
          }
          Some(Message::WindowMoved(point.x, point.y))
        }
        window::Event::Resized(size) => {
          if cfg!(debug_assertions) {
            debug!("subscription: event: window: resized: {size:#?}");
          }
          Some(Message::WindowResized(size.width, size.height))
        }
        window::Event::Focused | window::Event::Unfocused => {
          if cfg!(debug_assertions) {
            debug!("subscription: event: window: focused: сохранение настроек положения окна");
          }
          Some(Message::WindowSettingsSave)
        }
        _ => None,
      },
      _ => None,
    });

    Subscription::batch(vec![
      window,
    ])
  }

  pub fn theme(&self) -> Theme {
    match dark_light::detect() {
      Ok(t) => match t {
        dark_light::Mode::Dark => Theme::Dark,
        dark_light::Mode::Light | dark_light::Mode::Unspecified => Theme::Light,
      },
      Err(_) => Theme::Light,
    }
  }
}
}
