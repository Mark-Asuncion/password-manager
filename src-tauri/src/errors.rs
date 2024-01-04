use tauri::api::dialog::{MessageDialogBuilder, MessageDialogKind};

pub fn show_error(description: &str) {
    MessageDialogBuilder::new("Error", description)
        .kind(MessageDialogKind::Error)
        .show(|_| {});
}

pub fn json_error(description: &str) -> String {
    format!("{{\"error\":\"{}\"}}", description)
}
