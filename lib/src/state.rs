use std::cell::RefCell;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::new());
}

pub struct State {
    counter: usize,
}

impl State {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn counter(&self) -> usize {
        self.counter
    }

    pub fn increment(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
}
