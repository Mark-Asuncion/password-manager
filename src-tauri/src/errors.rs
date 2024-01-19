use tauri::api::dialog::{MessageDialogBuilder, MessageDialogKind};

pub fn mutex_lock_error(what: &str) -> String {
    format!("Cannot obtain lock of {}", what)
}

pub fn show_error(description: &str) {
    MessageDialogBuilder::new("Error", description)
        .kind(MessageDialogKind::Error)
        .show(|_| {});
}

pub fn json_error(description: &str) -> String {
    format!("{{\"error\":\"{}\"}}", description)
}
