use gtk::{Application, ApplicationWindow, ButtonBuilder,
          BoxBuilder, ScrolledWindowBuilder, ScrolledWindow, Orientation, WidgetExt, ButtonExt,
          ContainerExt, GtkWindowExt, TextBufferExt};
use glib::clone;
use gtk::prelude::*;
use std::process::Command;

struct WindowConfiguration {
    height: i32,
    width: i32,
}

const STD_WINDOW_CONFIG: WindowConfiguration = WindowConfiguration {
    height: 800,
    width: 600
};

pub fn on_window_activate(app: &Application) {
    let window = ApplicationWindow::new(app);
    let button = ButtonBuilder::new()
        .label("Exit mproc!")
        .build();
    button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    let process_container = process_container::create_process_ui_container();

    let scrolled_window: ScrolledWindow = ScrolledWindowBuilder::new()
        .min_content_height(400)
        .min_content_width(600)
        .child(&process_container.text_view)
        .build();

    let box_container = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .build();
    box_container.add(&scrolled_window);
    box_container.add(&button);

    window.add(&box_container);
    window.set_title("mproc");
    window.set_default_size(STD_WINDOW_CONFIG.width, STD_WINDOW_CONFIG.height);
    window.show_all();

    // Listen for text view changes, auto-scroll as text is added.
    process_container.text_view.connect_size_allocate(clone!(@weak scrolled_window => move |_,_| {
        let adj = scrolled_window.get_vadjustment().unwrap();
        adj.set_value(adj.get_upper() - adj.get_page_size());
    }));

    let output = Command::new("lsof")
        .output()
        .expect("failed to execute process");

    match std::str::from_utf8(&output.stdout) {
        Ok(x) => {
            let text_buffer = process_container.text_buffer;
            // Display the output of command in GUI
            text_buffer.insert(&mut text_buffer.get_end_iter(), x);
        }
        _ => {
            println!("Nothing");
        }
    }
}

mod process_container {
    use gtk::{TextView, TextBuffer, TextBufferBuilder};

    pub struct ProcessUIContainer {
        pub text_buffer: TextBuffer,
        pub text_view: TextView,
    }

    pub fn create_process_ui_container() -> ProcessUIContainer {
        let text_buffer: TextBuffer = TextBufferBuilder::new().build();
        let text_view: TextView = TextView::new_with_buffer(&text_buffer);
        let process_ui_container = ProcessUIContainer {
            text_buffer,
            text_view,
        };
        process_ui_container
    }
}