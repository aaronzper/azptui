use crate::component::{
    COMPONENTS, ComponentLocation, RENDERED, context::RenderContext,
    data::ComponentData,
};

pub fn pre_render(location: ComponentLocation) -> RenderContext {
    let data = COMPONENTS.with_borrow_mut(|components| {
        if let Some(d) = components.remove(location) {
            d
        } else {
            ComponentData::new(location)
        }
    });

    RENDERED.with_borrow_mut(|r| {
        r.insert(location);
    });

    RenderContext { data }
}

pub fn post_render(context: RenderContext) {
    COMPONENTS.with_borrow_mut(|components| {
        let data = context.data;
        if components.insert(data.location(), data).is_some() {
            unreachable!()
        }
    });
}

pub fn cleanup() {
    RENDERED.with_borrow_mut(|r| {
        COMPONENTS.with_borrow_mut(|components| {
            components.retain(|loc, _| r.contains(loc));
        });
        r.clear();
    });
}
