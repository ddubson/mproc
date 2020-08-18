use log::info;

pub struct MprocSession {}

impl MprocSession {
    pub fn first_application_run_setup() {
        info!("First time? Creating working directory.")
    }
}
