pub const LOG_SECTION: &str = "gravitas-wasm";

pub fn info(spring: &spring::Spring<'_, spring::RulesSynced>, message: &str) {
    let _ = spring
        .messages()
        .log(LOG_SECTION, spring::log_level::INFO, message);
}

pub fn check<T>(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    result: spring::Result<T>,
    what: impl FnOnce() -> alloc::string::String,
) {
    if let Err(error) = result {
        spring::log_warning!(
            spring,
            LOG_SECTION,
            "{} failed (error {})",
            what(),
            error.code
        );
    }
}
