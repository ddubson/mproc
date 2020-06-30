use crate::command_loader;
use crate::command_loader::MprocCommand;
use gtk::{TextBuffer, TextBufferExt};
use std::fs::read_to_string;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::{process, thread};

pub fn spawn(mproc_command: MprocCommand, output_buffer: TextBuffer) {
    // Create a channel to pass between threads
    let (thread_sender, thread_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

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

        if let Some(ref mut stdout) = spawned_process.stdout {
            for line in BufReader::new(stdout).lines() {
                thread_sender
                    .send(line.expect("Unable to read stdout line"))
                    .expect("Unable to write standard output to view.");
            }
        }
    });

    // Receive spawned process standard output text
    let buffer = output_buffer.clone();

    // Listen for standard output data from the command thread
    thread_receiver.attach(None, move |msg| {
        buffer.insert(
            &mut buffer.get_end_iter(),
            format!("{}{}", &msg.as_str(), "\n").as_str(),
        );
        glib::Continue(true)
    });
}

pub fn run_sample_process(output_buffer: TextBuffer, args: &Vec<String>) {
    let commands_file_path = &args.get(1).expect("Please provide a path to CommandFile!");

    let contents =
        read_to_string(commands_file_path).expect("Something went wrong reading the file.");

    let commands = command_loader::get_commands(&contents).expect("Unable to read commands!");

    let first_command = commands
        .first()
        .expect("Can't find the first command in Yaml")
        .clone();
    spawn(first_command, output_buffer);
}
