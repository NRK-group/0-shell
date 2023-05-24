use crate::{zeroshellcommands::ZeroShellCommands, zeroshellcommandserror::ZeroShellCommandsError};

pub trait Execute {
    fn execute(&self) -> Result<(), ZeroShellCommandsError<String>>;
}

impl Execute for ZeroShellCommands {
    fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        match self {
            ZeroShellCommands::Cd(cd) => cd.execute(),
            // ZeroShellCommands::Exit(exit) => exit.execute(),
            // ZeroShellCommands::Help(help) => help.execute(),
            // ZeroShellCommands::History(history) => history.execute(),
            // ZeroShellCommands::Pwd(pwd) => pwd.execute(),
            ZeroShellCommands::Ls(ls) => ls.execute(),
            ZeroShellCommands::Unknown(unknown) => unknown.execute(),
        }
    }
}
