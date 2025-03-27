use std::fs;

use log::error;

#[cfg(test)]
mod config_dir {
    use crate::{
        configuration::{
            APPLICATION_NAME,
            config::{create_config_dir, get_config_dir},
        },
        tests::config::cleanup_config,
    };

    #[tokio::test]
    async fn get_config_dir_linux() {
        let config_path = get_config_dir().await;
        let username = whoami::username();
        let result = format!("/home/{}/.config/{}", username, APPLICATION_NAME);

        assert_eq!(config_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn get_config_dir_windows() {
        let config_path = get_config_dir().await;
        let username = whoami::username();
        let result = format!("C:/Users/{}/Documents/{}", username, APPLICATION_NAME);

        assert_eq!(config_path.display().to_string(), result)
    }

    #[tokio::test]
    async fn create_config_dir_linux() {
        let config_path = get_config_dir().await;

        cleanup_config(&config_path);

        let result = create_config_dir().await;

        assert!(result.is_ok());

        cleanup_config(&config_path);
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn create_config_dir_windows() {
        let config_path = get_config_dir().await;

        cleanup_config(&config_path);

        let result = create_config_dir();

        assert!(result.is_ok());

        cleanup_config(&config_path);
    }
}

#[cfg(test)]
mod config_file {
    use crate::{
        configuration::{
            APPLICATION_NAME,
            config::{create_config_dir, create_config_file, get_config_dir, get_config_file},
        },
        tests::config::cleanup_config,
    };

    #[tokio::test]
    async fn get_config_file_linux() {
        let config_file_path = get_config_file().await;
        let username = whoami::username();
        let result = format!("/home/{}/.config/{}/claws.toml", username, APPLICATION_NAME);

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn get_config_file_windows() {
        let config_file_path = get_config_file().await;
        let username = whoami::username();
        let result = format!(
            "C:/Users/{}/Documents/{}/claws.toml",
            username, APPLICATION_NAME
        );

        assert_eq!(config_file_path.display().to_string(), result)
    }

    #[tokio::test]
    async fn create_config_file_linux() {
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
    async fn create_config_file_windows() {
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
