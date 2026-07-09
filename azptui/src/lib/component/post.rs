use log::info;

use crate::state::STATE;

pub fn post_hooks() {
    let count = STATE.with_borrow(|s| s.counter());

    info!("Running POST hook (#{})!", count);
}
