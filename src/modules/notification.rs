use notify_rust::Notification;

pub fn send_notification(summary: &str, body: &str) {
    if let Err(e) = Notification::new()
        .summary(summary)
        .body(body)
        .show() {
        eprintln!("Notification Fail: {}", e);
    }
}
