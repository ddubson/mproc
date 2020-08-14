use duct::ReaderHandle;
use std::cell::RefCell;

pub struct State {
    pub running_processes: RefCell<Vec<ReaderHandle>>,
}

impl State {
    pub fn new() -> Self {
        State {
            running_processes: RefCell::new(Vec::new()),
        }
    }

    pub fn add_running_process(&self, process: ReaderHandle) {
        self.running_processes.borrow_mut().push(process);
    }

    pub fn kill_all_processes_gracefully(&self) {
        self.running_processes.borrow().iter().for_each(|process| {
            let pid = process.pids().to_vec();
            process.kill().expect("Unable to kill process");
            println!("Processes killed: {:?}", pid);
        })
    }
}
