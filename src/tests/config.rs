#[cfg(test)]
mod test_config_dir {
    use std::env::set_var;
    use tokio::runtime::Builder;

    use crate::configuration::config::get_config_dir;

    #[test]
    fn test_get_config_dir_linux() {
        set_var("OS", "linux");

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let config_file_path = runtime.block_on(get_config_dir());

        let username = whoami::username();
        let result = format!("/home/{}/.config/Lapa", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[test]
    fn test_get_config_dir_windows() {
        set_var("OS", "windows");
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let config_file_path = runtime.block_on(get_config_dir());

        let username = whoami::username();
        let result = format!("C:/Users/{}/Documents/Lapa", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    // #[test]
    fn test_create_config_dir_linux() {
        todo!()
    }

    #[cfg(windows)]
    // #[test]
    fn test_create_config_dir_windows() {
        todo!()
    }
}

#[cfg(test)]
mod test_config_file {

    use std::env::set_var;
    use tokio::runtime::Builder;

    use crate::configuration::config::get_config_file;

    #[test]
    fn test_get_config_file_linux() {
        set_var("OS", "linux");
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let config_file_path = runtime.block_on(get_config_file());

        let username = whoami::username();
        let result = format!("/home/{}/.config/Lapa/lapa.toml", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[test]
    fn test_get_config_file_windows() {
        set_var("OS", "windows");

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let config_file_path = runtime.block_on(get_config_file());

        let username = whoami::username();
        let result = format!("C:/Users/{}/Documents/Lapa/lapa.toml", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    fn test_create_config_file() {
        todo!()
    }
}
