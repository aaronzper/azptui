mod context;
mod data;
mod lifecycle;

use crate::component::data::ComponentData;
pub use lifecycle::cleanup;
pub use lifecycle::post_render;
pub use lifecycle::pre_render;
use std::collections::HashSet;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::{cell::RefCell, collections::HashMap, panic::Location};

thread_local! {
    static COMPONENTS: RefCell<HashMap<ComponentLocation, ComponentData>> = RefCell::new(HashMap::new());
    static RENDERED: RefCell<HashSet<ComponentLocation>> = RefCell::new(HashSet::new());
}

pub type ComponentLocation = &'static Location<'static>;

pub fn hash_location(loc: ComponentLocation) -> u64 {
    let mut h = DefaultHasher::new();
    loc.hash(&mut h);
    h.finish()
}

pub fn get_components() -> Vec<ComponentData> {
    COMPONENTS.with_borrow(|c| c.values().cloned().collect())
}
