mod context;
mod lifecycle;

use crate::component::context::ComponentContext;
pub use lifecycle::post_render;
pub use lifecycle::pre_render;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::{cell::RefCell, collections::HashMap, panic::Location};

thread_local! {
    static COMPONENTS: RefCell<HashMap<&'static Location<'static>, ComponentContext>> = RefCell::new(HashMap::new());
}

pub type ComponentLocation = &'static Location<'static>;

pub fn hash_location(loc: ComponentLocation) -> u16 {
    let mut h = DefaultHasher::new();
    loc.hash(&mut h);
    h.finish() as u16 // trunc for readability
}

pub fn get_components() -> Vec<ComponentContext> {
    COMPONENTS.with_borrow(|c| c.values().cloned().collect())
}
