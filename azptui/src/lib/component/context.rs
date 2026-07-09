pub struct ComponentContext {
    name: &'static str,
    counter: usize,
}

impl ComponentContext {
    pub fn new(name: &'static str) -> Self {
        Self { name, counter: 0 }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn counter(&self) -> usize {
        self.counter
    }

    pub fn increment(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
}
