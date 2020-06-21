use gtk::{Application, ApplicationWindow, ButtonBuilder, TextBuffer, TextBufferBuilder, TextView,
          BoxBuilder, ScrolledWindowBuilder, ScrolledWindow, Orientation, WidgetExt, ButtonExt,
          ContainerExt, GtkWindowExt, TextBufferExt};
use glib::clone;
use gtk::prelude::*;
use std::process::Command;

pub fn on_window_activate(app: &Application) {
    let window = ApplicationWindow::new(app);
    let button = ButtonBuilder::new()
        .label("Exit mproc!")
        .build();
    button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    let text_buffer: TextBuffer = TextBufferBuilder::new().build();
    let text_view: TextView = TextView::new_with_buffer(&text_buffer);
    let scrolled_window: ScrolledWindow = ScrolledWindowBuilder::new()
        .min_content_height(400)
        .min_content_width(600)
        .child(&text_view)
        .build();

    let box_container = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .build();
    box_container.add(&scrolled_window);
    box_container.add(&button);

    window.add(&box_container);
    window.set_title("mproc");
    window.set_default_size(600, 800);
    window.show_all();

    // Listen for text view changes, auto-scroll as text is added.
    text_view.connect_size_allocate(clone!(@weak scrolled_window => move |_,_| {
        let adj = scrolled_window.get_vadjustment().unwrap();
        adj.set_value(adj.get_upper() - adj.get_page_size());
    }));

    let output = Command::new("lsof")
        .output()
        .expect("failed to execute process");

    match std::str::from_utf8(&output.stdout) {
        Ok(x) => {
            // Display the output of command in GUI
            text_buffer.insert(&mut text_buffer.get_end_iter(), x);
        }
        _ => {
            println!("Nothing");
        }
    }
}