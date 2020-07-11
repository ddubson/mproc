pub struct WindowConfiguration {
    pub title: &'static str,
    pub height: i32,
    pub width: i32,
}

pub const STD_WINDOW_CONFIG: WindowConfiguration = WindowConfiguration {
    title: "mproc",
    height: 800,
    width: 1200,
};
