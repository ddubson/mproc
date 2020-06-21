use gtk::{Application, ApplicationWindow, ButtonBuilder,
          BoxBuilder, Orientation, WidgetExt, ButtonExt,
          ContainerExt, GtkWindowExt};
use glib::clone;
use crate::process_container::ProcessUIContainer;
use crate::process_container;

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

    machine_process::run_sample_process(process_container.text_buffer)
}

mod machine_process {
    use std::process::{Command};
    use gtk::{TextBuffer, TextBufferExt};

    pub struct MachineProcess {
        command: &'static str,
    }

    pub trait SpawnsProcess {
        //FIXME output buffer should be generic, not tied to GTK
        fn spawn(&self, output_buffer: TextBuffer);
    }

    impl SpawnsProcess for MachineProcess {
        fn spawn(&self, output_buffer: TextBuffer) {
            let output = Command::new(&self.command)
                .output()
                .expect("failed to execute process");

            match std::str::from_utf8(&output.stdout) {
                Ok(x) => {
                    let text_buffer = output_buffer;
                    // Display the output of command in GUI
                    text_buffer.insert(&mut text_buffer.get_end_iter(), x);
                }
                _ => {
                    println!("Nothing");
                }
            }
        }
    }

    pub fn run_sample_process(output_buffer: TextBuffer) {
        let lsof_proc = MachineProcess {
            command: "lsof"
        };
        lsof_proc.spawn(output_buffer);
    }
}
