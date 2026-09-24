//! The e2e scenario's screenshot requests (only with the `e2e` feature).

#[cfg(feature = "e2e")]
pub fn handle(message: &str) {
    match message.split('|').nth(1) {
        Some("shot") => {
            let _ = spring::send_commands("screenshot", "");
        }
        Some("done") => {
            let _ = spring::send_commands("quitforce", "");
        }
        _ => {}
    }
}

#[cfg(not(feature = "e2e"))]
pub fn handle(_message: &str) {}
