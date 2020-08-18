use log::debug;
use log::info;
use std::fs;

pub struct MprocSession {}

impl MprocSession {
    pub fn first_application_run_setup(sessions_directory: &String) {
        let not_exists = fs::metadata(&sessions_directory).is_err();

        if not_exists {
            info!("First time? Creating sessions directory.");
            if let Ok(_) = fs::create_dir_all(&sessions_directory) {
                info!("Sessions directory created.");
            };
        } else {
            debug!("Working sessions directory exists.")
        }
    }
}
