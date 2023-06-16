use std::fs;

use crate::utils::parse_generic_command;

use super::ZeroShellCommandsError;

#[derive(Debug, PartialEq, Eq)]
pub struct Mkdir {
    pub file: Vec<String>,
}

impl Mkdir {
    pub fn from_str(command: &str) -> Self {
        let (_command, file) = parse_generic_command(command);
        Self { file }
    }
    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        for file in &self.file {
            match fs::create_dir(file) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mkdir_execute() {
        let mkdir = Mkdir::from_str("mkdir test");
        assert_eq!(mkdir.execute(), Ok(()));
        assert!(std::path::Path::new("test").exists());
        std::fs::remove_dir("test").unwrap();
    }

    #[test]
    fn test_mkdir_execute_multiple() {
        let mkdir = Mkdir::from_str("mkdir test1 test2 test3");
        assert_eq!(mkdir.execute(), Ok(()));
        assert!(std::path::Path::new("test1").exists());
        assert!(std::path::Path::new("test2").exists());
        assert!(std::path::Path::new("test3").exists());
        std::fs::remove_dir_all("test1").unwrap();
        std::fs::remove_dir_all("test2").unwrap();
        std::fs::remove_dir_all("test3").unwrap();
    }

    #[test]
    fn test_mkdir_execute_error() {
        let mkdir = Mkdir::from_str("mkdir test/HELLO/WORLD.txt");
        assert_eq!(mkdir.execute(), Ok(())); // TODO: Fix this test case, it should handle the error that is printed to the console
    }
}
