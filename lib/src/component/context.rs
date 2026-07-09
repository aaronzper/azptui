use log::warn;

use crate::component::ComponentLocation;
pub struct ComponentContext {
    location: ComponentLocation,
    counter: usize,
}

impl ComponentContext {
    pub fn new(location: ComponentLocation) -> Self {
        Self {
            location,
            counter: 0,
        }
    }

    pub fn location(&self) -> ComponentLocation {
        self.location
    }

    pub fn counter(&self) -> usize {
        warn!("COUNTER");
        self.counter
    }

    pub fn increment(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
}
