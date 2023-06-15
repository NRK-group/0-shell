mod cd;
mod ls;
use crate::command::cd::Cd;
use crate::command::ls::Ls;
use std::str::FromStr;

#[derive(Debug)]
pub struct Command<'c>(pub &'c str);

impl<'c> Command<'c> {
    pub fn new(command: &'c str) -> Self {
        assert!(!command.is_empty(), "Command cannot be empty");
        Self(command)
    }

    pub fn bin_path(&self) -> &str {
        self.0.split_whitespace().next().unwrap()
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum ZeroShellCommands {
    Cd(Cd),
    Ls(Ls),
    Exit,
}
impl ZeroShellCommands {
    pub fn from_str(command: &str) -> Result<Self, ZeroShellCommandsError<String>> {
        match command.split_whitespace().next().unwrap_or("") {
            "cd" => Ok(ZeroShellCommands::Cd(Cd::from_str(command))),
            "ls" => Ok(ZeroShellCommands::Ls(Ls::from_str(command))),
            "exit" => Ok(ZeroShellCommands::Exit),
            _ => Err(ZeroShellCommandsError::Unknown),
        }
    }
}
impl FromStr for ZeroShellCommands {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        Ok(ZeroShellCommands::from_str(command).unwrap())
    }
}

#[derive(Debug)]
pub enum ZeroShellCommandsError<T> {
    Cd(T),
    Ls(T),
    Unknown,
}
