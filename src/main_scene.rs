use gtk::{Application, ApplicationWindow, ButtonBuilder,
          BoxBuilder, Orientation, WidgetExt, ButtonExt,
          ContainerExt, GtkWindowExt, TextBufferExt};
use glib::clone;
use std::process::Command;
use crate::main_scene::process_container::ProcessUIContainer;

struct WindowConfiguration {
    title: &'static str,
    height: i32,
    width: i32,
}

const STD_WINDOW_CONFIG: WindowConfiguration = WindowConfiguration {
    title: "mproc",
    height: 800,
    width: 600,
};

pub fn on_window_activate(app: &Application) {
    let window = ApplicationWindow::new(app);
    let button = ButtonBuilder::new()
        .label("Exit mproc!")
        .build();
    button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    let process_container: ProcessUIContainer = process_container::create_process_ui_container();

    let box_container = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .build();
    box_container.add(&process_container.scrolled_window);
    box_container.add(&button);

    window.add(&box_container);
    window.set_title(&STD_WINDOW_CONFIG.title);
    window.set_default_size(STD_WINDOW_CONFIG.width, STD_WINDOW_CONFIG.height);
    window.show_all();

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
    use gtk::{TextView, TextBuffer, TextBufferBuilder, ScrolledWindow, ScrolledWindowBuilder, WidgetExt};
    use glib::clone;
    use gtk::prelude::*;

    pub struct ProcessUIContainer {
        pub text_buffer: TextBuffer,
        pub text_view: TextView,
        pub scrolled_window: ScrolledWindow,
    }

    pub fn create_process_ui_container() -> ProcessUIContainer {
        let text_buffer: TextBuffer = TextBufferBuilder::new().build();
        let text_view: TextView = TextView::new_with_buffer(&text_buffer);
        let scrolled_window: ScrolledWindow = ScrolledWindowBuilder::new()
            .min_content_height(400)
            .min_content_width(600)
            .child(&text_view)
            .build();

        // Listen for text view changes, auto-scroll as text is added.
        text_view.connect_size_allocate(clone!(@weak scrolled_window => move |_,_| {
            let adj = scrolled_window.get_vadjustment().unwrap();
            adj.set_value(adj.get_upper() - adj.get_page_size());
        }));

        ProcessUIContainer {
            text_buffer,
            text_view,
            scrolled_window,
        }
    }
}
