use crate::core::command_loader::extract_all_commands;
use crate::core::session::MprocSession;
use crate::core::settings::AppSettings;
use crate::core::spawn_process::spawn_process;
use crate::core::state::State;
use crate::ui::gtk::styles_loader::initialize_styles;
use crate::ui::main_window::MainWindow;
use gio::ApplicationExt;
use gtk::{Application, ApplicationWindow, GtkWindowExt};
use log::{debug, info};
use std::rc::Rc;

pub struct MprocApplication {
    state: Rc<State>,
    settings: AppSettings,
    main_window: MainWindow,
    gtk_application: gtk::Application,
}

impl MprocApplication {
    pub fn create(gtk_app: &Application) -> Self {
        MprocApplication {
            state: Rc::new(State::new()),
            settings: AppSettings::default(),
            main_window: MainWindow::new(gtk_app),
            gtk_application: gtk_app.clone(),
        }
    }

    pub fn start(&mut self, args: &Vec<String>) {
        info!("Starting application.");
        initialize_styles();
        debug!("Reading .mproc configuration.");
        MprocSession::first_application_run_setup(&self.settings.sessions_directory);

        let commands = extract_all_commands(&args, self.settings.process_limit);

        self.main_window
            .on_exit_button_clicked(move |app_window: &ApplicationWindow| {
                app_window.close();
            });
        self.on_application_exit();

        let state = self.state.clone();
        let settings = self.settings.clone();
        commands.iter().for_each(|command| {
            let mproc_process_container = self.main_window.create_process_container();
            spawn_process(mproc_process_container, command.clone(), &state, &settings);
        });

        self.main_window.show();
        info!("Application started.")
    }

    fn on_application_exit(&self) {
        let state = self.state.clone();
        self.gtk_application.connect_shutdown(move |_| {
            info!("Shutting down...");
            state.kill_all_processes_gracefully();
            info!("mproc stopped. Goodbye!")
        });
    }
}
