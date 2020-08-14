mod file_watcher;
mod state;

use std::env::args;

use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use glib::clone;
use gtk::{Application, ButtonExt, CssProviderExt, GtkWindowExt};

use crate::command_loader::extract_all_commands;
use crate::spawn_process::spawn_process;
use crate::state::State;
use crate::ui::main_window::MainWindow;
use std::rc::Rc;

mod command_loader;
mod spawn_process;
mod ui;

fn main() {
    let args = args().collect::<Vec<_>>().clone();
    let app = Application::new(Some("com.ddubson.basic"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Initialization failed...");

    app.connect_activate(move |app| {
        initialize_styles();
        let main_window: MainWindow = MainWindow::new(app);
        let commands = extract_all_commands(&args, 4);
        let state = Rc::new(State::new());

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
    });
    app.run(&vec![]);
}

fn initialize_styles() {
    let style = include_str!("../styles/styles.css");

    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(style.as_bytes())
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
