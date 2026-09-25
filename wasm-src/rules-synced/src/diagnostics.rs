pub const LOG_SECTION: &str = "gravitas-wasm";

pub fn info(message: &str) {
    let _ = spring::log(LOG_SECTION, spring::log_level::INFO, message);
}

/// Log a failed engine call whose failure breaks gameplay (an order, a
/// move-control switch). `what` says which call, for which unit.
pub fn check<T>(result: spring::Result<T>, what: impl FnOnce() -> alloc::string::String) {
    if let Err(error) = result {
        spring::log_warning!(LOG_SECTION, "{} failed (error {})", what(), error.code);
    }
}
