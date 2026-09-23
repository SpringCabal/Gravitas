use alloc::format;

use crate::diagnostics;

/// Log which walls levelled terrain, once the level has settled.
pub fn report_levelled(frame: i32) {
    if frame != SETTLE_FRAME {
        return;
    }
    diagnostics::info(&format!(
        "gravitas: terrain levelled {}",
        crate::terrain::levelled_report()
    ));
}

const SETTLE_FRAME: i32 = 30;
