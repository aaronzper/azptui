use std::{
    any::Any,
    cell::Cell,
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::{component::ComponentLocation, events::EventHandler};

#[derive(Clone)]
pub struct ComponentData {
    pub(crate) dirty: Rc<Cell<bool>>,
    pub(crate) state: HashMap<ComponentLocation, Rc<RefCell<dyn Any>>>,
    pub(crate) event_handlers: HashMap<ComponentLocation, Rc<EventHandler>>,
    pub(crate) location: ComponentLocation,
}

impl ComponentData {
    pub fn new(location: ComponentLocation) -> Self {
        Self {
            dirty: Rc::new(Cell::new(true)),
            state: HashMap::new(),
            event_handlers: HashMap::new(),
            location,
        }
    }

    pub fn location(&self) -> ComponentLocation {
        self.location
    }

    pub fn dirty(&self) -> bool {
        self.dirty.get()
    }
}
