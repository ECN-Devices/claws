use crate::{
  App,
  data::ConfigWindow,
  hardware::{Keypad, communication_protocol::KeypadCommands},
};
use iced::{
  Alignment, Element, Event,
  Length::{self, Fill},
  Subscription, Task, Theme, event,
  widget::{Button, Container, Tooltip, column, container, row, svg, tooltip, vertical_rule},
  window,
};
use log::{debug, error};
use pages::{Icon, Pages};
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
  SearchPort,
  ChangePage(Pages),
  ButtonClicked,
  WindowResized(f32, f32),
  WindowMoved(f32, f32),
  WindowSettingsSave,
  RebootToBootloader,
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
      Message::SearchPort => {
        let port = Keypad::get_port();

        if port.is_empty() {
          return Task::none();
        };

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

        self.keypad = Keypad {
          port: Some(serial_port),
          is_open: true,
        };

        if cfg!(debug_assertions) {
          debug!("Подключение к последовательному порту");
        }

        self.pages = Pages::Profiles;

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
      Message::RebootToBootloader => {
        self.keypad.port = None;
        self.keypad.is_open = false;
        let port = Keypad::get_port();

        let _ = serialport::new(&port, 1200)
          .timeout(Duration::from_millis(10))
          .open();

        self.pages = Pages::ConnectedDeviceNotFound;

        if cfg!(debug_assertions) {
          debug!("Кейпад перезагружен в режим прошивки");
        }
        Task::none()
      }
    }
  }

  pub fn view(&self) -> Element<Message> {
    let page = Pages::get_content(self);

    let sidebar = container(
      column![
        create_button_with_svg_and_text(
          "Профили",
          Icon::Profiles,
          Message::ChangePage(Pages::Profiles)
        ),
        create_button_with_svg_and_text(
          "Настройки",
          Icon::Settings,
          Message::ChangePage(Pages::Settings)
        ),
        create_button_with_svg_and_text(
          "Обновление",
          Icon::Update,
          Message::ChangePage(Pages::Updater)
        ),
        create_button_with_svg_and_text(
          "Экспериментальные настройки",
          Icon::Experimental,
          Message::ChangePage(Pages::Experimental)
        ),
      ]
      .spacing(10),
    )
    .align_y(Alignment::Center)
    .padding(10)
    .height(Length::Fill);

    let content = match self.keypad.is_open {
      true => row![sidebar, vertical_rule(2), page],
      false => {
        #[cfg(debug_assertions)]
        {
          row![sidebar, vertical_rule(10), page]
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
    let port_read = match self.keypad.is_open {
      true => iced::time::every(Duration::from_millis(10)).map(|_| Message::ReadPort),
      false => Subscription::none(),
    };
    let port_search = match self.keypad.is_open {
      true => Subscription::none(),
      false => iced::time::every(Duration::from_secs(2)).map(|_| Message::SearchPort),
    };
    let port_disconect = match self.keypad.port {
      Some(_) => Subscription::none(),
      None => iced::time::every(Duration::from_secs(2)).map(|_| Message::SearchPort),
    };

    let port_available = iced::time::every(Duration::from_secs(5)).map(|_| {
      Message::WritePort(KeypadCommands::Empty(
        crate::hardware::communication_protocol::CommandEmpty::VoidRequest,
      ))
    });

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
      port_read,
      port_search,
      port_disconect,
      port_available,
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

fn create_button_with_svg_and_text<'a>(
  button_text: &'a str,
  icon: Icon,
  on_press: Message,
) -> Tooltip<'a, Message> {
  let button = Button::new(column![
    svg(svg::Handle::from_memory(icon.icon()))
      .height(Fill)
      .width(Fill),
  ])
  .width(Length::Fixed(50.))
  .height(Length::Fixed(50.))
  .on_press(on_press);

  tooltip(button, button_text, tooltip::Position::Right)
}
