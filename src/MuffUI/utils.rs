#![allow(non_snake_case)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::hash::Hash;
use std::collections::HashMap;
use crate::muffui::SharedProps;

#[allow(dead_code)]
pub fn uniqId() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);

    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn merge<L: Hash + Eq, R: Hash + Eq>(left: HashMap<L, R>, right: HashMap<L, R>) -> HashMap<L, R> {
    left.into_iter().chain(right).collect()
}
#[allow(dead_code)]
pub fn concat<T, C: Into<Vec<T>>>(left: Vec<T>, right: C) -> Vec<T> {
    left.into_iter().chain(right.into()).collect()
}
#[allow(dead_code)]
pub fn prop<'a>(props: &'a Vec<SharedProps>, variant: &'a SharedProps) -> Option<&'a SharedProps> {
    props.iter().find_map(|d|{
        if std::mem::discriminant(d) == std::mem::discriminant(variant) {
            Some(d)
        } else {
            None
        }
    })
}
