use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn on_activate(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    let button = Button::new_with_label("Hello World!");
    button.connect_clicked(clone!(@weak window => move |_| window.destroy()));
    window.add(&button);
    window.show_all();
}

fn main() {
    // Create a new application
    let app = Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));
    app.run(&std::env::args().collect::<Vec<_>>());
}
