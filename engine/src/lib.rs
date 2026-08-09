use std::sync::{Mutex, OnceLock};

pub mod graphics;
pub mod input;

pub struct Shared<T>(pub OnceLock<Mutex<T>>);

impl<T> Shared<T> {
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    pub fn set(&self, t: T) -> Result<(), ()> {
        self.0.set(Mutex::new(t)).map_err(|_| ())
    }

    pub fn get(&self) -> std::sync::MutexGuard<'_, T> {
        self.0.get().unwrap().lock().unwrap()
    }
}
