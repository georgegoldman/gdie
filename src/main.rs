use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Entry, TextView, Box, Orientation};
use regex::Regex;
use gtk4::glib::clone;  // Use clone macro from gtk4::glib, not from glib directly



fn main() {
    let app = Application::builder()
        .application_id("com.george.driveidextractor")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let entry = Entry::new();
    entry.set_placeholder_text(Some("Paste your Google Drive URL here"));

    let button = Button::with_label("Extract ID");

    let result_view = TextView::new();
    result_view.set_editable(false);
    result_view.set_cursor_visible(false);

    let container = Box::new(Orientation::Vertical, 10);
    container.append(&entry);
    container.append(&button);
    container.append(&result_view);

    let re = Regex::new(r"/d/([a-zA-Z0-9_-]+)").unwrap();

    button.connect_clicked(clone!(@weak entry, @weak result_view => move |_| {
        let url = entry.text().to_string();
        let result = match re.captures(&url) {
            Some(caps) => format!("Extracted ID: {}", &caps[1]),
            None => "No valid ID found.".to_string(),
        };
        result_view.buffer().set_text(&result);
    }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Google Drive ID Extractor")
        .default_width(400)
        .default_height(200)
        .child(&container)
        .build();

    window.show();
}

