use glib::random_int;

pub struct ProcessOutputHandler {
    pub output_capture_file_name: String,
}

impl ProcessOutputHandler {
    pub fn new(output_capture_file_name: String) -> Self {
        ProcessOutputHandler {
            output_capture_file_name,
        }
    }

    pub fn assign_output_capture_file() -> String {
        format!("proc-{}.out", random_int())
    }
}
