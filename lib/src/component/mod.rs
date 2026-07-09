mod context;
mod lifecycle;

use std::panic::Location;

pub use lifecycle::post_render;
pub use lifecycle::pre_render;

pub type ComponentLocation = &'static Location<'static>;
