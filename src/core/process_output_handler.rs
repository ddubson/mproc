use log::{debug, error};
use rand::Rng;
use std::fs;
use std::path::Path;

pub struct ProcessOutputHandler {
    pub output_capture_file_name: String,
}

impl ProcessOutputHandler {
    pub fn new(sessions_directory: &String) -> Self {
        let output_capture_file_name =
            ProcessOutputHandler::assign_output_capture_file(sessions_directory);
        ProcessOutputHandler {
            output_capture_file_name,
        }
    }

    pub fn remove_output_capture_file(&self) -> () {
        match fs::remove_file(&self.output_capture_file_name) {
            Ok(()) => debug!(
                "Removed output capture file at {}",
                &self.output_capture_file_name
            ),
            _ => error!(
                "Unable to remove output capture file at {}",
                &self.output_capture_file_name
            ),
        }
    }

    fn assign_output_capture_file(sessions_directory: &String) -> String {
        Path::new(sessions_directory)
            .join(format!("proc-{}.out", rand::thread_rng().gen::<u32>()))
            .into_os_string()
            .into_string()
            .unwrap()
    }
}
