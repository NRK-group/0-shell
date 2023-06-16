use std::path::Path;

use crate::utils::parse_generic_command;

use super::ZeroShellCommandsError;

#[derive(Debug, PartialEq, Eq)]
pub struct Rm {
    pub options: Vec<String>,
    pub file: Vec<String>,
}

impl Rm {
    pub fn from_str(command: &str) -> Self {
        let (_command, args) = parse_generic_command(command);
        let mut file = Vec::new();
        let mut new_args = Vec::<String>::new();
        if !args.is_empty() {
            if args[0].starts_with('-') {
                for arg in &args {
                    if arg.starts_with('-') {
                        new_args.push(arg.clone());
                        continue;
                    }
                    file.push(arg.clone());
                }
            } else {
                file = args;
            }
        }
        Self {
            options: new_args,
            file,
        }
    }
    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        for file in &self.file {
            //check if file is a directory or a file
            let path = Path::new(file);
            if path.is_dir() && !self.options.contains(&"-r".to_string()) {
                eprintln!("{}: is a directory", file);
                continue;
            } else if path.is_dir() && self.options.contains(&"-r".to_string()) {
                match std::fs::remove_dir_all(file) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                }
                continue;
            } else if path.is_file() {
                match std::fs::remove_file(file) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
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
    fn test_rm_from_str() {
        let rm = Rm::from_str("rm file");
        assert_eq!(rm.file, vec!["file".to_string()]);
    }
    #[test]
    fn test_rm_from_str_with_opt() {
        let rm = Rm::from_str("rm -r file");
        assert_eq!(rm.options, vec!["-r".to_string()]);
        assert_eq!(rm.file, vec!["file".to_string()]);
    }

    #[test]
    fn test_rm_from_str_with_multiple_files() {
        let rm = Rm::from_str("rm file1 file2 -r");
        assert_eq!(
            rm.file,
            vec!["file1".to_string(), "file2".to_string(), "-r".to_string()]
        );
    }
}
