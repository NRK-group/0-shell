use std::env;

use super::ZeroShellCommandsError;

#[derive(Debug, PartialEq, Eq)]
pub struct Cd {
    pub path: String,
}
impl Cd {
    pub fn from_str(command: &str) -> Self {
        let path = command.split_whitespace().nth(1).unwrap_or("");
        Self {
            path: path.to_string(),
        }
    }
    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        if !&self.path.is_empty() {
            match env::set_current_dir(&self.path) {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => {
                    return Err(ZeroShellCommandsError::Cd(err.to_string()));
                }
            };
        }
        let home_dir = env::var("HOME").unwrap_or(String::new());
        match env::set_current_dir(home_dir) {
            Ok(_) => Ok(()),
            Err(err) => {
                return Err(ZeroShellCommandsError::Cd(err.to_string()));
            }
        }
    }
}
