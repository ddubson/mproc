use gtk::{Application};
use gio::ApplicationExt;
use gio::prelude::ApplicationExtManual;

mod main_scene;

fn main() {
    let app = Application::new(
        Some("com.ddubson.basic"),
        gio::ApplicationFlags::FLAGS_NONE,
    ).expect("Initialization failed...");

    app.connect_activate(|app| main_scene::on_window_activate(app));
    app.run(&vec![]);
}