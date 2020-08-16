use duct::ReaderHandle;
use log::info;
use std::cell::RefCell;

pub struct AppSettings {
    pub process_limit: usize,
}

pub struct State {
    pub app_settings: AppSettings,
    pub running_processes: RefCell<Vec<ReaderHandle>>,
}

impl State {
    pub fn new() -> Self {
        State {
            app_settings: AppSettings { process_limit: 4 },
            running_processes: RefCell::new(Vec::new()),
        }
    }

    pub fn add_running_process(&self, process: ReaderHandle) {
        info!("Process(es) {:?} added to state.", process.pids());
        self.running_processes.borrow_mut().push(process);
    }

    pub fn kill_all_processes_gracefully(&self) {
        self.running_processes.borrow().iter().for_each(|process| {
            let pid = process.pids().to_vec();
            process.kill().expect("Unable to kill process");
            info!("Processes killed: {:?}", pid);
        })
    }
}
