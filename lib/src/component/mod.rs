mod context;
mod pre_post;

use std::panic::Location;

pub use pre_post::post_hooks;
pub use pre_post::pre_hooks;

pub type ComponentLocation = &'static Location<'static>;
