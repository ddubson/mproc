mod file_watcher;
mod state;
mod styles;

extern crate log;
extern crate simple_logger;

use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use glib::clone;
use gtk::{Application, ButtonExt, GtkWindowExt};
use log::{debug, info};
use std::env::args;

use crate::command_loader::extract_all_commands;
use crate::spawn_process::spawn_process;
use crate::state::State;
use crate::styles::initialize_styles;
use crate::ui::main_window::MainWindow;
use std::rc::Rc;

mod command_loader;
mod spawn_process;
mod ui;

fn main() {
    simple_logger::init().unwrap();
    let args = args().collect::<Vec<_>>().clone();
    let app = Application::new(Some("com.ddubson.mproc"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Initialization failed...");

    app.connect_activate(move |app| {
        info!("Starting application.");
        initialize_styles();
        let main_window: MainWindow = MainWindow::new(app);
        debug!("Reading .mproc configuration.");
        let state = Rc::new(State::new());

        let commands = extract_all_commands(&args, state.app_settings.process_limit);
        let app_window = &main_window.window;

        let state_c = state.clone();
        main_window
            .controls
            .exit_button
            .connect_clicked(clone!(@weak app_window => move |_| {
                app_window.close();
                state_c.kill_all_processes_gracefully();
            }));

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
