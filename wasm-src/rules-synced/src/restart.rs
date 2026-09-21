use core::cell::Cell;

use crate::state;
use crate::terrain;

pub fn request() {
    PENDING.with(|pending| pending.set(true));
}

pub fn take() -> bool {
    PENDING.with(|pending| pending.replace(false))
}

pub fn wiping() -> bool {
    WIPING.with(|wiping| wiping.get())
}

pub fn finish_wipe() {
    WIPING.with(|wiping| wiping.set(false));
}

pub fn perform() {
    WIPING.with(|wiping| wiping.set(true));
    wipe();
    terrain::clear();
    crate::mission::clear_over();
    let _ = state::with_world(|world| world.reset());
}

thread_local! {
    static PENDING: Cell<bool> = const { Cell::new(false) };
    static WIPING: Cell<bool> = const { Cell::new(false) };
}

fn wipe() {
    let Ok(units) = spring::get_all_units() else {
        return;
    };
    for unit_id in units {
        let _ = spring::destroy_unit(
            unit_id,
            spring::generated::owned::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed: true,
                attacker_id: -1,
                recycle_id: false,
            },
        );
    }
}
