use std::sync::atomic::{AtomicUsize};

#[derive(Debug)]
pub struct State {
    pub letter: char,
    pub atomic_id: AtomicUsize,
}
impl State {
    pub fn new() -> Self {
        Self { letter: '-', atomic_id: AtomicUsize::new(0) }
    }
}
