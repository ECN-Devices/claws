[[ru](./README_ru.md)]

# Claws

A utility for managing a programmable cape: configuring profiles, assigning combinations to 16 buttons and a crosspiece/stick, importing/exporting configurations, and interacting with the device via the serial port.

## 📦 Description
Claws is a desktop application in Rust for users of programmable capads. The app helps:
- View and edit the active profile of buttons and stick;
- assign keyboard shortcuts and special codes;
- manage the "dead zone" of the stick;
- import/export profiles to a file;
- write the profile to the internal memory (RAM/ROM) of the device;
- View device information and firmware version;
- reboot the device into the bootloader to update the firmware.

## 🛠 Technology and stack
- **Language/Runtime**: Rust 2024 Edition
- **GUI**: `iced` 0.13 (widgets, svg, image, tokio, advanced)
- **Asynchrony**: `tokio` 1.x
- **Serial port**: `serialport` (with `serde`)
- **Configurations/profiles**: `serde`, `confy`
- **File Dialogs**: `rfd`
- **Logging**: `log`, `pretty_env_logger`
- **Errors**: `thiserror`, `anyhow`
- **OS Theme**: `dark-light`
- **Nix**: flake with `crane`, `rust-overlay`, `flake-utils`, `git-hooks.nix`

Links to key files:
- Manifest: `Cargo.toml`
- Entry point: `src/main.rs`
- UI and logic: `src/ui/mod.rs`, `src/ui/pages.rs`, `src/ui/styles/`
- Working with the device: `src/hardware/*` (buffers, commands, serial)
- Data and configs: `src/data/*` (profiles, device, window, file_dialog)
- Assets: `assets/` (icons, fonts)
- Build via Nix: `flake.nix`, `nix/`

## 🚀 Installation and launch
### Option 1: Cargo
Rust (including Cargo) is required to be installed.
```bash
# Clone
git clone https://git.sr.ht/~fisuri/claws
cd claws

# Build release version
cargo build --release

# Run
cargo run
```

### Option 2: Nix Flakes
If you have Nix installed with flakes enabled:
```bash
# Package build via flake
nix build
# The finished binary will be in result/bin/
```
The flake configuration looks at `nix/default.nix` and uses `crane` to build the Rust project.

## 🗂 Project structure
```
claws/
  assets/             # Application icons and fonts
  src/
    assets.rs # Application constants: name, version, font, icon
    main.rs # Entry point: Launching an Iced application and window
    ui/ # UI, posts, pages, styles
      mod.rs # Messages, update/view, subscriptions, handlers
      pages.rs # Pages
      styles/         # Styles
    data/             # Models/configs
      profiles.rs # Profile: 16 buttons × 6 codes, stick settings, I/O
      device.rs # Information about the device (version, serial number, etc.)
...
    hardware/ # Buffers and protocol commands, working with Serial
      buffers/
      commands/
      serial/
    errors/           # Errors
    utils/            # Logger and utilities
  nix/ # Build files via Nix
  flake.nix # Flake
  LICENSE # Project license
```

## ✨ Features
- **Profiles**: edit the active profile, change the name
- **16 buttons**: Quickly assign up to 6 codes per button and clear
- **Stick**: setting directions and the "dead zone"
- **RAM/ROM**: Write the active profile to the RAM/ROM of the device cell
- **Import/Export**: profile to and from file
- **Device Information**: Application and firmware version on the Update screen
- **Serial connection**: automatic port search, if unavailable — the "Device not found" page

## ❓ FAQ
- **The device is not detected by the application**
  Check the USB cable/port and access to the serial port. On Linux, udev rules may be required or a user may be added to a group that has access to tty devices - _dialout_.

- **Where are the profiles stored on the disk?**
  Profiles are saved via `confy` in the OS configuration directory (a subdirectory `profiles/` is created). The exact path depends on the platform — [confy](https://github.com/rust-cli/confy?tab=readme-ov-file#config-file-location).

- **What codes are supported for buttons/stick?**
  Standard key codes (Esc, F1–F24, modifiers, etc.) and stick directions are supported. For the display, see `src/data/profiles.rs` (`code_to_title`). The full list is in the project code.

- **Are themes (light/dark) supported?**
  The application uses `dark-light` to determine the theme of the system.

- **Is there an automatic firmware update?**
  The Update tab shows the versions, but the firmware process is performed separately.

## 📄 License
This project is licensed under the [APACHE 2.0 License](./LICENSE).

## 👤 Author
- **Yaroslav Malykh** — <fisuri@murena.io>

---

Additionally:
- Fonts: used Inter Medium (`assets/fonts/Inter-Medium.ttf`).
- Icons: placed in `assets/icons/` and embedded in the binary.
- The console is hidden under Windows when building in the release (`windows_subsystem = "windows"`).
- Logs are enabled via `pretty_env_logger` and configured by the `RUST_LOG` variable.
