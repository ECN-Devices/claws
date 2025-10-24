//! Двунаправленные буферы обмена с устройством.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::hardware::commands::{KeypadCommands, Value};

/// Очередь исходящих пакетов к устройству
#[derive(Debug, Clone, Default)]
pub struct Send {
  buffer: VecDeque<Vec<u8>>,
}
/// Очередь входящих пакетов от устройства
#[derive(Debug, Clone, Default)]
pub struct Receive {
  buffer: VecDeque<Vec<u8>>,
}

/// Объединяет очереди отправки и приёма и делает их потокобезопасными
#[derive(Debug, Clone, Default)]
pub struct Buffers {
  send: Arc<Mutex<Send>>,
  receive: Arc<Mutex<Receive>>,
}

impl Buffers {
  /// Возвращает MutexGuard на очередь отправки
  pub fn send(&self) -> MutexGuard<'_, Send> {
    self.send.lock().unwrap()
  }

  /// Возвращает MutexGuard на очередь приёма
  pub fn receive(&self) -> MutexGuard<'_, Receive> {
    self.receive.lock().unwrap()
  }
}

impl Send {
  /// Извлекает первый пакет из очереди отправки
  pub fn pull(&mut self) -> Option<Vec<u8>> {
    match self.buffer.is_empty() {
      true => None,
      false => self.buffer.pop_front(),
    }
  }
}
impl Receive {
  /// Извлекает первый пакет, соответствующий команде `command`
  pub fn pull(&mut self, command: &KeypadCommands) -> Option<Vec<u8>> {
    self
      .buffer
      .iter()
      .position(|data| data[0] == command.get()[0])
      .and_then(|i| self.buffer.remove(i))
  }
}

/// Унифицированный интерфейс добавления пакетов в очереди
pub trait BuffersIO {
  // fn len(&self) -> usize;
  /// Добавляет пакет в очередь
  fn push(&mut self, data: Vec<u8>);
}
impl BuffersIO for Send {
  // fn len(&self) -> usize {
  //   self.buffer.len()
  // }

  fn push(&mut self, data: Vec<u8>) {
    self.buffer.push_back(data);
  }
}
impl BuffersIO for Receive {
  // fn len(&self) -> usize {
  //   self.buffer.len()
  // }

  fn push(&mut self, data: Vec<u8>) {
    self.buffer.push_back(data)
  }
}
