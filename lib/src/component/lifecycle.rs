use crate::component::{
    COMPONENTS, ComponentLocation, context::ComponentContext,
};

pub fn pre_render(location: ComponentLocation) -> ComponentContext {
    let mut context = COMPONENTS.with_borrow_mut(|contexts| {
        if let Some(c) = contexts.remove(location) {
            c
        } else {
            ComponentContext::new(location)
        }
    });

    context
}

pub fn post_render(context: ComponentContext) {
    COMPONENTS.with_borrow_mut(|contexts| {
        if contexts.insert(context.location(), context).is_some() {
            unreachable!()
        }
    });
}
