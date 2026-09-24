//! The e2e scenario's screenshot requests (only with the `e2e` feature).

#[cfg(feature = "e2e")]
pub fn handle(message: &[u8]) {
    let Some(request) = message.strip_prefix(b"GRAVITAS_SHOT|") else {
        return;
    };
    if request.starts_with(b"shot") {
        let _ = spring::send_commands("screenshot", "");
    } else if request == b"done" {
        let _ = spring::send_commands("quitforce", "");
    }
}

#[cfg(not(feature = "e2e"))]
pub fn handle(_message: &[u8]) {}
