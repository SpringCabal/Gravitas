pub const LOG_SECTION: &str = "gravitas-ui";

pub fn info(spring: &spring::Spring<'_, spring::Ui>, message: &str) {
    let _ = spring
        .messages()
        .log(LOG_SECTION, spring::log_level::INFO, message);
}
