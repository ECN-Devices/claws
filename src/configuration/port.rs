#![allow(unused_doc_comments)]

use std::{
    io::{Read, Write},
    sync::Arc,
    time::Duration,
};

use lazy_static::lazy_static;
use serialport::{available_ports, new, SerialPort};
use tokio::sync::Mutex;

use super::{
    command::command_to_string,
    keypad_port_commands::{EmptyCommand, Value},
    ARRAY_LEN,
};
use log::{debug, error};

#[cfg(target_os = "linux")]
use regex::Regex;

/** Структура `Keypad` представляет клавиатуру, подключенную к приложению.
 * Она содержит информацию о порте, к которому подключена клавиатура, и статус открытия порта.
 */
#[derive(Debug, Clone)]
pub struct Keypad {
    /// Используется `Arc<Mutex<>>` для обеспечения безопасного доступа из разных потоков.
    pub port: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
    pub is_open: bool,
}

/**
# Глобальный буфер для хранения сообщений

Используется для хранения строковых сообщений, полученных из порта.
*/
lazy_static! {
    #[derive(Debug)]
    pub static ref BUFFER: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

/**
# Получение содержимого буфера

Асинхронная функция для вывода содержимого буфера.
*/
pub async fn get_buffer() {
    println!("{:?}", BUFFER.lock().await);
}

impl Keypad {
    /** Асинхронная функция для обработки доступных портов.
     *
     * Эта функция принимает список портов, пытается открыть каждый из них и записать данные.
     *
     * Возвращает строку с именами успешно открытых портов.
     */
    async fn process_ports(ports: Vec<String>) -> String {
        let mut result = String::new();
        let write_data_array = EmptyCommand::Empty.get_value();
        for port_name in ports.iter() {
            debug!("Port: {}; bytes: {:?}", port_name, port_name.as_bytes());
            // Открываем порт
            let port = match new(port_name, 115_200)
                .timeout(Duration::from_millis(10))
                .open()
            {
                Ok(port) => Arc::new(Mutex::new(port)),
                Err(err) => {
                    error!("Ошибка открытия порта {}: {}", port_name, err);
                    continue;
                }
            };

            #[cfg(target_os = "windows")]
            {
                let mut port_lock = port.lock().await;
                if let Err(e) = port_lock.write_data_terminal_ready(true) {
                    error!("Ошибка при установке DTR: {}", e);
                }
            }

            // Пишем данные в порт
            match Self::write_keypad_port(port.clone(), write_data_array).await {
                Ok(_) => {
                    // Читаем данные из порта
                    match Self::read_keypad_port(port.clone()).await {
                        Ok(_) => result.push_str(port_name),
                        Err(err) => {
                            error!("Ошибка чтения из порта {}: {}", port_name, err);
                            continue;
                        }
                    };
                }
                Err(err) => {
                    error!("Ошибка записи в порт {}: {}", port_name, err);
                    continue;
                }
            };
        }
        result
    }

    /** Асинхронная функция для получения порта на Linux.
     *
     * Эта функция ищет доступные порты и фильтрует их по регулярному выражению.
     */
    #[cfg(target_os = "linux")]
    pub async fn get_keypad_port() -> String {
        let ports = available_ports().expect("No ports found!");
        let mut vec_ports_str = Vec::new();
        let regex = Regex::new(r"/dev/ttyACM\d+").unwrap();

        for port in ports.into_iter() {
            let port_name = port.port_name;
            //debug!("{}", port_name);
            vec_ports_str.push(port_name);
        }

        let filtered_ports: Vec<String> = vec_ports_str
            .into_iter()
            .filter(|port_name| regex.is_match(port_name))
            .collect();
        Self::process_ports(filtered_ports).await
    }

    /** Асинхронная функция для получения порта на Windows.
     *
     * Эта функция ищет доступные порты и фильтрует их, исключая COM1(материнская плата).
     */
    #[cfg(target_os = "windows")]
    pub async fn get_keypad_port() -> String {
        let mut vec_ports_str: Vec<String> = Vec::new();
        let ports = available_ports().expect("No ports found!");

        for port in ports.into_iter() {
            let port_name = port.port_name;
            //debug!("{}", port_name);
            vec_ports_str.push(port_name);
        }

        let filtered_ports: Vec<String> = vec_ports_str
            .into_iter()
            .filter(|port| !port.contains("COM1"))
            .collect();
        Self::process_ports(filtered_ports).await
    }

    /** Асинхронная функция для записи данных в порт.
     *
     * Эта функция принимает порт и массив данных для записи.
     *
     * Возвращает `Result`, указывающий на успех или ошибку операции.
     */
    pub async fn write_keypad_port(
        port: Arc<Mutex<Box<dyn SerialPort>>>,
        write_data_array: [u16; ARRAY_LEN],
    ) -> Result<(), serialport::Error> {
        let write_data = command_to_string(&write_data_array).await;
        let mut port_lock = port.lock().await; // Ожидаем получения блокировки

        if let Err(e) = port_lock.write_all(write_data.as_bytes()) {
            error!("Failed to write to port: {}", e);
        };

        port_lock.flush()?;

        debug!(
            "Write data: {}, bytes: {:?}",
            write_data,
            write_data.as_bytes()
        );

        Ok(())
    }

    /**
    # Чтение данных из последовательного порта

    Асинхронная функция для чтения данных из заданного последовательного порта.

    # Параметры
    - `port`: Arc<Mutex<Box<dyn SerialPort>>>, содержащая заблокированный доступ к последовательному порту.

    # Возвращаемое значение
    Возвращает результат в виде строки, содержащей прочитанные сообщения, или ошибку.
    */
    pub async fn read_keypad_port(
        port: Arc<Mutex<Box<dyn SerialPort>>>,
    ) -> Result<String, serialport::Error> {
        // Создаем буфер для чтения данных из порта
        let mut serial_buf = vec![0; 1024];
        // Блокируем доступ к порту для безопасного чтения
        let mut port_lock = port.lock().await;

        // Проверяем, сколько байтов доступно для чтения
        let available_bytes = port_lock.bytes_to_read()?;
        if available_bytes == 0 {
            return Ok(String::new()); // Если нет доступных байтов, возвращаем пустую строку
        }

        // Читаем данные из порта
        let n = port_lock.read(serial_buf.as_mut_slice())?;
        if n == 0 {
            return Ok(String::new()); // Если ничего не прочитано, возвращаем пустую строку
        }

        // Обрабатываем прочитанные данные и получаем сообщения
        let messages = Self::process_data(&serial_buf[..n]).await;
        // Сохраняем сообщения в буфер
        Self::save_messages(&messages).await;

        Ok(messages) // Возвращаем обработанные сообщения
    }

    /**
    # Обработка прочитанных данных
    Асинхронная функция для обработки данных, полученных из порта.

    # Параметры
    - `data`: Срез байтов, содержащий прочитанные данные.

    # Возвращаемое значение
    Возвращает строку, содержащую обработанные сообщения.
    */
    async fn process_data(data: &[u8]) -> String {
        // Преобразуем данные в строку
        let buf = String::from_utf8_lossy(data);
        let mut messages = String::new(); // Для хранения сообщений
        let mut complete_data = buf.to_string(); // Полные данные для обработки

        // Извлекаем сообщения, разделенные символом ';'
        while let Some(pos) = complete_data.find(';') {
            let message = complete_data[..pos + 1].to_string(); // Извлекаем сообщение
            if !message.trim().is_empty() {
                messages.push_str(&message); // Добавляем непустое сообщение в результат
            }
            complete_data = complete_data[pos + 1..].to_string(); // Обновляем полные данные
        }

        // Обрабатываем оставшиеся данные, если они есть
        //if !complete_data.is_empty() {
        //    let trimmed_data = complete_data.trim_end_matches(|c: char| c.is_control()); // Удаляем управляющие символы
        //    if !trimmed_data.is_empty() {
        //        messages.push_str(trimmed_data); // Добавляем оставшиеся данные, если они не пустые
        //    }
        //}

        messages // Возвращаем все обработанные сообщения
    }

    /**
    # Сохранение сообщений в буфер

    Асинхронная функция для сохранения сообщений в глобальный буфер.

    # Параметры
    - `messages`: Строка, содержащая сообщения для сохранения.
    */
    async fn save_messages(messages: &str) {
        if !messages.is_empty() {
            // Блокируем доступ к глобальному буферу
            let mut buffet_state = BUFFER.lock().await;
            buffet_state.push(messages.to_string()); // Сохраняем сообщения в буфер
            println!("{:?}", buffet_state);
        }
    }
}
