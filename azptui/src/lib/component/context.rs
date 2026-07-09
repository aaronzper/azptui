pub struct ComponentContext {
    counter: usize,
}

impl ComponentContext {
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
