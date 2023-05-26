mod cd;
mod exit;
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

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        assert_eq!(
            ZeroShellCommands::from_str(""),
            ZeroShellCommands::Unknown(Unknown::from_str(""))
        );
    }
    #[test]
    fn test_from_str() {
        assert_eq!(
            ZeroShellCommands::from_str("cd"),
            ZeroShellCommands::Cd(Cd::from_str("cd"))
        );
        assert_eq!(
            ZeroShellCommands::from_str("exit"),
            ZeroShellCommands::Exit(Exit::from_str("exit"))
        );
        assert_eq!(
            ZeroShellCommands::from_str("pwd"),
            ZeroShellCommands::Pwd(Pwd::from_str("pwd"))
        );
        assert_eq!(
            ZeroShellCommands::from_str("ls"),
            ZeroShellCommands::Ls(Ls::from_str("ls"))
        );
        assert_eq!(
            ZeroShellCommands::from_str("unknown"),
            ZeroShellCommands::Unknown(Unknown::from_str("unknown"))
        );
    }
}
