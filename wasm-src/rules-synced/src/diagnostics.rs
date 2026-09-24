const LOG_SECTION: &str = "gravitas-wasm";
const LOG_INFO: i32 = 30;
const LOG_WARNING: i32 = 40;

pub fn info(message: &str) {
    let _ = spring::log(LOG_SECTION, LOG_INFO, message);
}

pub fn warning(message: &str) {
    let _ = spring::log(LOG_SECTION, LOG_WARNING, message);
}

/// Log a failed engine call whose failure breaks gameplay (an order, a
/// move-control switch). `what` says which call, for which unit.
pub fn check<T>(result: spring::Result<T>, what: impl FnOnce() -> alloc::string::String) {
    if let Err(error) = result {
        warning(&alloc::format!("{} failed (error {})", what(), error.code));
    }
}
