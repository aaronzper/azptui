use std::{collections::HashMap, rc::Rc};

use crate::{component::ComponentLocation, events::EventHandler};
use crossterm::event::Event;
use log::warn;

pub struct ComponentContext {
    dirty: bool,
    event_handlers: HashMap<ComponentLocation, Rc<EventHandler>>,
    location: ComponentLocation,
    counter: usize,
}

impl ComponentContext {
    pub fn new(location: ComponentLocation) -> Self {
        Self {
            dirty: true,
            event_handlers: HashMap::new(),
            location,
            counter: 0,
        }
    }

    pub fn location(&self) -> ComponentLocation {
        self.location
    }

    pub fn counter(&self) -> usize {
        self.counter
    }

    pub fn increment(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }

    pub fn register_handler<F, H>(
        &mut self,
        loc: ComponentLocation,
        filter: F,
        handler: H,
    ) where
        F: Fn(&Event) -> bool + 'static,
        H: Fn(Event) -> () + 'static,
    {
        if self.event_handlers.contains_key(&loc) {
            return;
        }

        let h = EventHandler::register(filter, handler);
        self.event_handlers.insert(loc, h);
    }
}
