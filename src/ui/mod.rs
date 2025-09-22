//! Пользовательский интерфейс на базе Iced: состояние, сообщения и представления.

use crate::{
  State,
  data::{Config, code::CodeAscii, device::Device, profiles::Profile, window::Window},
  hardware::{
    buffers::{Buffers, BuffersIO},
    commands::{Value, device, empty, profile, switch},
    serial::{DeviceIO, Keypad, buttons::KeypadButton},
  },
};
use iced::{
  Alignment, Color, Element, Event,
  Length::{self, Fill},
  Point, Subscription, Task, Theme, event,
  widget::{Button, column, container, row, svg, vertical_rule},
  window,
};
use log::{debug, error, info, trace};
use pages::{Icon, PADDING, Pages, SPACING};
use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

pub mod pages;
pub mod styles;

/// Базовая ширина окна по умолчанию
pub const WINDOW_WIDTH: f32 = 800.;
/// Базовая высота окна по умолчанию
pub const WINDOW_HEIGH: f32 = 600.;

/**
Сообщения приложения, обрабатываемые в системе событий
Определяют все возможные действия и взаимодействия в приложении
*/
#[derive(Debug, Clone)]
pub enum Message {
  /// Пустое сообщение (ничего не делает)
  None,

  // --- Порт / последовательный интерфейс ---
  /// Запрос чтения данных из последовательного порта (асинхронно)
  PortReceive,
  /// Сигнал о завершении операции чтения порта
  PortReceived,

  /// Запрос записи данных в последовательный порт (асинхронно)
  PortSend,
  /// Сигнал о завершении операции записи в порт
  PortSended,

  /// Поиск доступного порта устройства
  PortSearch,

  /// Проверка доступности порта (ping пустой командой)
  PortAvalaible,

  /// Запрос состояния всех переключателей на устройстве
  RequestCondition,

  // --- Навигация/страницы ---
  /// Изменение текущей страницы приложения
  ChangePage(Pages),

  // --- UI и настройки окна ---
  /// Сохранение выбранной клавиши/стика для редактирования
  GetButtonSettings(usize, bool),

  /// Сообщение о ресайзе окна (ширина, высота)
  WindowResized(f32, f32),
  /// Сообщение о перемещении окна
  WindowMoved(Point),
  /// Сохранить текущие параметры окна
  WindowSettingsSave,

  // --- Служебные операции устройства ---
  /// Перезагрузка устройства в загрузчик прошивки
  RebootToBootloader,

  // --- Профили ---
  /// Отправить текущий профиль в устройство
  ProfileWrite,
  /// Завершение отправки профиля
  ProfileWrited,
  /// Запросить профиль с устройства
  ProfileReceive,
  /// Сохранить полученный профиль в состоянии
  ProfileReceived(Profile),

  /// Импорт профиля из файла
  ProfileImport,
  /// Экспорт текущего профиля в файл
  ProfileExport,
  /// Записать в устройство профиль, загруженный из файла
  ProfileFileWrite(Profile),

  /// Сохранить активный профиль устройства во Flash
  ProfileFlashSave,

  /// Сделать профиль активным в RAM (1..=4)
  ProfileActiveWriteToRam(u8),
  /// Сделать профиль активным в ROM/Flash (1..=4)
  ProfileActiveWriteToRom(u8),

  /// Запрос номера активного профиля
  ProfileRequestActiveNum,
  /// Сохранить номер активного профиля в состоянии
  ProfileRequestActiveNumState(u8),
  /// Загрузить профиль из RAM в активный
  ProfileLoadRamToActive(u8),
  /// Обновить имя профиля из поля ввода
  ProfileUpdateName(String),

  /// Завершение операций сохранения профиля (в файл или Flash)
  ProfileSaved,

  // --- Редактирование комбинаций ---
  /// Переключить режим записи в ROM/Flash
  WriteButtonIsRom,

  /// Разрешить ввод комбинации (кнопки/стик)
  AllowWriteButtonCombination,
  /// Запретить ввод комбинации и сбросить таймер
  DisallowWriteButtonCombination,
  /// Очистить комбинацию у кнопки/стика
  ClearButtonCombination(usize, bool),
  /// Добавить код в текущую комбинацию (если разрешено)
  WriteButtonCombination(Option<u8>),
  /// Сохранить набранную комбинацию в профиль
  SaveButtonCombination(usize),

  /// Установить мёртвую зону стика
  WriteDeadZone(u8),

  // --- Информация об устройстве ---
  /// Запросить информацию об устройстве
  GetDeviceInfo,
  /// Распарсить ответ об устройстве в структуру
  DeviceInfoParse(Vec<u8>),
  /// Сохранить информацию об устройстве
  DeviceInfoSave(Device),

  // --- Таймеры/служебные ---
  /// Периодическая проверка таймаута режима записи комбинации
  TimerWriteCheck,
}

impl State {
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

    let profile = Task::done(Message::ProfileReceive);
    // let profile = match keypad.is_open {
    //   true => Keypad::receive_profile(),
    //   false => Profile::default(),
    // };

    let device_info_task = match keypad.is_open {
      true => Task::done(Message::GetDeviceInfo),
      false => Task::none(),
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
        active_profile_num: None,
        allow_write: false,
        buffers: Buffers::default(),
        button: KeypadButton::default(),
        device_info: Device::default(),
        is_rom: false,
        keypad,
        pages,
        profile: Profile::default(),
        time_write: None,
        window_settings: Window::load(),
      },
      Task::batch(vec![profile, device_info_task]),
    )
  }

  /// Возвращает заголовок окна приложения
  pub fn title(&self) -> String {
    "Claws".to_string()
  }

  /// Обрабатывает входящие сообщения и обновляет состояние приложения.
  /// Это "сердце" приложения: реакция на события UI, таймеры и обмен с устройством.
  ///
  /// Группы действий:
  /// - Порт: чтение/запись/поиск (`PortReceive`, `PortSend`, `PortSearch`, `PortAvalaible`, `PortSended`, `PortReceived`)
  /// - Навигация: смена страниц (`ChangePage`)
  /// - Окно: размеры/позиция/сохранение (`WindowResized`, `WindowMoved`, `WindowSettingsSave`)
  /// - Профили: запрос/получение/запись/экспорт/импорт/активация (`Profile*`)
  /// - Редактирование комбинаций: разрешение, ввод, очистка, сохранение (`Allow*/Disallow*`, `WriteButtonCombination`, `Clear*`, `SaveButtonCombination`)
  /// - Устройство: информация (`GetDeviceInfo`, `DeviceInfoParse`, `DeviceInfoSave`) и перезагрузка в загрузчик (`RebootToBootloader`)
  /// - Таймеры: автоотключение режима записи (`TimerWriteCheck`)
  ///
  /// # Аргументы
  /// * `message` - Сообщение для обработки
  /// # Возвращает
  /// Задачу для выполнения после обработки сообщения
  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      // Ничего не делаем
      Message::None => Task::none(),
      // Читает данные из порта
      Message::PortReceive => {
        if !self.keypad.is_open {
          return Task::none();
        }

        let mut port = self.keypad.port.clone().unwrap();
        let mut buffers = self.buffers.clone();

        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || Keypad::receive(&mut port, &mut buffers)).await
          },
          |_| Message::PortReceived,
        )
      }
      // Завершение чтения порта — побочных действий нет
      Message::PortReceived => Task::none(),
      // Записывает данные в порт
      Message::PortSend => {
        if !self.keypad.is_open {
          return Task::none();
        }

        let mut port = self.keypad.port.clone().unwrap();
        let mut buf = self.buffers.clone();

        Task::perform(
          async move { tokio::task::spawn_blocking(move || Keypad::send(&mut port, &mut buf)).await },
          |_| Message::PortSended,
        )
      }
      // Завершение записи в порт — побочных действий нет
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
      // Пытаемся найти и открыть последовательный порт устройства
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

        if cfg!(windows)
          && let Err(e) = serial_port.lock().unwrap().write_data_terminal_ready(true)
        {
          error!("Ошибка при установке DTR: {e}");
        }

        self.keypad = Keypad {
          is_open: true,
          port: Some(serial_port),
        };

        info!("Подключение к последовательному порту");

        self.pages = Pages::Profiles;

        // После подключения — запросить профиль
        Task::done(Message::ProfileReceive)
      }
      // Проверка доступности порта пустой командой
      Message::PortAvalaible => {
        let mut buf = self.buffers.clone();

        Task::perform(
          async move { tokio::task::spawn_blocking(move || empty::empty(&mut buf)).await },
          |_| Message::PortSended,
        )
      }
      // Запрос состояния переключателей
      Message::RequestCondition => {
        let mut buf = self.buffers.clone();

        Task::perform(
          async move { tokio::task::spawn_blocking(move || switch::request_condition(&mut buf)).await },
          |_| Message::PortSended,
        )
      }
      // Переключение страницы
      Message::ChangePage(page) => {
        self.pages = page;
        Task::none()
      }
      // Подготовка к вводу/редактированию комбинации кнопки или стика
      Message::GetButtonSettings(id, stick) => {
        self.button.vec_str.clear();
        self.button.code.clear();

        self.button.id = id;

        if stick {
          self.button.code = self.profile.stick.word.to_vec();
        }

        self.button.is_stick = stick;

        // Разрешаем запись комбинации
        Task::done(Message::AllowWriteButtonCombination)
      }
      // Обновление размеров окна
      Message::WindowResized(width, height) => {
        self.window_settings.width = width;
        self.window_settings.height = height;
        Task::none()
      }
      // Обновление позиции окна
      Message::WindowMoved(point) => {
        self.window_settings.x = point.x;
        self.window_settings.y = point.y;
        Task::none()
      }
      // Сохранение настроек окна
      Message::WindowSettingsSave => {
        self.window_settings.save();
        Task::none()
      }
      // Перезагрузка устройства в режим загрузчика
      Message::RebootToBootloader => {
        self.keypad.is_open = false;
        self.keypad.port = None;
        self.pages = Pages::ConnectedDeviceNotFound;

        let port = match Keypad::get_port() {
          Ok(port) => port,
          Err(_) => return Task::none(),
        };
        // Открытие порта на 1200 бод для входа в бутлоадер
        let _ = serialport::new(&port, 1200)
          .timeout(Duration::from_millis(10))
          .open();

        info!("Кейпад перезагружен в режим прошивки");
        Task::none()
      }
      // Отправка текущего профиля в устройство
      Message::ProfileWrite => {
        let mut buffers = self.buffers.clone();
        let profile = self.profile.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || Keypad::profile_send(&mut buffers, profile)).await
          },
          |_| Message::ProfileWrited,
        )
      }
      // Завершение записи профиля
      Message::ProfileWrited => Task::none(),
      // Запрос профиля с устройства
      Message::ProfileReceive => {
        if !self.keypad.is_open {
          return Task::none();
        };

        let mut buffers = self.buffers.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              Keypad::profile_receive(&mut buffers).unwrap_or_default()
            })
            .await
          },
          |profile| match profile {
            Ok(profile) => Message::ProfileReceived(profile),
            Err(_) => Message::ProfileReceive,
          },
        )
      }
      // Сохранение полученного профиля в структуру профиля
      Message::ProfileReceived(profile) => {
        self.profile = profile;
        Task::none()
      }
      // Открыть диалог импорта профиля
      Message::ProfileImport => Profile::open_load_file_dialog(),
      // Экспорт профиля в файл
      Message::ProfileExport => {
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
      // Запись выбранного из файла профиля в устройство
      Message::ProfileFileWrite(profile) => {
        let mut buffers = self.buffers.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || Keypad::profile_send(&mut buffers, profile)).await
          },
          |_| Message::ProfileReceive,
        )
      }
      // Сохранение активного профиля во Flash
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
      // Активировать профиль в RAM
      Message::ProfileActiveWriteToRam(num) => {
        let mut buf = self.buffers.clone();
        let profile = self.profile.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              let _ = Keypad::profile_send(&mut buf, profile);
              buf
                .send()
                .push(profile::Command::WriteActiveToRam(num).get())
            })
            .await
          },
          // |_| Message::ProfileRequestActiveNum,
          |_| Message::None,
        )
      }
      // Активировать профиль в ROM/Flash и синхронизировать RAM
      Message::ProfileActiveWriteToRom(num) => {
        let mut buf = self.buffers.clone();
        let profile = self.profile.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              let _ = Keypad::profile_send(&mut buf, profile);
              buf
                .send()
                .push(profile::Command::WriteActiveToFlash(num).get());
              buf.send().push(profile::Command::LoadFlashToRam.get())
            })
            .await
          },
          // |_| Message::ProfileRequestActiveNum,
          |_| Message::None,
        )
      }
      // Запрос номера активного профиля
      Message::ProfileRequestActiveNum => {
        let mut buf = self.buffers.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || profile::request_active_num(&mut buf)).await
          },
          |res| match res {
            Ok(Ok(num)) => Message::ProfileRequestActiveNumState(num),
            _ => Message::None,
          },
        )
      }
      // Сохраняем номер активного профиля в состояние
      Message::ProfileRequestActiveNumState(num) => {
        self.active_profile_num = Some(num);
        Task::none()
      }
      // Загрузить профиль из RAM ячейки в активный
      Message::ProfileLoadRamToActive(num) => {
        let buf = self.buffers.clone();
        self.active_profile_num = Some(num);
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              buf
                .send()
                .push(profile::Command::LoadRamToActive(num).get())
            })
            .await
          },
          |_| Message::ProfileReceive,
        )
      }
      // Обновление имени профиля из поля ввода
      Message::ProfileUpdateName(string) => {
        self.profile.name = string;
        Task::none()
      }
      // Маркер завершения операций с профилем
      Message::ProfileSaved => Task::none(),
      // Переключение режима записи (ROM<->RAM) и подгрузка данных из Flash при необходимости
      Message::WriteButtonIsRom => {
        self.is_rom = !self.is_rom;

        match self.is_rom {
          true => {
            let buf = self.buffers.clone();
            Task::perform(
              async move {
                tokio::task::spawn_blocking(move || {
                  buf.send().push(profile::Command::LoadFlashToRam.get())
                })
                .await
              },
              |_| Message::None,
            )
          }
          false => Task::none(),
        }
      }
      // Разрешаем набор комбинации, запускаем таймер автоотмены
      Message::AllowWriteButtonCombination => {
        self.allow_write = true;
        self.time_write = Some(std::time::Instant::now());
        Task::none()
      }
      // Запрещаем набор комбинации, сбрасываем таймер
      Message::DisallowWriteButtonCombination => {
        self.allow_write = false;
        self.time_write = None;
        Task::none()
      }
      // Очистка комбинации у кнопки или стика
      Message::ClearButtonCombination(id, is_stick) => {
        match is_stick {
          true => self.profile.stick.word[id - 1] = 0,
          false => self.profile.buttons[id - 1] = [0u8; 6],
        };

        Task::done(Message::DisallowWriteButtonCombination)
      }
      // Добавляем код в текущую комбинацию с ограничениями (1 для стика, до 6 для кнопки)
      Message::WriteButtonCombination(code) => {
        self.time_write = Some(std::time::Instant::now());

        match self.button.is_stick {
          true => {
            if !self.button.vec_str.is_empty() {
              return Task::none();
            }
          }
          false => {
            if self.button.vec_str.len() >= 6 {
              return Task::none();
            }
          }
        }

        let elem = Profile::code_to_title(code.unwrap_or(0));
        let code = code.unwrap_or(0);

        if self.button.vec_str.contains(&elem) {
          return Task::none();
        };

        self.button.vec_str.push(elem);

        match self.button.is_stick {
          true => self.button.code[self.button.id - 1] = code,
          false => self.button.code.push(code),
        };

        Task::done(Message::SaveButtonCombination(self.button.id))
      }
      // Сохраняем набранную комбинацию в профиле
      Message::SaveButtonCombination(id) => {
        debug!("{:?}", self.profile.buttons);

        let mut code = self.button.code.clone();

        match self.button.is_stick {
          true => {
            code.resize(4, 0);
            self.profile.stick.word = code.try_into().unwrap();
          }
          false => {
            code.resize(6, 0);
            self.profile.buttons[id - 1] = code.try_into().unwrap();
          }
        }

        debug!("{:?}", self.profile.buttons);
        debug!("{:?}", self.profile.stick.word);

        // Task::done(Message::ClearButtonCombination)
        Task::none()
      }
      // Сохраняем мёртвую зону стика
      Message::WriteDeadZone(deadzone) => {
        self.profile.stick.deadzone = deadzone;
        Task::none()
      }
      // Запрашиваем информацию об устройстве
      Message::GetDeviceInfo => {
        let mut buffers = self.buffers.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || device::request_info(&mut buffers).unwrap()).await
          },
          |res| match res {
            Ok(res) => Message::DeviceInfoParse(res),
            Err(_) => Message::GetDeviceInfo,
          },
        )
      }
      // Асинхронный парсинг ответа устройства
      Message::DeviceInfoParse(res) => Task::perform(
        async move { Device::parse(&res).await },
        Message::DeviceInfoSave,
      ),
      // Сохраняем информацию об устройстве в структуру устройства
      Message::DeviceInfoSave(device) => {
        self.device_info = device;
        Task::none()
      }
      // Периодическая проверка таймаута набора комбинации (2 секунды)
      Message::TimerWriteCheck => {
        if let Some(start_time) = self.time_write
          && start_time.elapsed() >= Duration::from_secs(2)
        {
          self.allow_write = false;
          self.time_write = None;
        }
        Task::none()
      }
    }
  }

  /**
  Возвращает текущее представление приложения
  Строит UI на основе текущего состояния приложения
  */
  pub fn view(&self) -> Element<'_, Message> {
    let page = if cfg!(debug_assertions) {
      Pages::get_content(self, &self.profile).explain(Color::from_rgb(255., 0., 0.))
    } else {
      Pages::get_content(self, &self.profile)
    };

    let sidebar = container(
      column![
        create_button_with_svg_and_text(Icon::Profiles, Message::ChangePage(Pages::Profiles)),
        create_button_with_svg_and_text(Icon::Settings, Message::ChangePage(Pages::Settings)),
        create_button_with_svg_and_text(Icon::Update, Message::ChangePage(Pages::Updater)),
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
    // Таймеры обмена с портом: при открытом порте — частый опрос чтения/записи,
    // при закрытом — редкий опрос на подключение устройства
    let port_sub = match self.keypad.is_open {
      true => iced::Subscription::batch(vec![
        iced::time::every(Duration::from_millis(1)).map(|_| Message::PortSend),
        iced::time::every(Duration::from_millis(1)).map(|_| Message::PortReceive),
      ]),
      false => iced::time::every(Duration::from_secs(1)).map(|_| Message::PortSearch),
    };

    // Подписка на события окна: перемещение, изменение размера,
    // а также сохранение параметров при фокусе/расфокусе/закрытии
    let window = event::listen_with(|event, _status, _id| match event {
      Event::Window(event) => match event {
        #[cfg(windows)]
        window::Event::Moved(point) => {
          trace!("subscription: window: moved: {point:#?}");
          Some(Message::WindowMoved(point))
        }
        window::Event::Resized(size) => {
          trace!("subscription: window: resized: {size:#?}");
          Some(Message::WindowResized(size.width, size.height))
        }
        window::Event::Focused | window::Event::Unfocused | window::Event::CloseRequested => {
          info!("subscription: window: сохранение положения окна");
          Some(Message::WindowSettingsSave)
        }
        _ => None,
      },
      _ => None,
    });

    // Захват нажатий клавиатуры для набора комбинации
    // Активен только когда разрешён режим записи комбинации
    let keyboard = match self.allow_write {
      true => event::listen_with(|event, _status, _id| match event {
        Event::Keyboard(event) => {
          if let iced::keyboard::Event::KeyPressed {
            key, physical_key, ..
          } = event
          {
            match (key, physical_key) {
              (iced::keyboard::Key::Named(named), iced::keyboard::key::Physical::Code(code)) => {
                debug!("named: {:?}, code {:?}", named, code.to_ascii());
                Some(Message::WriteButtonCombination(code.to_ascii()))
              }
              (iced::keyboard::Key::Character(c), iced::keyboard::key::Physical::Code(code)) => {
                debug!("named: {:?}, code {:?}", c, code.to_ascii());
                Some(Message::WriteButtonCombination(code.to_ascii()))
              }
              _ => None,
            }
          } else {
            None
          }
        }
        _ => None,
      }),
      false => Subscription::none(),
    };

    // Периодический запрос номера активного профиля при открытой странице "Профили"
    let profile_active = match (&self.pages, &self.keypad.is_open) {
      (Pages::Profiles, true) => {
        iced::time::every(Duration::from_millis(500)).map(|_| Message::ProfileRequestActiveNum)
      }
      _ => Subscription::none(),
    };

    // Таймер проверки таймаута режима записи комбинации
    let timer_check = match self.allow_write {
      true => iced::time::every(Duration::from_millis(100)).map(|_| Message::TimerWriteCheck),
      false => Subscription::none(),
    };

    Subscription::batch(vec![
      port_sub,
      window,
      keyboard,
      profile_active,
      timer_check,
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
Создает кнопку с SVG-иконкой
# Аргументы
* `icon` - Иконка из перечисления `Icon`
* `on_press` - Сообщение при нажатии
# Возвращает
Кнопка с иконкой
*/
fn create_button_with_svg_and_text<'a>(icon: Icon, on_press: Message) -> Button<'a, Message> {
  Button::new(container(
    svg(svg::Handle::from_memory(icon.icon()))
      .height(Fill)
      .width(Fill),
  ))
  .width(Length::Fixed(50.))
  .height(Length::Fixed(50.))
  .on_press(on_press)
  .style(styles::button::rounding)
}
