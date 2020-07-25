use crate::ui::mproc_process_container::MprocProcessContainer;
use glib::clone;
use gtk::prelude::*;
use gtk::{
    LabelBuilder, ScrolledWindow, ScrolledWindowBuilder, TextBuffer, TextBufferBuilder, TextView,
    TextViewBuilder, WidgetExt,
};

#[derive(Clone)]
pub struct GtkMprocProcessContainer {
    pub tab_label: gtk::Label,
    pub text_buffer: gtk::TextBuffer,
    pub text_view: gtk::TextView,
    pub scrolled_window: gtk::ScrolledWindow,
}

impl MprocProcessContainer for GtkMprocProcessContainer {
    fn new() -> GtkMprocProcessContainer {
        let tab_label = LabelBuilder::new().label("(New Process)").build();
        let text_buffer: TextBuffer = TextBufferBuilder::new().build();
        let text_view: TextView = TextViewBuilder::new().buffer(&text_buffer).build();
        let scrolled_window: ScrolledWindow = ScrolledWindowBuilder::new()
            .min_content_height(400)
            .min_content_width(600)
            .vexpand(true)
            .child(&text_view)
            .build();

        // Listen for text view changes, auto-scroll as text is added.
        text_view.connect_size_allocate(clone!(@weak scrolled_window => move |_,_| {
            let adj = scrolled_window.get_vadjustment().unwrap();
            adj.set_value(adj.get_upper() - adj.get_page_size());
        }));

        GtkMprocProcessContainer {
            tab_label,
            text_buffer,
            text_view,
            scrolled_window,
        }
    }

    fn append_to_view(&self, text: String) {
        self.text_buffer.insert(
            &mut self.text_buffer.get_end_iter(),
            format!("{}{}", &text.as_str(), "\n").as_str(),
        );
    }

    fn set_title(&self, title: String) {
        self.tab_label.set_label(title.as_str());
    }
}
