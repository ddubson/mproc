use crate::core::process_handler::ProcessHandler;
use log::{debug, info};
use std::cell::RefCell;

pub struct State {
    pub running_processes: RefCell<Vec<ProcessHandler>>,
}

impl State {
    pub fn new() -> Self {
        State {
            running_processes: RefCell::new(Vec::new()),
        }
    }

    pub fn add_process_handler(&self, process_handler: ProcessHandler) {
        debug!(
            "Process(es) {:?} added to state.",
            process_handler.reader_handle.pids()
        );
        self.running_processes.borrow_mut().push(process_handler);
    }

    pub fn kill_all_processes_gracefully(&self) {
        self.running_processes
            .borrow()
            .iter()
            .for_each(|process_handler: &ProcessHandler| {
                let pid = process_handler.reader_handle.pids().to_vec();
                process_handler.output_handler.remove_output_capture_file();
                if let Err(_) = process_handler.reader_handle.try_wait() {
                    process_handler
                        .reader_handle
                        .kill()
                        .expect("Unable to kill process");
                    info!("Processes killed: {:?}", pid);
                }
            })
    }
}
