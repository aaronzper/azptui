use std::{cell::RefCell, collections::HashMap, panic::Location};

use crate::component::{ComponentLocation, context::ComponentContext};
use log::info;

thread_local! {
    static CONTEXTS: RefCell<HashMap<&'static Location<'static>, ComponentContext>> = RefCell::new(HashMap::new());
}

pub fn pre_render(location: ComponentLocation) -> ComponentContext {
    let mut context = CONTEXTS.with_borrow_mut(|contexts| {
        if let Some(c) = contexts.remove(location) {
            c
        } else {
            ComponentContext::new(location)
        }
    });

    context
}

pub fn post_render(context: ComponentContext) {
    CONTEXTS.with_borrow_mut(|contexts| {
        if contexts.insert(context.location(), context).is_some() {
            unreachable!()
        }
    });
}
