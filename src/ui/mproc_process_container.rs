pub trait MprocProcessContainer {
    fn new() -> Self;

    fn append_to_view(&self, text: String);

    fn set_title(&self, title: String);
}
