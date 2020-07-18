use gtk::{ButtonBox, ButtonBoxBuilder, ButtonBoxStyle, ButtonBuilder, ContainerExt, Orientation};

const EXIT_LABEL: &'static str = "Exit mproc";

pub struct NavControls {
    pub main_button_box: gtk::ButtonBox,
    pub exit_button: gtk::Button,
}

impl NavControls {
    pub fn new() -> NavControls {
        let exit_button = ButtonBuilder::new().label(EXIT_LABEL).build();

        let main_button_box: ButtonBox = ButtonBoxBuilder::new()
            .orientation(Orientation::Horizontal)
            .margin_top(15)
            .layout_style(ButtonBoxStyle::Start)
            .build();
        main_button_box.add(&exit_button);

        NavControls {
            main_button_box,
            exit_button,
        }
    }
}
