use crate::core::command_loader::extract_all_commands;
use crate::core::session::MprocSession;
use crate::core::settings::AppSettings;
use crate::core::spawn_process::spawn_process;
use crate::core::state::State;
use crate::ui::gtk::styles_loader::initialize_styles;
use crate::ui::main_window::MainWindow;
use gtk::{Application, ApplicationWindow, GtkWindowExt};
use log::{debug, info};
use std::rc::Rc;

pub struct MprocApplication {
    state: Rc<State>,
    settings: AppSettings,
    main_window: MainWindow,
}

impl MprocApplication {
    pub fn create(gtk_app: &Application) -> Self {
        MprocApplication {
            state: Rc::new(State::new()),
            settings: AppSettings::default(),
            main_window: MainWindow::new(gtk_app),
        }
    }

    pub fn start(&mut self, args: &Vec<String>) {
        info!("Starting application.");
        initialize_styles();
        debug!("Reading .mproc configuration.");
        MprocSession::first_application_run_setup(&self.settings.sessions_directory);

        let commands = extract_all_commands(&args, self.settings.process_limit);

        let state_c = self.state.clone();
        self.main_window
            .on_exit_button_clicked(move |app_window: &ApplicationWindow| {
                app_window.close();
                state_c.kill_all_processes_gracefully();
                info!("mproc stopped. Goodbye!")
            });

        let state_c2 = self.state.clone();
        let settings_c = self.settings.clone();
        commands.iter().for_each(|command| {
            let mproc_process_container = self.main_window.create_process_container();
            spawn_process(
                mproc_process_container,
                command.clone(),
                &state_c2,
                &settings_c,
            );
        });

        self.main_window.show();
        info!("Application started.")
    }
}
