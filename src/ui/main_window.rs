use crate::ui::gtk::gtk_mproc_process_container::GtkMprocProcessContainer;
use crate::ui::mproc_process_container::MprocProcessContainer;
use crate::ui::nav_controls::NavControls;
use crate::ui::view_settings::STD_WINDOW_CONFIG;
use glib::clone;
use gtk::{
    Application, ApplicationWindow, BoxBuilder, ButtonExt, ContainerExt, GtkWindowExt,
    NotebookBuilder, NotebookExt, Orientation, WidgetExt,
};

pub struct MainWindow {
    pub window: gtk::ApplicationWindow,
    pub controls: NavControls,
    process_notebook: gtk::Notebook,
}

impl MainWindow {
    pub fn new(app: &Application) -> MainWindow {
        let main_window = ApplicationWindow::new(app);

        let bottom_nav_controls: NavControls = NavControls::new();

        let box_container = BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .margin(15)
            .build();
        let notebook_of_processes = NotebookBuilder::new().show_tabs(true).build();
        box_container.add(&notebook_of_processes);
        box_container.add(&bottom_nav_controls.main_button_box);

        main_window.add(&box_container);
        main_window.set_title(&STD_WINDOW_CONFIG.title);
        main_window.set_default_size(STD_WINDOW_CONFIG.width, STD_WINDOW_CONFIG.height);
        MainWindow {
            window: main_window,
            controls: bottom_nav_controls,
            process_notebook: notebook_of_processes,
        }
    }

    pub fn on_exit_button_clicked<F: Fn(&ApplicationWindow) + 'static>(&self, f: F) {
        let app_window = &self.window;
        self.controls
            .exit_button
            .connect_clicked(clone!(@weak app_window => move |_| f(&app_window)));
    }

    pub fn create_process_container(&self) -> Box<GtkMprocProcessContainer> {
        let process_container = GtkMprocProcessContainer::new();
        let process_box = BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .margin(15)
            .build();
        process_box.add(&process_container.scrolled_window);

        self.process_notebook.add(&process_box);
        self.process_notebook
            .set_tab_label(&process_box, Some(&process_container.tab_label));
        Box::new(process_container)
    }

    pub fn show(&self) {
        self.window.show_all();
    }
}
