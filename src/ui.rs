#[cfg(feature = "ui")]
use gtk4 as gtk;

#[cfg(feature = "ui")]
pub fn toggle_popup() {
    use gtk::prelude::*;
    if gtk::is_initialized() { /* future: focus existing window */ } else {
        let _ = gtk::init();
    }
    let window = gtk::ApplicationWindow::builder()
        .title("Clipboard (Stub)")
        .default_width(400)
        .default_height(300)
        .build();
    window.present();
}

#[cfg(not(feature = "ui"))]
pub fn toggle_popup() {
    println!("UI feature disabled (rebuild with --features ui)");
}
