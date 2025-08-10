use super::commands::{KeypadCommands, Value};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug, Clone, Default)]
pub struct Send {
  buffer: Vec<Vec<u8>>,
}
#[derive(Debug, Clone, Default)]
pub struct Receive {
  buffer: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Default)]
pub struct Buffers {
  send: Arc<Mutex<Send>>,
  receive: Arc<Mutex<Receive>>,
}

impl Buffers {
  pub fn send(&self) -> MutexGuard<'_, Send> {
    self.send.lock().unwrap()
  }

  pub fn receive(&self) -> MutexGuard<'_, Receive> {
    self.receive.lock().unwrap()
  }
}

impl Send {
  pub fn pull(&mut self) -> Option<Vec<u8>> {
    match self.buffer.is_empty() {
      true => None,
      false => Some(self.buffer.remove(0)),
    }
  }
}
impl Receive {
  pub fn pull(&mut self, command: &KeypadCommands) -> Option<Vec<u8>> {
    self
      .buffer
      .iter()
      .position(|data| data[0] == command.get()[0])
      .map(|i| self.buffer.remove(i))
  }
}

pub trait BuffersIO {
  fn len(&self) -> usize;
  fn push(&mut self, data: Vec<u8>);
}
impl BuffersIO for Send {
  fn len(&self) -> usize {
    self.buffer.len()
  }

  fn push(&mut self, data: Vec<u8>) {
    self.buffer.push(data);
  }
}
impl BuffersIO for Receive {
  fn len(&self) -> usize {
    self.buffer.len()
  }

  fn push(&mut self, data: Vec<u8>) {
    self.buffer.push(data)
  }
}
