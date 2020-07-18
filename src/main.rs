use std::env::args;

use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use gtk::Application;

use crate::command_loader::extract_first_command;
use crate::machine_process::spawn_process;
use crate::ui::main_window::MainWindow;

mod command_loader;
mod machine_process;
mod ui;

fn main() {
    let args = args().collect::<Vec<_>>().clone();
    let app = Application::new(Some("com.ddubson.basic"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Initialization failed...");

    app.connect_activate(move |app| {
        let main_window = MainWindow::new(app);
        let first_command = extract_first_command(&args);

        let mproc_process_container = main_window.create_process_container();
        spawn_process(mproc_process_container, first_command);

        main_window.show();
    });
    app.run(&vec![]);
}
