mod machine_process;
mod main_scene;
mod process_container;
mod command_loader;

use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use gtk::Application;

use std::fs::{read_to_string};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let commands_file_path = &args.get(1).expect(
        "Please provide a path to CommandFile!"
    );

    let contents = read_to_string(commands_file_path)
        .expect("Something went wrong reading the file.");

    let commands = command_loader::get_commands(&contents).expect(
        "Unable to read commands!"
    );

    let app = Application::new(
        Some("com.ddubson.basic"),
        gio::ApplicationFlags::FLAGS_NONE,
    ).expect("Initialization failed...");

    app.connect_activate(|app| main_scene::on_window_activate(app, &commands));
    app.run(&vec![]);
}
