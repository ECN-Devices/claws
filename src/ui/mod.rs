use crate::{
  App,
  data::{
    Config, open_load_file_dialog,
    profiles::{Profile, SerialProfile},
    window::Window,
  },
  hardware::{
    commands::{KeypadCommands, empty},
    serial::{Keypad, SerialOperations},
  },
};
use iced::{
  Alignment, Element, Event,
  Length::{self, Fill},
  Subscription, Task, Theme, event,
  widget::{Button, Container, Tooltip, column, container, row, svg, tooltip, vertical_rule},
  window,
};
use log::{debug, error};
use pages::{Icon, PADDING, Pages, SPACING};
use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

pub mod pages;

pub const WINDOW_WIDTH: f32 = 800.;
pub const WINDOW_HEIGH: f32 = 600.;

/**
Сообщения приложения, обрабатываемые в системе событий
Определяют все возможные действия и взаимодействия в приложении
*/
#[derive(Debug, Clone)]
pub enum Message {
  /// Чтение данных с последовательного порта
  ReadPort,

  /// Запись команды на последовательный порт
  WritePort(KeypadCommands),

  /// Поиск доступного последовательного порта
  SearchPort,

  /// Изменение текущей страницы приложения
  ChangePage(Pages),
  ButtonClicked,

  /// Изменение размеров окна
  WindowResized(f32, f32),

  /// Перемещение окна
  WindowMoved(f32, f32),

  /// Сохранение настроек окна
  WindowSettingsSave,
  /// Перезагрузка устройства в режим прошивки
  RebootToBootloader,

  /// Запись профиля на устройство
  WriteProfile,

  /// Сохранение профиля из устройства
  SaveProfile,

  /// Открытие диалога выбора файла
  OpenFileDialog,
}

impl App {
  /**
  Создает новый экземпляр приложения с инициализацией
  # Возвращает
  Кортеж из:
  - Экземпляр приложения
  - Инициализационная задача
  */
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

        if cfg!(windows) {
          if let Err(e) = serial_port.lock().unwrap().write_data_terminal_ready(true) {
            error!("Ошибка при установке DTR: {e}");
          }
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
        window_settings: Window::load(),
      },
      Task::none(),
    )
  }

  /// Возвращает заголовок окна приложения
  pub fn title(&self) -> String {
    String::from("Claws")
  }

  /**
  Обрабатывает входящие сообщения и обновляет состояние приложения
  # Аргументы
  * `message` - Сообщение для обработки
  # Возвращает
  Задачу для выполнения после обработки сообщения
  */
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
        match Keypad::write_port(&mut port, &command) {
          Ok(_) => (),
          Err(err) => {
            self.pages = Pages::ConnectedDeviceNotFound;
            self.keypad = err
          }
        };
        Task::none()
      }
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

        if cfg!(windows) {
          if let Err(e) = serial_port.lock().unwrap().write_data_terminal_ready(true) {
            error!("Ошибка при установке DTR: {e}");
          }
        }

        self.keypad = Keypad {
          is_open: true,
          port: Some(serial_port),
        };

        debug!("Подключение к последовательному порту");

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
        self.keypad.is_open = false;
        self.keypad.port = None;
        let port = Keypad::get_port();

        let _ = serialport::new(&port, 1200)
          .timeout(Duration::from_millis(10))
          .open();

        self.pages = Pages::ConnectedDeviceNotFound;

        debug!("Кейпад перезагружен в режим прошивки");
        Task::none()
      }
      Message::WriteProfile => {
        let mut port = self.keypad.port.clone().unwrap();
        let profile = Profile::load("Lol");
        Keypad::write_profile(&mut port, profile);
        Task::none()
      }
      Message::SaveProfile => {
        let mut port = self.keypad.port.clone().unwrap();
        Keypad::save_profile_file(&mut port);
        Task::none()
      }
      Message::OpenFileDialog => open_file_dialog(),
    }
  }

  /**
  Возвращает текущее представление приложения
  Строит UI на основе текущего состояния приложения
  */
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
      .spacing(SPACING),
    )
    .align_y(Alignment::Center)
    .padding(PADDING)
    .height(Length::Fill);

    let content = match self.keypad.is_open {
      true => row![sidebar, vertical_rule(2), page],
      false => {
        if cfg!(debug_assertions) {
          return row![sidebar, vertical_rule(2), page].into();
        }
        row![page]
      }
    };

    Container::new(content).into()
  }

  /// Возвращает подписки на события приложения
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

    let port_available = iced::time::every(Duration::from_secs(5))
      .map(|_| Message::WritePort(KeypadCommands::Empty(empty::Command::VoidRequest)));

    let window = event::listen_with(|event, _status, _id| match event {
      Event::Window(event) => match event {
        #[cfg(windows)]
        window::Event::Moved(point) => {
          debug!("subscription: event: window: moved: {point:#?}");
          Some(Message::WindowMoved(point.x, point.y))
        }
        window::Event::Resized(size) => {
          debug!("subscription: event: window: resized: {size:#?}");
          Some(Message::WindowResized(size.width, size.height))
        }
        window::Event::Focused | window::Event::Unfocused => {
          debug!("subscription: event: window: focused: сохранение настроек положения окна");
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

  /// Возвращает текущую тему приложения
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

/**
Создает кнопку с SVG-иконкой и текстовой подсказкой
# Аргументы
* `button_text` - Текст подсказки
* `icon` - Иконка из перечисления `Icon`
* `on_press` - Сообщение при нажатии
# Возвращает
Элемент интерфейса с кнопкой и подсказкой
*/
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
