use std::fs;

use log::error;

#[cfg(test)]
mod test_config_dir {
    use crate::{
        configuration::{
            config::{create_config_dir, get_config_dir},
            APPLICATION_NAME,
        },
        tests::config::cleanup_config,
    };
    use std::env::set_var;

    #[tokio::test]
    async fn test_get_config_dir_linux() {
        set_var("OS", "linux");

        let config_path = get_config_dir().await;
        let username = whoami::username();
        let result = format!("/home/{}/.config/{}", username, APPLICATION_NAME);

        assert_eq!(config_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn test_get_config_dir_windows() {
        set_var("OS", "windows");
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let config_path = get_config_dir().await;
        let username = whoami::username();
        let result = format!("C:/Users/{}/Documents/{}", username, APPLICATION_NAME);

        assert_eq!(config_path.display().to_string(), result)
    }

    #[tokio::test]
    async fn test_create_config_dir_linux() {
        set_var("OS", "linux");

        let config_path = get_config_dir().await;

        cleanup_config(&config_path);

        let result = create_config_dir().await;

        assert!(result.is_ok());

        cleanup_config(&config_path);
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn test_create_config_dir_windows() {
        set_var("OS", "windows");

        let config_path = get_config_dir().await;

        cleanup_config(&config_path);

        let result = create_config_dir();

        assert!(result.is_ok());

        cleanup_config(&config_path);
    }
}

#[cfg(test)]
mod test_config_file {
    use crate::{
        configuration::{
            config::{create_config_dir, create_config_file, get_config_dir, get_config_file},
            APPLICATION_NAME,
        },
        tests::config::cleanup_config,
    };
    use std::env::set_var;

    #[tokio::test]
    async fn test_get_config_file_linux() {
        set_var("OS", "linux");

        let config_file_path = get_config_file().await;
        let username = whoami::username();
        let result = format!("/home/{}/.config/{}/claws.toml", username, APPLICATION_NAME);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn test_get_config_file_windows() {
        set_var("OS", "windows");

        let config_file_path = get_config_file().await;
        let username = whoami::username();
        let result = format!(
            "C:/Users/{}/Documents/{}/claws.toml",
            username, APPLICATION_NAME
        );

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[tokio::test]
    async fn test_create_config_file_linux() {
        set_var("OS", "linux");

        let config_path = get_config_dir().await;

        cleanup_config(&config_path);

        let result = async {
            create_config_dir().await?;
            create_config_file().await
        }
        .await;

        assert!(result.is_ok());

        cleanup_config(&config_path);
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn test_create_config_file_windows() {
        set_var("OS", "windows");

        let config_path = get_config_dir().await;

        cleanup_config(&config_path);

        let result = async {
            create_config_dir().await?;
            create_config_file().await
        }
        .await;

        assert!(result.is_ok());

        cleanup_config(&config_path);
    }
}

#[allow(dead_code)]
fn cleanup_config(config_path: &std::path::Path) {
    if config_path.exists() {
        if let Err(e) = fs::remove_dir_all(config_path) {
            error!("Ошибка удаления папки конфигурации: {}", e);
        };
    }
}
