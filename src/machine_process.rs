use gtk::{TextBuffer, TextBufferExt};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::{process, thread};

pub struct MachineProcess {
    command: &'static str,
}

pub trait SpawnsProcess {
    //FIXME output buffer should be generic, not tied to GTK
    fn spawn(&self, output_buffer: TextBuffer);
}

impl SpawnsProcess for MachineProcess {
    fn spawn(&self, output_buffer: TextBuffer) {
        // Create a channel to pass between threads
        let (thread_sender, thread_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        // Create an atomic reference of the command to be run
        let cmd = Arc::new(self.command.clone());

        // Spawn a new thread to run the user command on
        thread::spawn(move || {
            let spawned_process: process::Child;

            if cfg!(windows) {
                spawned_process = Command::new("cmd")
                    .stdout(Stdio::piped())
                    .args(&["/C", *cmd])
                    .spawn()
                    .expect("Unable to spawn Windows process");
            } else {
                spawned_process = Command::new(*cmd)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Unable to spawn process");
            }

            let process_output: process::Output = spawned_process
                .wait_with_output()
                .expect("Unable to capture standard output of process");

            match std::str::from_utf8(&process_output.stdout) {
                Ok(x) => {
                    // Emit the std output data back to the main thread
                    thread_sender
                        .send(x.to_string())
                        .expect("Unable to write standard output to view.");
                }
                _ => {
                    println!("Nothing");
                }
            }
        });

        // Receive spawned process standard output text
        let buffer = output_buffer.clone();

        // Listen for standard output data from the command thread
        thread_receiver.attach(None, move |msg| {
            buffer.insert(&mut buffer.get_end_iter(), msg.as_str());
            glib::Continue(true)
        });
    }
}

pub fn run_sample_process(output_buffer: TextBuffer) {
    if cfg!(windows) {
        MachineProcess { command: "dir" }.spawn(output_buffer);
    } else {
        MachineProcess { command: "lsof" }.spawn(output_buffer);
    }
}
