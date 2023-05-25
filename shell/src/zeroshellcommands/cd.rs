use crate::{utils::parse_generic_command, zeroshellcommandserror::ZeroShellCommandsError};
use std::{env, str::FromStr};

#[derive(Debug, PartialEq)]
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
        if &self.args.len() >= &1 {
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
    fn test_cd() {
        let cd = Cd::from_str("cd");
        assert_eq!(cd.command, "cd");
        assert_eq!(cd.args, Vec::<String>::new());
    }
    #[test]
    fn test_cd_with_args() {
        let cd = Cd::from_str("cd /tmp");
        assert_eq!(cd.command, "cd");
        assert_eq!(cd.args, vec!["/tmp".to_string()]);
    }

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
