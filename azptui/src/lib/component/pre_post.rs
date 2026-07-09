use crate::{component::context::ComponentContext, state::STATE};
use log::info;

pub fn pre_hooks(name: &'static str) -> ComponentContext {
    // TODO store this between renders
    let mut context = ComponentContext::new(name);

    let g_count = STATE.with_borrow_mut(|s| s.increment());
    let c_count = context.increment();

    info!("{}() PRE  | G: {}, C: {},", name, g_count, c_count);
    context
}

pub fn post_hooks(context: ComponentContext) {
    let g_count = STATE.with_borrow(|s| s.counter());

    info!(
        "{}() POST | G: {}, C: {},",
        context.name(),
        g_count,
        context.counter()
    );
}
