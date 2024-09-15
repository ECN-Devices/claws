#[cfg(test)]
mod test_command_to_string {
    use crate::configuration::command_to_string;
    use tokio::runtime::Builder;

    #[test]
    fn positive_numbers() {
        let array: [i16; 9] = [11, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(command_to_string(&array, array.len()));

        assert_eq!(result, "11,0,0,0,0,0,0,0,0;")
    }

    #[test]
    fn negative_numbers() {
        let array: [i16; 9] = [-11, -1, -1, -1, -1, -1, -1, -1, -1];
        let result = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(command_to_string(&array, array.len()));

        assert_eq!(result, "-11,-1,-1,-1,-1,-1,-1,-1,-1;")
    }

    #[test]
    fn mixed_numbers() {
        let array: [i16; 9] = [-11, 0, -1, 0, -1, 0, -1, 0, -1];
        let result = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(command_to_string(&array, array.len()));

        assert_eq!(result, "-11,0,-1,0,-1,0,-1,0,-1;")
    }
}

#[cfg(test)]
mod test_config_dir {
    use crate::configuration::{create_config_dir, get_config_dir};
    use std::env::set_var;
    use tokio::runtime::Builder;

    #[test]
    fn test_get_config_dir_linux() {
        set_var("OS", "linux");

        let config_file_path = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(get_config_dir());

        let username = whoami::username();
        let result = format!("/home/{}/.config/Lapa", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[test]
    fn test_get_config_dir_windows() {
        set_var("OS", "windows");

        let config_file_path = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(get_config_dir());

        let username = whoami::username();
        let result = format!("C:/Users/{}/Documents/Lapa", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[test]
    fn test_create_config_dir_linux() {
        todo!()
    }

    #[cfg(windows)]
    #[test]
    fn test_create_config_dir_windows() {
        todo!()
    }
}

#[cfg(test)]
mod test_config_file {
    use crate::configuration::{create_config_file, get_config_file};
    use std::env::set_var;
    use tokio::runtime::Builder;

    #[test]
    fn test_get_config_file_linux() {
        set_var("OS", "linux");

        let config_file_path = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(get_config_file());

        let username = whoami::username();
        let result = format!("/home/{}/.config/Lapa/lapa.toml", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[test]
    fn test_get_config_file_windows() {
        set_var("OS", "windows");

        let config_file_path = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(get_config_file());

        let username = whoami::username();
        let result = format!("C:/Users/{}/Documents/Lapa/lapa.toml", username);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    fn test_create_config_file() {
        todo!()
    }
}
