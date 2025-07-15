#![allow(dead_code)]
#[derive(Debug, Clone)]
pub enum KeypadCommands {
  Stick(CommandStick),
  Swtich(CommandSwitch),
  Profile(CommandProfile),
  Device(CommandDevice),
  Empty(CommandEmpty),
}

#[derive(Debug, Clone)]
pub enum CommandStick {
  /**
  Запрос: 0x73, 0x1, 0x1, 0x65

  Ответ: 0x73, 0x5, 0x1, (2 байта координаты x), (2 байта координаты y) 0x65
  */
  RequestPositionXY,

  /**
  Запрос: 0x73, 0x1, 0x3, 0x65

  Ответ: 0x73, 0x5, 0x3, (код "Вверх"), (код "Вправо"), (код "Вниз"), (код "Влево"), 0x65
  */
  RequestPositionASCII,

  /**
  Запрос: 0x73, 0x2 или 0x3, 0x4, (Код параметра), (1-2 байта значения параметра), 0x65

  Ответ: нет ответа
  Коды параметров:

    1 - центр оси x (Значение АЦП) (2 байта) (Не рекомендуется менять вручную)
    2 - центр оси y (Значение АЦП) (2 байта) (Не рекомендуется менять вручную)
    3 - радиус физической мертвой зоны (Значение АЦП) (2 байта) (Не рекомендуется менять вручную)
    4 - виртуальная внутренняя мертвая зона (Процент от физической мертвой зоны от 1 до 100) (1 байт)
  */
  SetParameters,

  /**
  Запрос: 0x73, 0x3, 0x5, (код положения стика), (ascii код), 0x65

  Ответ: нет ответа

  Коды положения стика:

      1 - вверх
      2 - влево
      3 - вниз
      4 - вправо
  */
  SetPositionASCII(u8, u8),

  /**
  Запрос: 0x73, 0x2, 0x6, (код опции), 0x65

  Ответ опции 1: 0x73, 0x9, 0x6, (код опции), (2 байта центр x), (2 байта центр y), (2 байта внешняя мёртвая зона), (внутренняя мёртвая зона)

  Ответ опции 2: нет ответа

  Коды опций:

      1 - запрос параметров стика
      2 - автоматическая калибровка параметров стика

  **WARN**: Внешняя мертвая зона (или физическая мертвая зона) измеряется в единицах АЦП от центра до крайнего положения стика. Внутренняя мертвая зона (или виртуальная мертвая зона) измеряется в процентах от внешней мертвой зоны, не изменяется при калибровке, задается пользователем.
  */
  Calibration(u8),
}
#[derive(Debug, Clone)]
pub enum CommandSwitch {
  /**
  Запрос: 0x73, 0x2, 0x7, (номер переключателя), 0x65

  Ответ: 0x73, 0x3, 0x7, (код переключателя),(состояние переключателя), 0x65

  Переключатели нумеруются по порядку сверху вниз, слева на право

  Состояние переключателя:

      0 - не нажат
      1 - нажат

  Кейпад имеет 16 переключателей
  */
  RequestCondition(u8),

  /**
  Запрос: 0x73, 0x2, 0x8, (номер переключателя), 0x65

  Ответ: 0x73, 0x8, 0x8, (номер переключателя), (код 1), (код 2), (код 3),(код 4),(код 5),(код 6), 0x65

  Кейпад имеет 16 переключателей
  */
  RequestCodeASCII(u8),

  /**
  Запрос: 0x73, 0x8, 0x9, (номер переключателя), (код 1), (код 2), (код 3),(код 4),(код 5),(код 6), 0x65

  Ответ: нет ответа

  Кейпад имеет 16 переключателей
  */
  SetCodeASCII(u8, [u8; 6]),
}
#[derive(Debug, Clone)]
pub enum CommandProfile {
  /**
  Запрос: 0x73, 0x1, 0x10, 0x65

  Ответ: 0x73, 0x2, 0x10, (номер профиля), 0x65
  */
  RequestActiveNum,

  /**
  Запрос: 0x73, 0x1, 0x11, 0x65

  Ответ: 0x73, 0x16, 0x11, (15 байт названия) , 0x65

  Название должно иметь длину 15 байт. Если название короче 15 символов, не использованные байты заполнятются нулями
  */
  RequestName,

  /**
  Запрос: 0x73, 0x16, 0x12, (15 байт названия) , 0x65

  Ответ: нет ответа

  Название должно иметь длину 15 байт. Если название короче 15 символов, не использованные байты заполняются нулями
  */
  SetName([u8; 15]),

  /**
  Запрос: 0x73, 0x2, 0x13, (номер профиля ОЗУ) , 0x65

  Ответ: нет ответа

  Кейпад имеет 4 профиля
  */
  WriteActiveToRam(u8),

  /**
  Запрос: 0x73, 0x2, 0x14, (номер профиля ПЗУ) , 0x65

  Ответ: нет ответа
  */
  WriteActiveToFlash(u8),

  /**
  Запрос: 0x73, 0x2, 0x15, (номер профиля ram) , 0x65

  Ответ: нет ответа

  Кейпад имеет 4 профиля
  */
  LoadRamToActive(u8),

  /**
  Запрос: 0x73, 0x1, 0x16, 0x65

  Загружает все профили из flash в ram. В следствии этого все профили в ram, а также активный профиль, будут перезаписаны.
  */
  LoadProfilesFlashToRam,
}
#[derive(Debug, Clone)]
pub enum CommandDevice {
  /**
  Запрос: 0x73, 0x1, 0x17, 0x65

  Ответ: 0x73, 0x9, 0x17, (название устройства),(количество доступных для настройки переключателей),(2 байта серийный номер),(2 байта год изготовления),(2 байта версия прошивки) , 0x65
  */
  RequestInfo,

  /**
  Запрос: 0x73, 0x5, 0x18, (2 байта серийный номер),(2 байта год изготовления) , 0x65

  Ответ: нет ответа
  */
  WriteInfo(u16, u16),
}
#[derive(Debug, Clone)]
pub enum CommandEmpty {
  /**
  Запрос: 0x73, 0x1, 0x101, 0x65

  Ответ: 0x73, 0x1, 0x101, 0x65
  */
  VoidRequest,
}

pub trait Value {
  fn get(&self) -> Vec<u8>;
}

impl Value for KeypadCommands {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::Stick(command_stick) => command_stick.get(),
      Self::Swtich(command_switch) => command_switch.get(),
      Self::Profile(command_profile) => command_profile.get(),
      Self::Device(command_device) => command_device.get(),
      Self::Empty(command_empty) => command_empty.get(),
    }
  }
}

impl Value for CommandStick {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestPositionXY => vec![1],
      Self::RequestPositionASCII => vec![3],
      Self::SetParameters => vec![3],
      Self::SetPositionASCII(position, ascii_code) => vec![5, *position, *ascii_code],
      Self::Calibration(option) => vec![6, *option],
    }
  }
}
impl Value for CommandSwitch {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestCondition(num) => vec![7, *num],
      Self::RequestCodeASCII(num) => vec![8, *num],
      Self::SetCodeASCII(num, buttons_code) => {
        let mut result = vec![9, *num];
        for code in buttons_code {
          result.push(*code)
        }
        result
      }
    }
  }
}
impl Value for CommandProfile {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestActiveNum => vec![10],
      Self::RequestName => vec![11],
      Self::SetName(profile_name) => {
        let mut result = vec![12];
        for e in profile_name {
          result.push(*e)
        }
        result
      }
      Self::WriteActiveToRam(num) => vec![13, *num],
      Self::WriteActiveToFlash(num) => vec![14, *num],
      Self::LoadRamToActive(num) => vec![15, *num],
      Self::LoadProfilesFlashToRam => vec![16],
    }
  }
}
impl Value for CommandDevice {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::RequestInfo => vec![17],
      Self::WriteInfo(serial_number, year) => {
        vec![
          18,
          (*serial_number >> 8) as u8,
          *serial_number as u8,
          (*year >> 8) as u8,
          *year as u8,
        ]
      }
    }
  }
}
impl Value for CommandEmpty {
  fn get(&self) -> Vec<u8> {
    match self {
      Self::VoidRequest => vec![101],
    }
  }
}
