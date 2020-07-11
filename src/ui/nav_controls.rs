use glib::clone;
use gtk::{
    ApplicationWindow, ButtonBox, ButtonBoxBuilder, ButtonBoxStyle, ButtonBuilder, ButtonExt,
    ContainerExt, GtkWindowExt, Orientation,
};

const EXIT_LABEL: &'static str = "Exit mproc";

pub fn create_bottom_nav_controls(main_window: &ApplicationWindow) -> ButtonBox {
    let button = ButtonBuilder::new().label(EXIT_LABEL).build();
    button.connect_clicked(clone!(@weak main_window => move |_| main_window.close()));
    let main_button_controls: ButtonBox = ButtonBoxBuilder::new()
        .orientation(Orientation::Horizontal)
        .margin_top(25)
        .layout_style(ButtonBoxStyle::Start)
        .build();
    main_button_controls.add(&button);
    main_button_controls
}
