use crate::command_loader::MprocCommand;
use gtk::{TextBuffer, TextBufferExt, Label, LabelExt};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::{process, thread};

pub fn spawn(mproc_command: MprocCommand, output_buffer: TextBuffer, tab_label: Label) {
    // Create a channel to pass between threads
    let (stdout_send, stdout_recv) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (pid_send, pid_recv) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // Create an atomic reference of the command to be run
    let cmd = Arc::new(mproc_command.run);

    // Spawn a new thread to run the user command on
    thread::spawn(move || {
        let mut spawned_process: process::Child;

        if cfg!(windows) {
            spawned_process = Command::new("cmd")
                .stdout(Stdio::piped())
                .args(&["/C", (*cmd).clone().as_str()])
                .spawn()
                .expect("Unable to spawn Windows process");
        } else {
            let commands = (*cmd)
                .clone()
                .split_whitespace()
                .map(|v| String::from(v))
                .collect::<Vec<String>>();
            let command: &String = commands.first().expect("Not given a valid command!");
            let args: &[String] = &commands[1..];

            spawned_process = Command::new(command)
                .args(args)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Unable to spawn process");
        }

        pid_send.send(spawned_process.id()).expect("Unable to send PID for newly spawned process.");

        if let Some(ref mut stdout) = spawned_process.stdout {
            for line in BufReader::new(stdout).lines() {
                stdout_send
                    .send(line.expect("Unable to read stdout line"))
                    .expect("Unable to write standard output to view.");
            }
        }
    });

    // Receive spawned process standard output text
    let buffer = output_buffer.clone();
    let tab_label_c = tab_label.clone();

    // Listen for standard output data from the command thread
    stdout_recv.attach(None, move |msg| {
        buffer.insert(
            &mut buffer.get_end_iter(),
            format!("{}{}", &msg.as_str(), "\n").as_str(),
        );

        glib::Continue(true)
    });

    pid_recv.attach(None, move |pid| {
        let new_label = format!("{} ({})", tab_label_c.get_label(), pid);
        tab_label_c.set_label(new_label.as_str());
        glib::Continue(true)
    });
}
