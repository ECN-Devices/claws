use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

use iced::{Point, Task};
use log::{debug, info, trace};

use crate::{
  State,
  data::{Config, device::Device, profiles::Profile, stick::Stick},
  hardware::{
    commands::{device, profile, stick},
    serial::{DeviceIO, Keypad, profile::profile_all_request},
  },
  ui::pages::Pages,
};

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
  PrePortSearch,
  PortSearch,

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
  /// Запросить профиль с устройства
  ProfileReceive(usize),
  /// Сохранить полученный профиль в состоянии
  ProfileReceived((Profile, usize)),

  ProfileReceiveKeypadVec,
  ProfileReceivedKeypadVec((Vec<Profile>, usize)),

  ProfileNew,
  ProfileRemove(usize),
  ProfileSave((usize, Profile)),

  ProfileLoadKeypad(usize),
  ProfileLoadLocal(usize),

  /// Импорт профиля из файла
  ProfileImport,
  ProfileImported(Vec<Profile>),

  /// Экспорт вектора профилей в файл
  ProfilesExport,

  /// Завершение операций сохранения профиля (в файл или Flash)
  ProfileSaved,

  /// Показать список сохранённых профилей
  ProfilesListLoad,
  ProfilesListSave(Vec<Profile>),

  /// Сделать профиль активным в RAM (1..=4)
  ProfileActiveWriteToRam(u8),
  /// Сделать профиль активным в ROM/Flash (1..=4)
  ProfileActiveWriteToRom(u8),

  /// Запрос номера активного профиля
  ProfileRequestActiveNum,
  /// Сохранить номер активного профиля в состоянии
  ProfileRequestActiveNumState(usize),
  /// Обновить имя профиля из поля ввода
  ProfileUpdateName(String),

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

  StickInitCalibration,
  StickStartCalibration,
  StickCalibrated,
  StickGetCalibrateParameters,
  StickParseCalibrateParameters(Vec<u8>),
  StickInfoSave(Stick),
  StickEndCalibration,

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
  Обрабатывает входящие сообщения и обновляет состояние приложения.
  Это "сердце" приложения: реакция на события UI, таймеры и обмен с устройством.

  Группы действий:
  - Порт: чтение/запись/поиск (PortReceive, PortSend, PortSearch, PortAvalaible, PortSended, PortReceived)
  - Навигация: смена страниц (ChangePage)
  - Окно: размеры/позиция/сохранение (WindowResized, WindowMoved, WindowSettingsSave)
  - Профили: запрос/получение/запись/экспорт/импорт/активация (Profile)
  - Редактирование комбинаций: разрешение, ввод, очистка, сохранение (Allow/Disallow, WriteButtonCombination, Clear, SaveButtonCombination)
  - Устройство: информация (GetDeviceInfo, DeviceInfoParse, DeviceInfoSave) и перезагрузка в загрузчик (RebootToBootloader)
  - Таймеры: автоотключение режима записи (TimerWriteCheck)

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
      Message::PrePortSearch => {
        self.pages = Pages::ConnectedDeviceNotFound;
        self.keypad.is_open = false;
        self.keypad.port = None;

        Task::done(Message::PortSearch)
      }
      Message::PortSearch => {
        let port_name = match Keypad::get_port() {
          Ok(port) if !port.is_empty() => port,
          _ => return Task::none(),
        };

        let serial_port = Arc::new(Mutex::new(
          serialport::new(&port_name, 115_200)
            .timeout(Duration::from_millis(10))
            .dtr_on_open(true)
            .open()
            .expect("Ошибка открытия порта"),
        ));

        self.keypad = Keypad {
          is_open: true,
          port: Some(serial_port),
        };

        info!("Подключение к последовательному порту");

        self.pages = Pages::Profiles;

        // После подключения — запросить профиль
        // Task::done(Message::ProfileReceive)
        Task::done(Message::ProfileReceiveKeypadVec)
      }
      Message::ChangePage(page) => {
        self.pages = page;
        Task::none()
      }
      Message::GetButtonSettings(id, stick) => {
        self.button.vec_str.clear();
        self.button.code.clear();

        self.button.id = id;

        if stick {
          self.button.code = self.profile.stick.word.to_vec();
        }

        self.button.is_stick = stick;

        // Разрешаем запись комбинации
        if !self.profile_on_keypad {
          return Task::done(Message::AllowWriteButtonCombination);
        }
        Task::none()
      }
      Message::WindowResized(width, height) => {
        self.window_settings.width = width;
        self.window_settings.height = height;
        Task::none()
      }
      Message::WindowMoved(point) => {
        self.window_settings.x = point.x;
        self.window_settings.y = point.y;
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
        // Открытие порта на 1200 бод для входа в бутлоадер
        let _ = serialport::new(&port, 1200)
          .timeout(Duration::from_millis(10))
          .open();

        info!("Кейпад перезагружен в режим прошивки");
        Task::none()
      }
      Message::ProfileReceive(id) => {
        if !self.keypad.is_open {
          return Task::none();
        };

        self.profile_write = true;

        let request_active_profile_id = self.request_active_profile_id.unwrap();
        let mut buffers = self.buffers.clone();
        Task::perform(
          async move {
            buffers.send().push(&profile::Command::LoadRamToActive(id));

            let profile = Keypad::profile_receive(&mut buffers).await;

            buffers.send().push(&profile::Command::LoadRamToActive(
              request_active_profile_id,
            ));

            profile
          },
          move |res| match res {
            Ok(profile) => Message::ProfileReceived((profile, id)),
            Err(_) => Message::ProfileReceive(id),
          },
        )
      }
      // Сохранение полученного профиля в структуру профиля
      Message::ProfileReceived((profile, id)) => {
        self.profile_write = false;
        self.profile = profile.clone();
        self.profiles_keypad_vec[id - 1] = profile;
        Task::none()
      }
      Message::ProfileReceiveKeypadVec => {
        self.profile_write = true;
        let mut buffers = self.buffers.clone();
        Task::perform(
          async move { profile_all_request(&mut buffers).await },
          |res| match res {
            Ok(res) => Message::ProfileReceivedKeypadVec(res),
            Err(_) => Message::ProfileReceiveKeypadVec,
          },
        )
      }
      Message::ProfileReceivedKeypadVec(res) => {
        self.profile_write = false;

        match self.is_first_start {
          true => {
            self.active_profile_id = Some(res.1);
            self.profiles_keypad_vec = res.0.clone();
            self.profile = res.0.get(res.1 - 1).unwrap().clone();
            self.profile_on_keypad = true;
            self.is_first_start = !self.is_first_start;
          }
          false => {
            self.profiles_keypad_vec = res.0.clone();
          }
        };

        Task::none()
      }
      Message::ProfileNew => {
        self.profiles_local_vec.push(self.profile.clone());

        Task::done(Message::ProfilesExport)
        // Task::none()
      }
      Message::ProfileRemove(idx) => {
        self.profiles_local_vec.remove(idx);
        Task::done(Message::ProfilesExport)
      }
      Message::ProfileSave(profile) => {
        if self.profile_on_keypad {
          return Task::none();
        }

        let (idx, profile) = profile;

        self.profiles_local_vec[idx] = profile;
        Task::done(Message::ProfilesExport)
      }
      Message::ProfileLoadKeypad(idx) => {
        self.profile_on_keypad = true;

        if let Some(profile) = self.profiles_keypad_vec.get(idx - 1).cloned() {
          self.active_profile_id = Some(idx);
          self.profile = profile;
        }

        Task::none()
      }
      Message::ProfileLoadLocal(idx) => {
        self.profile_on_keypad = false;

        if let Some(profile) = self.profiles_local_vec.get(idx).cloned() {
          self.local_profile_id = Some(idx);
          self.profile = profile;
        }
        Task::none()
      }
      // Открыть диалог импорта профиля
      Message::ProfileImport => Profile::open_load_file_dialog(),
      Message::ProfileImported(profiles) => {
        profiles
          .iter()
          .for_each(|profile| self.profiles_local_vec.push(profile.clone()));

        Task::done(Message::ProfilesExport)
      }
      Message::ProfilesExport => {
        let profiles = self.profiles_local_vec.clone();

        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              trace!("message: ProfilesExport: сохранение профилей в файл");
              // debug!("{:?}", profiles);
              Profile::save(profiles)
            })
            .await
          },
          |_| Message::ProfileSaved,
        )
      }
      Message::ProfileSaved => Task::none(),
      Message::ProfilesListLoad => Task::perform(
        async move { Profile::load().await },
        Message::ProfilesListSave,
      ),
      Message::ProfilesListSave(vec) => {
        if self.profiles_local_vec == vec {
          return Task::none();
        }

        self.profiles_local_vec = vec;
        Task::done(Message::ProfilesExport)
      }
      Message::ProfileActiveWriteToRam(num) => {
        self.profile_write = true;

        let mut buf = self.buffers.clone();
        let profile = self.profile.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              let _ = Keypad::profile_send(&mut buf, profile);
              buf.send().push(&profile::Command::WriteActiveToRam(num))
            })
            .await
          },
          // |_| Message::ProfileRequestActiveNum,
          move |_| Message::ProfileReceive(num as usize),
        )
      }
      Message::ProfileActiveWriteToRom(num) => {
        let mut buf = self.buffers.clone();
        let profile = self.profile.clone();
        Task::perform(
          async move {
            tokio::task::spawn_blocking(move || {
              let _ = Keypad::profile_send(&mut buf, profile);
              buf.send().push(&profile::Command::WriteActiveToFlash(num));
              buf.send().push(&profile::Command::LoadFlashToRam)
            })
            .await
          },
          // |_| Message::ProfileRequestActiveNum,
          |_| Message::None,
        )
      }
      Message::ProfileRequestActiveNum => {
        let mut buf = self.buffers.clone();
        Task::perform(
          async move { profile::request_active_num(&mut buf).await },
          |res| match res {
            Ok(num) => Message::ProfileRequestActiveNumState(num as usize),
            Err(_) => Message::PrePortSearch,
          },
        )
      }
      Message::ProfileRequestActiveNumState(num) => {
        self.request_active_profile_id = Some(num);
        Task::none()
      }
      Message::ProfileUpdateName(string) => {
        if string.len() >= 16 {
          return Task::none();
        }

        self.profile.name = string;

        Task::done(Message::ProfileSave((
          self.local_profile_id.unwrap_or(0),
          self.profile.clone(),
        )))
      }
      Message::WriteButtonIsRom => {
        self.is_rom = !self.is_rom;

        match self.is_rom {
          true => {
            let buf = self.buffers.clone();
            Task::perform(
              async move {
                tokio::task::spawn_blocking(move || {
                  buf.send().push(&profile::Command::LoadFlashToRam)
                })
                .await
              },
              |_| Message::ProfileReceiveKeypadVec,
            )
          }
          false => Task::none(),
        }
      }
      Message::AllowWriteButtonCombination => {
        self.allow_write = true;
        self.time_write = Some(std::time::Instant::now());
        Task::none()
      }
      Message::DisallowWriteButtonCombination => {
        self.allow_write = false;
        self.time_write = None;
        Task::done(Message::ProfilesExport)
      }
      Message::ClearButtonCombination(id, is_stick) => {
        match is_stick {
          true => self.profile.stick.word[id - 1] = 0,
          false => self.profile.buttons[id - 1] = [0u8; 6],
        };

        Task::done(Message::DisallowWriteButtonCombination)
      }
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
        Task::done(Message::ProfileSave((
          self.local_profile_id.unwrap_or(0),
          self.profile.clone(),
        )))
        // Task::none()
      }
      Message::WriteDeadZone(deadzone) => {
        self.profile.stick.deadzone = deadzone;
        Task::done(Message::ProfileSave((
          self.local_profile_id.unwrap_or(0),
          self.profile.clone(),
        )))
      }
      Message::StickInitCalibration => {
        self.stick_callibrate = true;
        Task::none()
      }
      Message::StickStartCalibration => {
        self.stick_callibrate_time = Some(std::time::Instant::now());

        self.buffers.send().push(&stick::Command::Calibration(
          stick::OptionsCalibration::Calibrate,
        ));

        Task::none()
      }
      Message::StickCalibrated => {
        if let Some(time) = self.stick_callibrate_time
          && time.elapsed() >= Duration::from_millis(6500)
        {
          self.stick_callibrate_time = None;
          self.stick_show_calibrate_parameters = true;
          return Task::done(Message::StickGetCalibrateParameters);
        }
        Task::none()
      }
      Message::StickGetCalibrateParameters => {
        let mut buf = self.buffers.clone();
        Task::perform(
          async move { stick::calibration_request(&mut buf).await },
          |res| match res {
            Ok(res) => Message::StickParseCalibrateParameters(res),
            _ => Message::StickGetCalibrateParameters,
          },
        )
      }
      Message::StickParseCalibrateParameters(res) => Task::perform(
        async move { Stick::parse(&res).await },
        Message::StickInfoSave,
      ),
      Message::StickInfoSave(stick) => {
        self.stick_info = stick;
        Task::none()
      }
      Message::StickEndCalibration => {
        self.stick_callibrate = false;
        self.stick_show_calibrate_parameters = false;
        Task::none()
      }
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
      Message::DeviceInfoParse(res) => Task::perform(
        async move { Device::parse(&res).await },
        Message::DeviceInfoSave,
      ),
      Message::DeviceInfoSave(device) => {
        self.device_info = device;
        Task::none()
      }
      Message::TimerWriteCheck => {
        if let Some(start_time) = self.time_write
          && start_time.elapsed() >= Duration::from_secs(2)
        {
          self.allow_write = false;
          self.time_write = None;
          return Task::done(Message::ProfilesExport);
        }
        Task::none()
      }
    }
  }
}
