use std::time::Duration;

use iced::{Event, Subscription, event, window};
use log::{debug, info, trace};

use crate::{
  State,
  data::code::CodeAscii,
  hardware::buffers::BuffersIO,
  ui::{Message, pages::Pages},
};

impl State {
  /// Возвращает подписки на события приложения
  pub fn subscription(&self) -> Subscription<Message> {
    // Таймеры обмена с портом: при открытом порте — частый опрос чтения/записи,
    // при закрытом — редкий опрос на подключение устройства
    let port_sub = match self.keypad.is_open {
      true => match !self.buffers.send().is_empty() {
        true => iced::Subscription::batch(vec![
          iced::time::every(Duration::from_millis(1)).map(|_| Message::PortSend),
          iced::time::every(Duration::from_millis(1)).map(|_| Message::PortReceive),
        ]),
        false => iced::time::every(Duration::from_millis(1)).map(|_| Message::PortReceive),
      },
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
    let profile_active = match (&self.pages, &self.keypad.is_open, &self.profile_write) {
      (Pages::Profiles, true, false) => {
        iced::time::every(Duration::from_millis(250)).map(|_| Message::ProfileRequestActiveNum)
      }
      _ => Subscription::none(),
    };

    // Таймер проверки таймаута режима записи комбинации
    let write_timer_check = match self.allow_write {
      true => iced::time::every(Duration::from_millis(100)).map(|_| Message::TimerWriteCheck),
      false => Subscription::none(),
    };

    let stick_calibrate_timer = match self.stick_callibrate_time {
      Some(_) => iced::time::every(Duration::from_millis(100)).map(|_| Message::StickCalibrated),
      None => Subscription::none(),
    };

    Subscription::batch(vec![
      port_sub,
      window,
      keyboard,
      profile_active,
      write_timer_check,
      stick_calibrate_timer,
    ])
  }
}
