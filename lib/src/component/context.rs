use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashMap,
    ops::DerefMut,
    rc::Rc,
};

use crate::{component::ComponentLocation, events::EventHandler};
use crossterm::event::Event;

pub struct ComponentContext {
    dirty: Rc<Cell<bool>>,
    state: HashMap<ComponentLocation, Rc<RefCell<dyn Any>>>,
    event_handlers: HashMap<ComponentLocation, Rc<EventHandler>>,
    location: ComponentLocation,
}

impl ComponentContext {
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

    pub fn use_state<T>(
        &mut self,
        loc: ComponentLocation,
        initial_val: T,
    ) -> (T, impl Fn(T) + use<T>)
    where
        T: Any + Clone,
    {
        let state = self.state.entry(loc).or_insert_with(|| {
            Rc::new(RefCell::new(initial_val)) as Rc<RefCell<dyn Any>>
        });

        let rc = Rc::clone(state);
        let rc_setter = Rc::clone(&rc);

        let dirty_setter = Rc::clone(&self.dirty);

        let value = rc.borrow().downcast_ref::<T>().unwrap().clone();
        let setter = move |new_val| {
            *rc_setter.borrow_mut().downcast_mut().unwrap() = new_val;
            dirty_setter.set(true);
        };
        (value, setter)
    }

    pub fn register_handler<F, H>(
        &mut self,
        loc: ComponentLocation,
        filter: F,
        handler: H,
    ) where
        F: Fn(&Event) -> bool + 'static,
        H: Fn(&Event) -> () + 'static,
    {
        let h = EventHandler::register(filter, handler);
        self.event_handlers.insert(loc, h);
    }
}
