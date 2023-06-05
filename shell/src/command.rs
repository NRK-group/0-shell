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
