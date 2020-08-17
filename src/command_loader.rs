extern crate serde_yaml;

use crate::settings::DEFAULT_CONFIG_FILE_NAME;
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

pub fn extract_all_commands(args: &Vec<String>, limit: usize) -> Vec<MprocCommand> {
    let default = Box::new(String::from(DEFAULT_CONFIG_FILE_NAME));
    let commands_file_path = &args
        .get(1)
        .or(Some(&default))
        .expect("Unable to find mproc yml config (.mproc.yml)");

    let contents =
        read_to_string(commands_file_path).expect("Something went wrong reading the file.");

    let mut commands = read_commands_from_yaml_string(&contents).expect("Unable to read commands!");
    commands.truncate(limit);

    commands
}

fn read_commands_from_yaml_string(yaml_string: &str) -> Result<Vec<MprocCommand>, String> {
    serde_yaml::from_str(&yaml_string)
        .and_then(|data: CommandFile| Ok(data.commands))
        .map_err(|error| error.to_string())
}
