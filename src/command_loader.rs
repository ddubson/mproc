extern crate serde_yaml;

use serde::Deserialize;
use std::fmt;
use std::fs::read_to_string;

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

pub fn extract_first_command(args: &Vec<String>) -> MprocCommand {
    let default = Box::new(String::from(".mproc.yml"));
    let commands_file_path = &args
        .get(1)
        .or(Some(&default))
        .expect("Unable to find mproc yml config (.mproc.yml)");

    let contents =
        read_to_string(commands_file_path).expect("Something went wrong reading the file.");

    let commands = get_commands(&contents).expect("Unable to read commands!");

    commands
        .first()
        .expect("Can't find the first command in Yaml")
        .clone()
}
