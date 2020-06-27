extern crate serde_yaml;

use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub struct MprocCommand {
    pub name: String,
    pub run: String,
}

impl fmt::Display for MprocCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "('name': {}, 'run': {})", self.name, self.run)
    }
}

#[derive(Deserialize)]
struct CommandFile {
    commands: Vec<MprocCommand>,
}

pub fn get_commands(file_contents: &str) -> Result<Vec<MprocCommand>, String> {
    serde_yaml::from_str(&file_contents)
        .and_then(|data: CommandFile| Ok(data.commands))
        .map_err(|error| error.to_string())
}
