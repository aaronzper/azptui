use std::{any::Any, rc::Rc};

use crate::component::{ComponentLocation, data::ComponentData};
use crate::events::EventHandler;
use crossterm::event::Event;

pub struct RenderContext {
    pub data: ComponentData,
}

impl RenderContext {
    pub fn use_state<T>(
        &mut self,
        loc: ComponentLocation,
        initial_val: T,
    ) -> (T, impl Fn(T) + use<T>)
    where
        T: Any + Clone,
    {
        let state = self.data.state.entry(loc).or_insert_with(|| {
            Rc::new(std::cell::RefCell::new(initial_val))
                as Rc<std::cell::RefCell<dyn Any>>
        });

        let rc = Rc::clone(state);
        let rc_setter = Rc::clone(&rc);

        let dirty_setter = Rc::clone(&self.data.dirty);

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
        self.data.event_handlers.insert(loc, h);
    }
}
