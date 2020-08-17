use crate::process_output_handler::ProcessOutputHandler;
use crate::settings::AppSettings;
use duct::ReaderHandle;
use log::info;
use std::cell::RefCell;

pub struct ProcessHandler {
    pub reader_handle: ReaderHandle,
    pub output_handler: ProcessOutputHandler,
}

pub struct State {
    pub app_settings: AppSettings,
    pub running_processes: RefCell<Vec<ProcessHandler>>,
}

impl State {
    pub fn new() -> Self {
        State {
            app_settings: AppSettings::default(),
            running_processes: RefCell::new(Vec::new()),
        }
    }

    pub fn add_process_handler(&self, process_handler: ProcessHandler) {
        info!(
            "Process(es) {:?} added to state.",
            process_handler.reader_handle.pids()
        );
        self.running_processes.borrow_mut().push(process_handler);
    }

    pub fn kill_all_processes_gracefully(&self) {
        self.running_processes
            .borrow()
            .iter()
            .for_each(|process_handler| {
                let pid = process_handler.reader_handle.pids().to_vec();
                process_handler
                    .reader_handle
                    .kill()
                    .expect("Unable to kill process");
                info!("Processes killed: {:?}", pid);
            })
    }
}
