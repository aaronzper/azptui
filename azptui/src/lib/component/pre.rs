use log::info;

use crate::state::STATE;

pub fn pre_hooks() {
    let count = STATE.with_borrow_mut(|s| s.increment());

    info!("Running PRE hook (#{})!", count);
}
