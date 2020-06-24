use glib::clone;
use gtk::prelude::*;
use gtk::{
    ScrolledWindow, ScrolledWindowBuilder, TextBuffer, TextBufferBuilder, TextTag, TextTagBuilder,
    TextTagTable, TextView, WidgetExt,
};

pub struct ProcessUIContainer {
    pub text_buffer: TextBuffer,
    pub text_view: TextView,
    pub scrolled_window: ScrolledWindow,
}

pub fn create_process_ui_container() -> ProcessUIContainer {
    let background_tag: TextTag = TextTagBuilder::new().background("black").build();
    let foreground_tag: TextTag = TextTagBuilder::new().background("white").build();
    let tag_table: TextTagTable = TextTagTable::new();
    tag_table.add(&background_tag);
    tag_table.add(&foreground_tag);
    let text_buffer: TextBuffer = TextBufferBuilder::new().tag_table(&tag_table).build();
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
