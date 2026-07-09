use crate::{component::context::ComponentContext, state::STATE};
use log::info;

pub fn pre_hooks() -> ComponentContext {
    let mut context = ComponentContext::new(); // TODO store this between renders

    let g_count = STATE.with_borrow_mut(|s| s.increment());
    let c_count = context.increment();

    info!("PRE  | G: {}, C: {},", g_count, c_count);
    context
}

pub fn post_hooks(context: ComponentContext) {
    let g_count = STATE.with_borrow(|s| s.counter());

    info!("POST | G: {}, C: {},", g_count, context.counter());
}
