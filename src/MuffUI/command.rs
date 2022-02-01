#![allow(non_snake_case)]
use crate::muffui::Action;

pub struct Command<T> {
    action: Box<dyn for<'a> FnMut(T)>,
}

impl<T> Action<T> for Command<T> {
    fn exec(&mut self, arg: T) {
        (self.action)(arg)
    }
}

impl<T> Command<T> {
    pub fn new(action: impl FnMut(T) + 'static) -> Self {
        Self {
            action: Box::new(action)
        }
    }
}

impl<T, F: 'static + FnMut(T)> From<F> for Command<T> {
    fn from(f: F) -> Self {
        Command::new(f)
    }
}
