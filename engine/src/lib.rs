use std::sync::{Arc, Mutex};

pub mod graphics;
mod input;
mod api;

pub struct Shared<T>(pub Arc<Mutex<T>>);

impl<T> Shared<T> {
    pub fn new(t: T) -> Self {
        Self(Arc::new(Mutex::new(t)))
    }

    pub fn get(&self) -> std::sync::MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Shared(self.0.clone())
    }
}
