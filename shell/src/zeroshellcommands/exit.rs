use std::str::FromStr;

use crate::{utils::parse_generic_command, zeroshellcommandserror::ZeroShellCommandsError};

#[derive(Debug, PartialEq)]
pub struct Exit {
    pub command: String,
    pub args: Vec<String>,
}

impl Exit {
    pub fn from_str(command: &str) -> Self {
        let (command, args) = parse_generic_command(command);
        Exit { command, args }
    }
    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        if self.args.len() > 0 {
            std::process::exit(match self.args[0].parse::<i32>() {
                Ok(code) => code,
                Err(_) => {
                    return Err(ZeroShellCommandsError::Exit(
                        "exit: numeric argument required".to_string(),
                    ));
                }
            });
        }
        std::process::exit(0);
    }
}

impl FromStr for Exit {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        Ok(Exit::from_str(command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit() {
        let exit = Exit::from_str("exit");
        assert_eq!(exit.command, "exit");
        assert_eq!(exit.args, Vec::<String>::new());
    }
    #[test]
    fn test_exit_with_args() {
        let exit = Exit::from_str("exit 1");
        assert_eq!(exit.command, "exit");
        assert_eq!(exit.args, vec!["1".to_string()]);
    }

    #[test]
    fn test_exit_with_invalid_args() {
        let exit = Exit::from_str("exit /tmp");
        assert_eq!(
            exit.execute().unwrap_err(),
            ZeroShellCommandsError::Exit("exit: numeric argument required".to_string())
        );
    }
}
