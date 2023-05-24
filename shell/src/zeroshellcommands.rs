mod cd;
mod exit;
mod help;
mod history;
mod ls;
mod pwd;
mod unknown;
use cd::Cd;
use exit::Exit;
// use help::Help;
// use history::History;
use ls::Ls;
use pwd::Pwd;
use std::str::FromStr;
use unknown::Unknown;

pub enum ZeroShellCommands {
    Cd(Cd),
    Exit(Exit),
    // Help(Help),
    // History(History),
    Pwd(Pwd),
    // Echo,
    // Cat,
    Ls(Ls),
    // Clear,
    // Rm,
    // Mkdir,
    // Touch,
    Unknown(Unknown),
}

impl ZeroShellCommands {
    pub fn from_str(command: &str) -> Self {
        match command.split_whitespace().next().unwrap_or("") {
            "cd" => ZeroShellCommands::Cd(Cd::from_str(command)),
            "exit" => ZeroShellCommands::Exit(Exit::from_str(command)),
            // "help" => ZeroShellCommands::Help(Help::from_str(command)),
            // "history" => ZeroShellCommands::History(History::from_str(command)),
            "pwd" => ZeroShellCommands::Pwd(Pwd::from_str(command)),
            "ls" => ZeroShellCommands::Ls(Ls::from_str(command)),
            _ => ZeroShellCommands::Unknown(Unknown::from_str(command)),
        }
    }
}

impl FromStr for ZeroShellCommands {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        Ok(ZeroShellCommands::from_str(command))
    }
}
