use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum ZeroShellCommandsError<T: Debug> {
    Cd(T),
    Exit(T),
    // HelpErr,
    // HistoryErr,
    Pwd(T),
    // Echo,
    // Cat,
    Ls(T),
    // Clear,
    // Rm,
    // Mkdir,
    // Touch,
    // Unknown(Unknown),
}
impl<T: Debug> std::fmt::Display for ZeroShellCommandsError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ZeroShellCommandsError::Exit(err) => write!(f, "{}", format!("{:?}", err)),
            ZeroShellCommandsError::Pwd(err) => write!(f, "{}", format!("{:?}", err)),
            ZeroShellCommandsError::Ls(err) => write!(f, "{}", format!("{:?}", err)),
            ZeroShellCommandsError::Cd(err) => write!(f, "{}", format!("{:?}", err)),
        }
    }
}

impl<T: Debug + Display> ZeroShellCommandsError<T> {
    pub fn handle_error(&self) {
        match self {
            err => eprintln!("{}", err),
        }
    }
}
