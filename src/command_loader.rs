extern crate serde_yaml;

use serde::Deserialize;
use std::fmt;

#[derive(Debug,Deserialize)]
pub struct Command {
    pub name: String,
    pub run: String
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "('name': {}, 'run': {})", self.name, self.run)
    }
}

#[derive(Deserialize)]
struct CommandFile {
    commands: Vec<Command>
}

pub fn get_commands(file_contents: &str) -> Result<Vec<Command>, String> {
    serde_yaml::from_str(&file_contents)
        .and_then(|data: CommandFile| Ok(data.commands))
        .map_err(|error| { error.to_string() })
}
