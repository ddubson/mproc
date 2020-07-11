use glib::clone;
use gtk::prelude::*;
use gtk::{
    ScrolledWindow, ScrolledWindowBuilder, TextBuffer, TextBufferBuilder, TextView,
    TextViewBuilder, WidgetExt,
};

pub struct MprocProcessContainer {
    pub text_buffer: TextBuffer,
    pub text_view: TextView,
    pub scrolled_window: ScrolledWindow,
}

impl MprocProcessContainer {
    pub fn new() -> Self {
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

        MprocProcessContainer {
            text_buffer,
            text_view,
            scrolled_window,
        }
    }
}
