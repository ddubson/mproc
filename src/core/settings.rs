use log::{debug, error};
use std::process::exit;

pub static DEFAULT_CONFIG_FILE_NAME: &str = ".mproc.yml";

pub struct AppSettings {
    pub process_limit: usize,
    pub mproc_working_directory: String,
    pub sessions_directory: String,
}

impl AppSettings {
    pub fn default() -> Self {
        let working_directory = match dirs::home_dir() {
            Some(home) => home.into_os_string().into_string().unwrap(),
            None => {
                error!("Cannot determine HOME directory.");
                exit(1);
            }
        };

        let mproc_working_directory = format!("{}/{}", working_directory, ".mproc");
        let sessions_directory = format!("{}/sessions", mproc_working_directory);
        debug!("Working session directory: {}", &sessions_directory);

        AppSettings {
            process_limit: 4,
            mproc_working_directory,
            sessions_directory,
        }
    }
}
