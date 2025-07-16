use crate::{
  App,
  data::WindowSettings,
  hardware::{
    Keypad,
    communication_protocol::{CommandEmpty, KeypadCommands},
  },
};
use iced::{
  Element, Event, Subscription, Task, event,
  widget::{Container, button, column, row},
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
  // UpdateConfigFile,
  // ReadPort,
  // // WritePort(KeypadCommands),
  // RequestingAsciiSwitchCodes,

  // PrintBuffer,
  // TaskPrintBuffer(()),

  // TaskRequestingAsciiSwitchCodes(Result<String, serialport::Error>),
  // TaskReadPort(Result<String, serialport::Error>),
  // TaskWritePort(Result<(), serialport::Error>),
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
        window_settings: WindowSettings::load(),
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
        self.window_settings.save();
        Task::none()
      }
      Message::WindowMoved(x, y) => {
        if cfg!(debug_assertions) {
          debug!(
            "window_settings : \nx - {:?}, \ny - {:?}",
            self.window_settings.x, self.window_settings.y
          );
        }
        self.window_settings.x = x;
        self.window_settings.y = y;
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
    let empty_buttons = button("Empty").on_press(Message::WritePort(KeypadCommands::Empty(
      CommandEmpty::VoidRequest,
    )));

    Container::new(column![content, empty_buttons]).into()
  }

  pub fn subscription(&self) -> Subscription<Message> {
    let port_subscription = match self.keypad.is_open {
      true => iced::time::every(Duration::from_millis(10)).map(|_| Message::ReadPort),
      false => Subscription::none(),
    };

    let window_subscription = event::listen_with(|event, _status, _id| match event {
      Event::Window(event) => match event {
        window::Event::Moved(point) => {
          if cfg!(debug_assertions) {
            debug!("point: {point:#?}");
          }
          Some(Message::WindowMoved(point.x, point.y))
        }
        window::Event::Resized(size) => {
          if cfg!(debug_assertions) {
            debug!("size: {size:#?}");
          }
          Some(Message::WindowResized(size.width, size.height))
        }
        _ => None,
      },
      _ => None,
    });

    Subscription::batch(vec![port_subscription, window_subscription])
  }
}
