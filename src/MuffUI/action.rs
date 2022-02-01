#![allow(non_snake_case)]


pub trait Action<T> {
    fn exec(&mut self, arg: T);
}
