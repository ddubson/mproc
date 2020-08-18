mod core;
mod file_watcher;
mod process_output_handler;

extern crate log;
extern crate simple_logger;

use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use gtk::{Application, ApplicationWindow, GtkWindowExt};
use log::{debug, info};
use std::env::args;

use crate::command_loader::extract_all_commands;
use crate::core::settings::AppSettings;
use crate::core::state::State;
use crate::spawn_process::spawn_process;
use crate::ui::main_window::MainWindow;
use std::rc::Rc;
use ui::gtk::styles_loader::initialize_styles;

mod command_loader;
mod spawn_process;
mod ui;

fn main() {
    simple_logger::init().unwrap();
    let args = args().collect::<Vec<_>>().clone();
    let app = Application::new(Some("com.ddubson.mproc"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Initialization failed...");

    app.connect_activate(move |app: &Application| {
        info!("Starting application.");
        initialize_styles();
        let main_window: MainWindow = MainWindow::new(app);
        debug!("Reading .mproc configuration.");
        let state = Rc::new(State::new());
        let settings = AppSettings::default();

        let commands = extract_all_commands(&args, settings.process_limit);

        let state_c = state.clone();
        main_window.on_exit_button_clicked(move |app_window: &ApplicationWindow| {
            app_window.close();
            state_c.kill_all_processes_gracefully();
        });

        let state_c2 = state.clone();
        commands.iter().for_each(|command| {
            let mproc_process_container = main_window.create_process_container();
            spawn_process(mproc_process_container, command.clone(), &state_c2);
        });

        main_window.show();
        info!("Application started.")
    });
    app.run(&vec![]);
}
