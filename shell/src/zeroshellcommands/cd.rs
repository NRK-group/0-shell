use crate::{zeroshellcommandserror::ZeroShellCommandsError, utils::parse_generic_command};
use std::{env, str::FromStr};

#[derive(Debug)]
pub struct Cd {
    pub command: String,
    pub args: Vec<String>,
}

impl Cd {
    pub fn from_str(command: &str) -> Self {
        let (command, args) = parse_generic_command(command);
        Self { command, args }
    }
    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        if &self.args.len() == &1 {
            match env::set_current_dir(&self.args[0]) {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => {
                    return Err(ZeroShellCommandsError::Cd(err.to_string()));
                }
            };
        }
        match env::set_current_dir("/") {
            Ok(_) => Ok(()),
            Err(err) => {
                return Err(ZeroShellCommandsError::Cd(err.to_string()));
            }
        }
    }
}

impl FromStr for Cd {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        Ok(Cd::from_str(command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_with_valid_directory() {
        let cd = Cd::from_str("cd /tmp");
        let result = cd.execute();
        assert!(result.is_ok(), "Expected Ok result");
    }

    #[test]
    fn test_execute_with_invalid_directory() {
        let cd = Cd::from_str("cd /nonexistent");
        println!("{:?}", cd);
        let result = cd.execute();
        assert!(result.is_err(), "Expected Err result");
    }
}
