use log::info;

use crate::component::{
    COMPONENTS, ComponentLocation, RENDERED, context::RenderContext,
    data::ComponentData, hash_location,
};

pub fn pre_render(location: ComponentLocation) -> RenderContext {
    let data = COMPONENTS.with_borrow_mut(|components| {
        if let Some(d) = components.remove(location) {
            d
        } else {
            ComponentData::new(location)
        }
    });

    let is_root = RENDERED.with_borrow_mut(|r| {
        let empty = r.is_empty();
        r.insert(location);
        empty
    });

    RenderContext { data, is_root }
}

pub fn post_render(context: RenderContext) {
    COMPONENTS.with_borrow_mut(|components| {
        let data = context.data;
        if components.insert(data.location(), data).is_some() {
            unreachable!()
        }
    });

    if context.is_root {
        RENDERED.with_borrow_mut(|r| r.clear());
    }
}
