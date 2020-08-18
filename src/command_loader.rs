extern crate serde_yaml;

use crate::core::settings::DEFAULT_CONFIG_FILE_NAME;
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
    let commands_file_path = resolve_config_file_by_args(&args);

    let contents =
        read_to_string(*commands_file_path).expect("Something went wrong reading the file.");

    let mut commands = read_commands_from_yaml_string(&contents).expect("Unable to read commands!");
    commands.truncate(limit);
    commands
}

fn resolve_config_file_by_args(args: &Vec<String>) -> Box<String> {
    if let Some(path) = args.get(1) {
        Box::new(path.clone())
    } else {
        Box::new(String::from(DEFAULT_CONFIG_FILE_NAME))
    }
}

fn read_commands_from_yaml_string(yaml_string: &str) -> Result<Vec<MprocCommand>, String> {
    serde_yaml::from_str(&yaml_string)
        .and_then(|data: CommandFile| Ok(data.commands))
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use crate::command_loader::read_commands_from_yaml_string;

    #[test]
    fn read_commands_from_yaml_string_when_provided_a_valid_yaml_returns_valid_repr_of_commands() {
        let yaml_string = "commands:\n
             - name: All processes and ports\n
               run: lsof";
        let actual_commands_res = read_commands_from_yaml_string(yaml_string);

        assert_eq!(true, actual_commands_res.is_ok());
        if let Ok(result) = actual_commands_res {
            assert_eq!(1, result.len());
            assert_eq!("lsof", result.first().unwrap().run);
            assert_eq!("All processes and ports", result.first().unwrap().name);
        }
    }

    #[test]
    fn read_commands_from_yaml_string_when_provided_an_invalid_yaml_returns_error() {
        let yaml_string = "commands:\n
             - badname: RRR\n
               badrun: lsof";
        let actual_commands_res = read_commands_from_yaml_string(yaml_string);
        assert_eq!(true, actual_commands_res.is_err());
    }
}
