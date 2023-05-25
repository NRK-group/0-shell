use std::str::FromStr;

use crate::{utils::parse_generic_command, zeroshellcommandserror::ZeroShellCommandsError};

#[derive(Debug, PartialEq)]
pub struct Unknown {
    pub command: String,
    pub args: Vec<String>,
}

impl Unknown {
    pub fn from_str(command: &str) -> Self {
        let (command, args) = parse_generic_command(command);
        Unknown { command, args }
    }
    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        Ok(())
    }
}

impl FromStr for Unknown {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        Ok(Unknown::from_str(command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown() {
        let unknown = Unknown::from_str("unknown");
        assert_eq!(unknown.command, "unknown");
        assert_eq!(unknown.args, Vec::<String>::new());
    }
    #[test]
    fn test_unknown_with_args() {
        let unknown = Unknown::from_str("unknown /tmp");
        assert_eq!(unknown.command, "unknown");
        assert_eq!(unknown.args, vec!["/tmp".to_string()]);
    }

    #[test]
    fn test_unknown_execute() {
        let unknown = Unknown::from_str("unknown");
        assert_eq!(unknown.execute().unwrap(), ());
    }
}
