use gtk::{
    Application, ApplicationWindow, Box, BoxBuilder, ContainerExt, GtkWindowExt, LabelBuilder,
    NotebookBuilder, NotebookExt, Orientation, WidgetExt,
};

use crate::command_loader::extract_first_command;
use crate::machine_process;
use crate::ui::mproc_process_container::MprocProcessContainer;
use crate::ui::nav_controls::create_bottom_nav_controls;
use crate::ui::view_settings::STD_WINDOW_CONFIG;

fn on_application_loading(main_box_container: &Box, args: &Vec<String>) {
    let first_command = extract_first_command(&args);
    let notebook_of_processes = NotebookBuilder::new().show_tabs(true).build();

    let process_container: MprocProcessContainer = MprocProcessContainer::new();
    let process_box = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .margin(25)
        .build();
    process_box.add(&process_container.scrolled_window);
    notebook_of_processes.add(&process_box);
    notebook_of_processes.set_tab_label(
        &process_box,
        Some(&LabelBuilder::new().label(&first_command.name).build()),
    );
    main_box_container.add(&notebook_of_processes);

    machine_process::spawn(first_command, process_container.text_buffer);
}

pub fn on_window_activate(app: &Application, args: &Vec<String>) {
    let main_window = ApplicationWindow::new(app);
    let bottom_nav_controls = create_bottom_nav_controls(&main_window);

    let box_container = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .margin(25)
        .build();
    on_application_loading(&box_container, args);
    box_container.add(&bottom_nav_controls);

    main_window.add(&box_container);
    main_window.set_title(&STD_WINDOW_CONFIG.title);
    main_window.set_default_size(STD_WINDOW_CONFIG.width, STD_WINDOW_CONFIG.height);
    main_window.show_all();
}
