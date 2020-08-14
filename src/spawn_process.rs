use std::thread;

use duct::{cmd, ReaderHandle};

use crate::command_loader::MprocCommand;
use crate::file_watcher::FileWatcher;
use crate::state::State;
use crate::ui::mproc_process_container::MprocProcessContainer;
use glib::random_int;
use std::fs::File;

pub fn spawn_process<T: MprocProcessContainer>(
    proc_container: Box<T>,
    mproc_command: MprocCommand,
    state: &State,
) where
    T: Clone + 'static,
{
    // Create a channel to pass between threads
    let (stdout_send, stdout_recv) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (gui_pid_send, gui_pid_recv) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // Spawn the command and get a result of either process handle or an error
    let file_name = format!("proc-{}.out", random_int());
    let output_file = File::create(&file_name)
        .expect(format!("Unable to create process output file: {}", &file_name).as_str());

    let commands = mproc_command
        .run
        .clone()
        .split_whitespace()
        .map(|v| String::from(v))
        .collect::<Vec<String>>();
    let command: String = commands
        .first()
        .expect("Not given a valid command!")
        .clone();
    let args: Vec<String> = commands[1..].to_vec();

    let spawned_process: Result<ReaderHandle, std::io::Error> = cmd!(command)
        .before_spawn(move |c| {
            c.args(args.to_vec());
            Ok(())
        })
        .stdout_file(output_file)
        .reader();

    if let Ok(proc_handle) = spawned_process {
        gui_pid_send
            .send(proc_handle.pids().clone())
            .expect("Unable to send GUI PID for newly spawned process.");

        state.add_running_process(proc_handle);

        thread::spawn(move || {
            let mut output_reader = FileWatcher::register(file_name).unwrap();

            output_reader.watch(|line: String| {
                stdout_send
                    .send(line)
                    .expect("Unable to write standard output to view.");
            });
        });

        let command_label = mproc_command.name.clone();
        let proc_container_c = proc_container.clone();
        // Listen for standard output data from the command thread
        stdout_recv.attach(None, move |msg| {
            proc_container_c.append_to_view(msg);
            glib::Continue(true)
        });

        let proc_container_c = proc_container.clone();
        gui_pid_recv.attach(None, move |pid| {
            proc_container_c.set_title(format!("{} ({})", command_label, pid.first().unwrap()));
            glib::Continue(true)
        });
    }
}
