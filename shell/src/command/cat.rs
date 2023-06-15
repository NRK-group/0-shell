use super::ZeroShellCommandsError;

#[derive(Debug, PartialEq, Eq)]
pub struct Cat {
    pub command: String,
    pub file: Vec<String>,
}

impl Cat {
    pub fn from_str(command: &str) -> Self {
        let file = command
            .split_whitespace()
            .skip(1)
            .map(|x| return x.to_string())
            .collect::<Vec<String>>();
        Self {
            command: command.to_string(),
            file,
        }
    }
    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        if !&self.file.is_empty() {
            for file in &self.file {
                match std::fs::read_to_string(file) {
                    Ok(content) => {
                        print!("{}", content);
                    }
                    Err(err) => {
                        return Err(ZeroShellCommandsError::Cat(err.to_string()));
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cd() {
        let cd = Cat::from_str("cat");
        // assert_eq!(cd., "cd");
        assert_eq!(cd.file, Vec::<String>::new());
    }
}
