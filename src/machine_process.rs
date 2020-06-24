use gtk::{TextBuffer, TextBufferExt};
use std::process::{Command, Output};

pub struct MachineProcess {
    command: &'static str,
}

pub trait SpawnsProcess {
    //FIXME output buffer should be generic, not tied to GTK
    fn spawn(&self, output_buffer: TextBuffer);
}

impl SpawnsProcess for MachineProcess {
    fn spawn(&self, output_buffer: TextBuffer) {
        let output: Output;

        if cfg!(windows) {
            output = Command::new("cmd")
                .args(&["/C", &self.command])
                .output()
                .expect("Unable to spawn Windows process.");
        } else {
            output = Command::new(&self.command)
                .output()
                .expect("Unable to spawn process.");
        }

        match std::str::from_utf8(&output.stdout) {
            Ok(x) => {
                let text_buffer = output_buffer;
                // Display the output of command in GUI
                text_buffer.insert(&mut text_buffer.get_end_iter(), x);
            }
            _ => {
                println!("Nothing");
            }
        }
    }
}

pub fn run_sample_process(output_buffer: TextBuffer) {
    if cfg!(windows) {
        MachineProcess { command: "dir" }.spawn(output_buffer);
    } else {
        MachineProcess { command: "lsof" }.spawn(output_buffer);
    }
}
