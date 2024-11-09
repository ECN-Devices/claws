#[cfg(test)]
mod test_config_dir {
    use log::error;
    use std::{env::set_var, fs};
    use tokio::runtime::Builder;

    use crate::configuration::config::{create_config_dir, get_config_dir};

    #[test]
    fn test_get_config_dir_linux() {
        set_var("OS", "linux");

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let config_path = runtime.block_on(get_config_dir());

        let username = whoami::username();
        let result = format!("/home/{}/.config/Claws", username);

        assert_eq!(config_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[test]
    fn test_get_config_dir_windows() {
        set_var("OS", "windows");
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let config_path = runtime.block_on(get_config_dir());

        let username = whoami::username();
        let result = format!("C:/Users/{}/Documents/Claws", username);

        assert_eq!(config_path.display().to_string(), result)
    }

    #[test]
    fn test_create_config_dir_linux() {
        set_var("OS", "linux");

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let config_path = runtime.block_on(get_config_dir());

        if config_path.exists() {
            if let Err(e) = fs::remove_dir_all(config_path.clone()) {
                error!("Ошибка удаления папки конфигурации: {}", e);
            };
        }

        let result = runtime.block_on(create_config_dir());

        assert!(result.is_ok());

        if let Err(e) = fs::remove_dir_all(config_path) {
            error!("Ошибка удаления папки конфигурации: {}", e);
        };
    }

    #[cfg(windows)]
    #[test]
    fn test_create_config_dir_windows() {
        set_var("OS", "windows");

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let config_path = runtime.block_on(get_config_dir());

        if let Err(e) = fs::remove_dir_all(config_path.clone()) {
            error!("Ошибка удаления папки конфигурации: {}", e);
        };

        let result = runtime.block_on(create_config_dir());

        assert!(result.is_ok());

        if let Err(e) = fs::remove_dir_all(config_path) {
            error!("Ошибка удаления папки конфигурации: {}", e);
        };
    }
}

#[cfg(test)]
mod test_config_file {

    use log::error;
    use std::{env::set_var, fs};
    use tokio::runtime::Builder;

    use crate::configuration::config::{
        create_config_dir, create_config_file, get_config_dir, get_config_file,
    };

    #[test]
    fn test_get_config_file_linux() {
        set_var("OS", "linux");
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let config_file_path = runtime.block_on(get_config_file());

        let username = whoami::username();
        let result = format!("/home/{}/.config/Claws/claws.toml", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[test]
    fn test_get_config_file_windows() {
        set_var("OS", "windows");

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let config_file_path = runtime.block_on(get_config_file());

        let username = whoami::username();
        let result = format!("C:/Users/{}/Documents/Claws/claws.toml", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[test]
    fn test_create_config_file_linux() {
        set_var("OS", "linux");

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let config_dir_path = runtime.block_on(get_config_dir());
        let config_file_path = runtime.block_on(get_config_file());
        if config_file_path.exists() {
            if let Err(e) = fs::remove_file(config_file_path.clone()) {
                error!("Ошибка удаления файла конфигурации: {}", e);
            };
        }

        let result = runtime.block_on(async {
            create_config_dir().await?;
            create_config_file().await
        });

        assert!(result.is_ok());

        if let Err(e) = fs::remove_dir_all(config_dir_path) {
            error!("Ошибка удаления файла конфигурации: {}", e);
        };
    }

    #[cfg(windows)]
    #[test]
    fn test_create_config_file_windows() {
        set_var("OS", "windows");

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let config_dir_path = runtime.block_on(get_config_dir());
        let config_file_path = runtime.block_on(get_config_file());
        if config_file_path.exists() {
            if let Err(e) = fs::remove_file(config_file_path.clone()) {
                error!("Ошибка удаления файла конфигурации: {}", e);
            };
        }

        let result = runtime.block_on(async {
            create_config_dir().await?;
            create_config_file().await
        });

        assert!(result.is_ok());

        if let Err(e) = fs::remove_dir_all(config_dir_path) {
            error!("Ошибка удаления файла конфигурации: {}", e);
        };
    }
}
