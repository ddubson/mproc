use std::env;
use std::process::Command;
use std::str;

use gio::prelude::*;
use glib::clone;
use gtk::{Application, ApplicationWindow, ButtonBuilder, Orientation,
          BoxBuilder, TextBuffer, TextView, TextBufferBuilder};
use gtk::prelude::*;

//use yaml_rust::{YamlLoader, Yaml};

fn on_activate(application: &Application) {
    let window = ApplicationWindow::new(application);
    let button = ButtonBuilder::new()
        .label("Exit mproc!")
        .build();
    button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    let text_buffer: TextBuffer = TextBufferBuilder::new()
        .build();
    let text_view: TextView = TextView::new_with_buffer(&text_buffer);
    let box_container = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .build();
    box_container.add(&text_view);
    box_container.add(&button);

    window.add(&box_container);
    window.set_title("mproc");
    window.set_default_size(600, 800);
    window.show_all();

    let output = Command::new("dotnet")
        .arg("--help")
        .output()
        .expect("failed to execute process");

    match str::from_utf8(&output.stdout) {
        Ok(x) => {
            // Display the output of command in GUI
            text_buffer.insert(&mut text_buffer.get_end_iter(), x);
        }
        _ => {
            println!("Nothing");
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect::<Vec<_>>();
    println!("{:?}", args);

    //TODO: check if arg is passed in
    // let contents = fs::read_to_string(&args[1])
    //     .expect("Something went wrong reading the file.");
    //
    // println!("File contents: {}", contents);

    // let mproc_config: Vec<Yaml> = YamlLoader::load_from_str(&contents).unwrap();
    // println!("{:?}", &mproc_config);
    // let commands = mproc_config.first().unwrap().as_hash().unwrap();
    // println!("{:?}", &commands);
    // //
    // // match commands.unwrap().as_hash() {
    // //     Some(x) => {
    // //         x.iter().for_each(|(key, value)| {
    // //             println!("{:?}: {:?}", key, value)
    // //         });
    // //     }
    // //     _ => println!("Nothing"),
    // // }
    // // Read a file into a string/buffer
    // // Read string/buffer as yaml via yaml loader

    let app = Application::new(
        Some("com.ddubson.basic"),
        gio::ApplicationFlags::FLAGS_NONE,
    ).expect("Initialization failed...");

    app.connect_activate(|app| on_activate(app));
    app.run(&vec![]);
}