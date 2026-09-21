const LOG_SECTION: &str = "gravitas-ui";
const LOG_INFO: i32 = 30;
const LOG_WARNING: i32 = 40;

pub fn info(message: &str) {
    let _ = spring::log(LOG_SECTION, LOG_INFO, message);
}

pub fn warning(message: &str) {
    let _ = spring::log(LOG_SECTION, LOG_WARNING, message);
}
