use crossterm::event::Event;
use log::{debug, error, info, trace};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

thread_local! {
    static HANDLERS: RefCell<Vec<Weak<EventHandler>>> =
        RefCell::new(Vec::new());
}

pub struct EventHandler {
    filter: Box<dyn Fn(&Event) -> bool>,
    handler: Box<dyn Fn(&Event) -> ()>,
}

impl EventHandler {
    pub fn register<F, H>(filter: F, handler: H) -> Rc<Self>
    where
        F: Fn(&Event) -> bool + 'static,
        H: Fn(&Event) -> () + 'static,
    {
        let handler = Self {
            filter: Box::new(filter),
            handler: Box::new(handler),
        };

        let rc = Rc::new(handler);
        let weak = Rc::downgrade(&rc);

        HANDLERS.with_borrow_mut(|h| h.push(weak));
        rc
    }
}

pub fn handle_event(event: Event) {
    HANDLERS.with_borrow_mut(|handlers| {
        let mut i = 0;
        while i < handlers.len() {
            match handlers[i].upgrade() {
                Some(h) if (h.filter)(&event) => {
                    (h.handler)(&event);
                }
                None => {
                    handlers.remove(i);
                    continue;
                }
                _ => (),
            };
            i += 1;
        }
    })
}
