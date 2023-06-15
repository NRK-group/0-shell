use super::ZeroShellCommandsError;

#[derive(Debug, PartialEq, Eq)]
pub struct Echo {
    args: Vec<String>,
}

impl Echo {
    pub fn from_str(command: &str) -> Self {
        let args = command
            .split_whitespace()
            .skip(1)
            .map(|s| s.to_string())
            .collect();
        Self { args }
    }
    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        println!(
            "{}",
            format!(
                "{}",
                self.args
                    .iter()
                    .fold(String::new(), |acc, s| acc + s.trim_matches('"') + " ")
            )
        );
        Ok(())
    }
}
