mod cat;
mod cd;
mod cp;
mod echo;
mod ls;
mod mkdir;
mod mv;
mod pwd;
mod rm;
use crate::command::cat::Cat;
use crate::command::cd::Cd;
use crate::command::cp::Cp;
use crate::command::echo::Echo;
use crate::command::ls::Ls;
use crate::command::mkdir::Mkdir;
use crate::command::mv::Mv;
use crate::command::pwd::Pwd;
use crate::command::rm::Rm;
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
    Pwd(Pwd),
    Cat(Cat),
    Echo(Echo),
    Rm(Rm),
    Cp(Cp),
    Mkdir(Mkdir),
    Mv(Mv),
    Exit,
}
impl ZeroShellCommands {
    pub fn from_str(command: &str) -> Result<Self, ZeroShellCommandsError<String>> {
        match command.split_whitespace().next().unwrap_or("") {
            "cd" => Ok(ZeroShellCommands::Cd(Cd::from_str(command))),
            "ls" => Ok(ZeroShellCommands::Ls(Ls::from_str(command))),
            "pwd" => Ok(ZeroShellCommands::Pwd(Pwd::from_str(command))),
            "cat" => Ok(ZeroShellCommands::Cat(Cat::from_str(command))),
            "echo" => Ok(ZeroShellCommands::Echo(Echo::from_str(command))),
            "rm" => Ok(ZeroShellCommands::Rm(Rm::from_str(command))),
            "cp" => Ok(ZeroShellCommands::Cp(Cp::from_str(command))),
            "mkdir" => Ok(ZeroShellCommands::Mkdir(Mkdir::from_str(command))),
            "mv" => Ok(ZeroShellCommands::Mv(Mv::from_str(command))),
            "exit" => Ok(ZeroShellCommands::Exit),
            _ => {
                println!("{}", format!("{}: command not found", command));
                return Err(ZeroShellCommandsError::Unknown);
            }
        }
    }
}
impl FromStr for ZeroShellCommands {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        Ok(ZeroShellCommands::from_str(command).unwrap())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ZeroShellCommandsError<T> {
    Cd(T),
    Ls(T),
    Pwd(T),
    Cat(T),
    Echo(T),
    Rm(T),
    Cp(T),
    Mkdir(T),
    Mv(T),
    Unknown,
}
