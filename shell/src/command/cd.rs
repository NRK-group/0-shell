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
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                return Err(ZeroShellCommandsError::Cd(err.to_string()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cd() {
        let cd = Cd::from_str("cd");
        // assert_eq!(cd., "cd");
        assert_eq!(cd.path, String::new());
    }
    #[test]
    fn test_cd_with_args() {
        let cd = Cd::from_str("cd /tmp");
        assert_eq!(cd.path, "/tmp".to_string());
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
