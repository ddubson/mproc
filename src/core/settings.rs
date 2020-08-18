pub static DEFAULT_CONFIG_FILE_NAME: &str = ".mproc.yml";
static DEFAULT_NIX_DIRECTORY: &str = "~/.mproc";
static DEFAULT_WINDOWS_DIRECTORY: &str = "~/AppData/Local/Temp/.mproc";

pub struct AppSettings {
    pub process_limit: usize,
    pub mproc_working_directory: String,
}

impl AppSettings {
    pub fn default() -> Self {
        AppSettings {
            process_limit: 4,
            mproc_working_directory: if cfg!(windows) {
                DEFAULT_WINDOWS_DIRECTORY.to_string()
            } else {
                DEFAULT_NIX_DIRECTORY.to_string()
            },
        }
    }
}
