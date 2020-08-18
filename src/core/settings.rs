pub static DEFAULT_CONFIG_FILE_NAME: &str = ".mproc.yml";

pub struct AppSettings {
    pub process_limit: usize,
}

impl AppSettings {
    pub fn default() -> Self {
        AppSettings { process_limit: 4 }
    }
}
