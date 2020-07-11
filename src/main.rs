use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use gtk::Application;
use std::env::args;

mod command_loader;
mod machine_process;
mod main_scene;
mod process_container;
mod ui;

fn main() {
    let args = args().collect::<Vec<_>>().clone();
    let app = Application::new(Some("com.ddubson.basic"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Initialization failed...");

    app.connect_activate(move |app| main_scene::on_window_activate(app, &args));
    app.run(&vec![]);
}
