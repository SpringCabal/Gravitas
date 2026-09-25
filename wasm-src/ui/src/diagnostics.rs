pub const LOG_SECTION: &str = "gravitas-ui";

pub fn info(message: &str) {
    let _ = spring::log(LOG_SECTION, spring::log_level::INFO, message);
}
