use alloc::format;

use crate::diagnostics;
use crate::state::Global;

/// Log which walls levelled terrain, once the level has settled.
pub fn report_levelled(global: &Global, frame: i32) {
    if frame != SETTLE_FRAME {
        return;
    }
    let report = global.terrain.access().levelled_report();
    diagnostics::info(&format!("gravitas: terrain levelled {report}"));
}

const SETTLE_FRAME: i32 = 30;
