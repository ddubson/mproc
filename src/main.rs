mod machine_process;
mod main_scene;
mod process_container;

use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use gtk::Application;

fn main() {
    let app = Application::new(Some("com.ddubson.basic"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Initialization failed...");

    app.connect_activate(|app| main_scene::on_window_activate(app));
    app.run(&vec![]);
}
