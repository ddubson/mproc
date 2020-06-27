use glib::clone;
use gtk::{
    Application, ApplicationWindow, BoxBuilder, ButtonBuilder, ButtonExt, ContainerExt,
    GtkWindowExt, Orientation, WidgetExt,
};

use crate::process_container::ProcessUIContainer;
use crate::{machine_process, process_container};

struct WindowConfiguration {
    title: &'static str,
    height: i32,
    width: i32,
}

const STD_WINDOW_CONFIG: WindowConfiguration = WindowConfiguration {
    title: "mproc",
    height: 800,
    width: 1200,
};

pub fn on_window_activate(app: &Application, args: &Vec<String>) {
    let window = ApplicationWindow::new(app);
    let button = ButtonBuilder::new().label("Exit mproc!").build();
    button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    let process_container: ProcessUIContainer = process_container::create_process_ui_container();

    let box_container = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .margin(25)
        .build();
    box_container.add(&process_container.scrolled_window);
    box_container.add(&button);

    window.add(&box_container);
    window.set_title(&STD_WINDOW_CONFIG.title);
    window.set_default_size(STD_WINDOW_CONFIG.width, STD_WINDOW_CONFIG.height);
    window.show_all();

    machine_process::run_sample_process(process_container.text_buffer, args)
}
