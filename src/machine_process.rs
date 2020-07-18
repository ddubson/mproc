use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::{process, thread};

use gtk::{LabelExt, TextBufferExt};

use crate::command_loader::MprocCommand;
use crate::ui::mproc_process_container::MprocProcessContainer;

pub fn spawn_process(proc_container: MprocProcessContainer, mproc_command: MprocCommand) {
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

        pid_send
            .send(spawned_process.id())
            .expect("Unable to send PID for newly spawned process.");

        if let Some(ref mut stdout) = spawned_process.stdout {
            for line in BufReader::new(stdout).lines() {
                stdout_send
                    .send(line.expect("Unable to read stdout line"))
                    .expect("Unable to write standard output to view.");
            }
        }
    });

    // Receive spawned process standard output text
    let buffer = proc_container.text_buffer.clone();
    let tab_label_c = proc_container.tab_label.clone();
    let command_label = mproc_command.name.clone();

    // Listen for standard output data from the command thread
    stdout_recv.attach(None, move |msg| {
        buffer.insert(
            &mut buffer.get_end_iter(),
            format!("{}{}", &msg.as_str(), "\n").as_str(),
        );

        glib::Continue(true)
    });

    pid_recv.attach(None, move |pid| {
        tab_label_c.set_label(format!("{} ({})", command_label, pid).as_str());
        glib::Continue(true)
    });
}
