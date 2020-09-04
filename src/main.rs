extern crate log;
extern crate simple_logger;

use std::env::args;

use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use gtk::Application;
use simple_logger::SimpleLogger;

use crate::application::MprocApplication;

mod core;

pub mod application;
mod ui;

fn main() {
    SimpleLogger::new();
    let args = args().collect::<Vec<_>>().clone();
    let app = Application::new(Some("com.ddubson.mproc"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("GTK application initialization failed...");

    app.connect_activate(move |app| MprocApplication::create(app).start(&args));
    app.run(&vec![]);
}
