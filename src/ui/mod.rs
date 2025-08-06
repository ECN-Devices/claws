use crate::{
  App,
  data::{Config, file_dialog::FileDialog, profiles::Profile, window::Window},
  hardware::{
    buffers::{Buffers, BuffersIO},
    commands::{Value, empty, profile, switch},
    serial::{DeviceIO, Keypad},
  },
};
use iced::{
  Alignment, Element, Event,
  Length::{self, Fill},
  Subscription, Task, Theme, event,
  widget::{Button, Tooltip, column, container, row, svg, tooltip, vertical_rule},
  window,
};
use log::{error, info, trace};
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
  None,

  /// Port
  PortReceive,
  PortReceived,

  PortSend,
  PortSended,

  PortSearch,

  PortAvalaible,

  RequestCondition,

  /// Изменение текущей страницы приложения
  ChangePage(Pages),

  ButtonClicked,

  /// Подписки
  WindowResized(f32, f32),
  WindowMoved(f32, f32),
  WindowSettingsSave,

  /// Перезагрузка устройства в режим прошивки
  RebootToBootloader,

  /// Профиль
  ProfileWrite,
  ProfileWrited,
  ProfileReceive,
  ProfileReceived(Profile),

  ProfileFileSave,
  ProfileFileWrite(Profile),

  ProfileFlashSave,

  ProfileSaved,

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
    let port = match Keypad::get_port() {
      Ok(s) => s,
      Err(e) => {
        error!("{e}");
        "".to_string()
      }
    };

    let keypad = match !port.is_empty() {
      true => {
        let serial_port = Arc::new(Mutex::new(
          serialport::new(&port, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Ошибка открытия порта"),
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

    let profile = Task::done(Message::ProfileReceive);
    // let profile = match keypad.is_open {
    //   true => Keypad::receive_profile(),
    //   false => Profile::default(),
    // };

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
        profile: Profile::default(),
        buffers: Buffers::default(),
      },
      Task::batch(vec![profile]),
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
      Message::None => Task::none(),
      Message::PortReceive => {
        if !self.keypad.is_open {
          return Task::none();
        }
        let mut port = self.keypad.port.clone().unwrap();
        let mut buffers = self.buffers.clone();

        // {
        //   trace!("send: {:?}", self.buffers.send());
        //   trace!("receive: {:?}", self.buffers.receive())
        // }

        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || Keypad::receive(&mut port, &mut buffers)).await
          },
          |_| Message::PortReceived,
        )
      }
      Message::PortReceived => Task::none(),
      Message::PortSend => {
        if !self.keypad.is_open {
          return Task::none();
        }

        let mut port = self.keypad.port.clone().unwrap();
        let mut buf = self.buffers.clone();
        // debug!("{}", buf.send().len());

        Task::perform(
          async move { tokio::task::spawn_blocking(move || Keypad::send(&mut port, &mut buf)).await },
          |_| Message::PortSended,
        )
      }
      Message::PortSended => {
        // match res {
        //   Ok(_) => (),
        //   Err(keypad) => {
        //     self.pages = Pages::ConnectedDeviceNotFound;
        //     self.keypad = keypad
        //   }
        // }
        Task::none()
      }
      Message::PortSearch => {
        let port = match Keypad::get_port() {
          Ok(port) if !port.is_empty() => port,
          _ => return Task::none(),
        };

        let serial_port = Arc::new(Mutex::new(
          serialport::new(&port, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Ошибка открытия порта"),
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

        info!("Подключение к последовательному порту");

        self.pages = Pages::Profiles;

        Task::done(Message::ProfileReceive)
      }
      Message::PortAvalaible => {
        let mut buf = self.buffers.clone();

        Task::perform(
          async move { tokio::task::spawn_blocking(move || empty::empty(&mut buf)).await },
          |_| Message::PortSended,
        )
      }
      Message::RequestCondition => {
        let mut buf = self.buffers.clone();

        Task::perform(
          async move { tokio::task::spawn_blocking(move || switch::request_condition(&mut buf)).await },
          |_| Message::PortSended,
        )
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
        self.pages = Pages::ConnectedDeviceNotFound;

        let port = match Keypad::get_port() {
          Ok(port) => port,
          Err(_) => return Task::none(),
        };

        serialport::new(&port, 1200)
          .timeout(Duration::from_millis(10))
          .open()
          .unwrap();

        info!("Кейпад перезагружен в режим прошивки");
        Task::none()
      }
      Message::ProfileWrite => {
        let mut buffers = self.buffers.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              let profile = Profile::load("Lol");
              Keypad::profile_send(&mut buffers, profile)
            })
            .await
          },
          |_| Message::ProfileReceive,
        )
      }
      Message::ProfileWrited => Task::none(),
      Message::ProfileReceive => {
        let mut buffers = self.buffers.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || Keypad::profile_receive(&mut buffers).unwrap())
              .await
          },
          |profile| match profile {
            Ok(profile) => Message::ProfileReceived(profile),
            Err(_) => Message::ProfileReceive,
          },
        )
      }
      Message::ProfileReceived(profile) => {
        self.profile = profile;
        Task::none()
      }
      Message::ProfileFileSave => {
        let profile = self.profile.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              trace!("message: ProfileFileSave : сохранение профиля в файл");
              Keypad::save_profile_file(profile)
            })
            .await
          },
          |_| Message::ProfileSaved,
        )
      }
      Message::ProfileFileWrite(profile) => {
        let mut buffers = self.buffers.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || Keypad::profile_send(&mut buffers, profile)).await
          },
          |_| Message::ProfileReceive,
        )
      }
      Message::ProfileFlashSave => {
        let buf = self.buffers.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              trace!("message: ProfileFlashSave: сохранение профиля в flash память");
              buf
                .send()
                .push(profile::Command::WriteActiveToFlash(1).get())
            })
            .await
          },
          |_| Message::ProfileSaved,
        )
      }
      Message::ProfileSaved => Task::none(),
      Message::OpenFileDialog => Profile::open_load_file_dialog(),
    }
  }

  /**
  Возвращает текущее представление приложения
  Строит UI на основе текущего состояния приложения
  */
  pub fn view(&self) -> Element<Message> {
    let page = Pages::get_content(self, self.profile.clone());

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

    container(content).into()
  }

  /// Возвращает подписки на события приложения
  pub fn subscription(&self) -> Subscription<Message> {
    let port_sub = match self.keypad.is_open {
      true => iced::Subscription::batch(vec![
        iced::time::every(Duration::from_millis(1)).map(|_| Message::PortSend),
        iced::time::every(Duration::from_millis(1)).map(|_| Message::PortReceive),
      ]),
      false => iced::time::every(Duration::from_secs(1)).map(|_| Message::PortSearch),
    };

    let window = event::listen_with(|event, _status, _id| match event {
      Event::Window(event) => match event {
        #[cfg(windows)]
        window::Event::Moved(point) => {
          trace!("subscription: window: moved: {point:#?}");
          Some(Message::WindowMoved(point.x, point.y))
        }
        window::Event::Resized(size) => {
          trace!("subscription: window: resized: {size:#?}");
          Some(Message::WindowResized(size.width, size.height))
        }
        window::Event::Focused | window::Event::Unfocused | window::Event::CloseRequested => {
          info!("subscription: window: сохранение настроек положения окна");
          Some(Message::WindowSettingsSave)
        }
        _ => None,
      },
      _ => None,
    });

    Subscription::batch(vec![port_sub, window])
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
