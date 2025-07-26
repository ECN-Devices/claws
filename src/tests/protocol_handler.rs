#[cfg(test)]
mod device {
  use crate::{
    hardware::{
      commands::{KeypadCommands, device},
      serial::{Keypad, ProtocolHandler},
    },
    utils::{BYTE_END, BYTE_START},
  };

  #[test]
  fn generate_command_request_info() {
    let command = KeypadCommands::Device(device::Command::RequestInfo);
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 1, 17, BYTE_END]);
  }

  #[test]
  fn generate_command_write_info() {
    let serial_number = 12u16;
    let year = 12u16;
    let command = KeypadCommands::Device(device::Command::WriteInfo(serial_number, year));
    let result = Keypad::generate_command(&command);
    assert_eq!(
      result,
      vec![
        BYTE_START,
        5,
        18,
        (serial_number >> 8) as u8,
        serial_number as u8,
        (year >> 8) as u8,
        year as u8,
        BYTE_END
      ]
    );
  }
}

#[cfg(test)]
mod empty {
  use crate::{
    hardware::{
      commands::{KeypadCommands, empty},
      serial::{Keypad, ProtocolHandler},
    },
    utils::{BYTE_END, BYTE_START},
  };

  #[test]
  fn generate_command_void_request() {
    let command = KeypadCommands::Empty(empty::Command::VoidRequest);
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 1, 101, BYTE_END]);
  }
}

#[cfg(test)]
mod profile {
  use crate::{
    hardware::{
      commands::{KeypadCommands, profile},
      serial::{Keypad, ProtocolHandler},
    },
    utils::{BYTE_END, BYTE_START},
  };

  #[test]
  fn generate_command_request_active_num() {
    let command = KeypadCommands::Profile(profile::Command::RequestActiveNum);
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 1, 10, BYTE_END]);
  }
  #[test]
  fn generate_command_request_name() {
    let command = KeypadCommands::Profile(profile::Command::RequestName);
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 1, 11, BYTE_END]);
  }
  #[test]
  fn generate_command_set_name() {
    let name = [0u8; 15];
    let command = KeypadCommands::Profile(profile::Command::SetName(name));
    let result = Keypad::generate_command(&command);
    assert_eq!(
      result,
      vec![
        BYTE_START, 16, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, BYTE_END
      ]
    );
  }
  #[test]
  fn generate_command_write_active_to_ram() {
    let num = 4;
    let command = KeypadCommands::Profile(profile::Command::WriteActiveToRam(num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 2, 13, num, BYTE_END]);
  }
  #[test]
  fn generate_command_write_active_to_flash() {
    let num = 4;
    let command = KeypadCommands::Profile(profile::Command::WriteActiveToFlash(num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 2, 14, num, BYTE_END]);
  }
  #[test]
  fn generate_command_load_ram_to_active() {
    let num = 4;
    let command = KeypadCommands::Profile(profile::Command::LoadRamToActive(num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 2, 15, num, BYTE_END]);
  }
  #[test]
  fn generate_command_load_flash_to_ram() {
    let command = KeypadCommands::Profile(profile::Command::LoadFlashToRam);
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 1, 16, BYTE_END]);
  }
}

#[cfg(test)]
mod stick {
  use crate::{
    hardware::{
      commands::{KeypadCommands, stick},
      serial::{Keypad, ProtocolHandler},
    },
    utils::{BYTE_END, BYTE_START},
  };

  #[test]
  fn generate_command_request_position_xy() {
    let command = KeypadCommands::Stick(stick::Command::RequestPositionXY);
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 1, 1, BYTE_END]);
  }
  #[test]
  fn generate_command_request_position_ascii() {
    let command = KeypadCommands::Stick(stick::Command::RequestPositionASCII);
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 1, 3, BYTE_END]);
  }
  #[test]
  fn generate_command_set_parameters() {
    let num = 10;
    let command = KeypadCommands::Stick(stick::Command::SetParameters(num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 2, 4, num, BYTE_END]);
  }
  #[test]
  fn generate_command_set_position_ascii_up() {
    let num = 10;
    let command = KeypadCommands::Stick(stick::Command::SetPositionASCII(1, num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 3, 5, 1, num, BYTE_END]);
  }
  #[test]
  fn generate_command_set_position_ascii_left() {
    let num = 10;
    let command = KeypadCommands::Stick(stick::Command::SetPositionASCII(4, num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 3, 5, 4, num, BYTE_END]);
  }
  #[test]
  fn generate_command_set_position_ascii_down() {
    let num = 10;
    let command = KeypadCommands::Stick(stick::Command::SetPositionASCII(3, num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 3, 5, 3, num, BYTE_END]);
  }
  #[test]
  fn generate_command_set_position_ascii_right() {
    let num = 10;
    let command = KeypadCommands::Stick(stick::Command::SetPositionASCII(2, num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 3, 5, 2, num, BYTE_END]);
  }
  #[test]
  fn generate_command_calibration_request() {
    let status = stick::OptionsCalibration::Request;
    let command = KeypadCommands::Stick(stick::Command::Calibration(status.clone()));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 2, 6, status.get(), BYTE_END]);
  }
  #[test]
  fn generate_command_calibration_calibrate() {
    let status = stick::OptionsCalibration::Calibrate;
    let command = KeypadCommands::Stick(stick::Command::Calibration(status.clone()));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 2, 6, status.get(), BYTE_END]);
  }
}

#[cfg(test)]
mod switch {
  use crate::{
    hardware::{
      commands::{KeypadCommands, switch},
      serial::{Keypad, ProtocolHandler},
    },
    utils::{BYTE_END, BYTE_START},
  };

  #[test]
  fn generate_command_request_condition() {
    let num = 8;
    let command = KeypadCommands::Swtich(switch::Command::RequestCondition(num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 2, 7, num, BYTE_END]);
  }
  #[test]
  fn generate_command_request_code_ascii() {
    let num = 8;
    let command = KeypadCommands::Swtich(switch::Command::RequestCodeASCII(num));
    let result = Keypad::generate_command(&command);
    assert_eq!(result, vec![BYTE_START, 2, 8, num, BYTE_END]);
  }
  #[test]
  fn generate_command_set_code_ascii() {
    let num = 8;
    let code = [0; 6];
    let command = KeypadCommands::Swtich(switch::Command::SetCodeASCII(num, code));
    let result = Keypad::generate_command(&command);
    assert_eq!(
      result,
      vec![BYTE_START, 8, 9, num, 0, 0, 0, 0, 0, 0, BYTE_END]
    );
  }
}
