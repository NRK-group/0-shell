use std::{fs, io, path::Path};

use super::ZeroShellCommandsError;

#[derive(Debug, PartialEq, Eq)]
pub struct Cp {
    pub source: Vec<String>,
    pub destination: Option<String>,
}

impl Cp {
    pub fn from_str(command: &str) -> Self {
        let mut command = command
            .split_whitespace()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        if command.len() < 3 {
            return Self {
                source: command[1..command.len()].to_vec(),
                destination: None,
            };
        }
        Self {
            source: command[1..command.len() - 1].to_vec(),
            destination: command.pop(),
        }
    }

    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        dbg!(self.source.clone());
        if !&self.source.is_empty() {
            if let Some(destination) = &self.destination {
                let path = Path::new(&destination);
                for source in &self.source {
                    let source = Path::new(&source);
                    if source.is_dir() {
                        if !path.is_dir() {
                            println!("{} is not a directory", path.display());
                            return Ok(());
                        }
                        let folder_name = source.file_name().ok_or_else(|| {
                            ZeroShellCommandsError::Cp("Invalid source path".to_string())
                        })?;
                        let path = path.join(folder_name);
                        fs::create_dir_all(&path.as_path()).map_err(|err| {
                            ZeroShellCommandsError::Cp(format!(
                                "Failed to create directory: {}",
                                err
                            ))
                        })?;
                        match copy_folder(source, path.as_path()) {
                            Ok(_) => {}
                            Err(err) => {
                                return Err(ZeroShellCommandsError::Cp(format!(
                                    "Failed to copy folder: {}",
                                    err
                                )))
                            }
                        };
                    } else {
                        if path.is_dir() {
                            let file_name = source.file_name().ok_or_else(|| {
                                ZeroShellCommandsError::Cp("Invalid source path".to_string())
                            })?;
                            let path = path.join(file_name);
                            fs::copy(source, path).map_err(|err| {
                                ZeroShellCommandsError::Cp(format!("Failed to copy file: {}", err))
                            })?;
                            continue;
                        } else {
                            fs::copy(source, destination).map_err(|err| {
                                ZeroShellCommandsError::Cp(format!("Failed to copy file: {}", err))
                            })?;
                        }
                    }
                }
            } else {
                println!("No destination specified");
                return Ok(());
            }
        } else {
            println!("No source files specified");
            return Ok(());
        }
        Ok(())
    }
}

fn copy_folder(source: &Path, destination: &Path) -> io::Result<()> {
    // Create the destination folder if it doesn't exist
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap();
        let destination_path = destination.join(&file_name);

        if entry_path.is_file() {
            // Copy the file to the destination folder
            match fs::copy(&entry_path, &destination_path) {
                Ok(_) => {
                    println!("Copied file {:?} to {:?}", entry_path, destination_path);
                }
                Err(e) => {
                    println!("Failed to copy file {:?}: {}", entry_path, e);
                    return Err(e);
                }
            };
        } else if entry_path.is_dir() {
            // Recursively copy subdirectories
            match copy_folder(&entry_path, &destination_path) {
                Ok(_) => {
                    println!("Copied folder {:?} to {:?}", entry_path, destination_path);
                }
                Err(e) => {
                    println!("Failed to copy folder {:?}: {}", entry_path, e);
                    return Err(e);
                }
            };
        }
    }

    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cp_from_str() {
        let cp = Cp::from_str("cp /hello.txt /hello2.txt");
        assert_eq!(cp.source, vec!["/hello.txt".to_string()]);
        assert_eq!(cp.destination.unwrap(), "/hello2.txt");
    }
    #[test]
    fn test_cp_from_str_folder() {
        let cp = Cp::from_str("cp /hello.txt /hello2.txt hello/");
        assert_eq!(
            cp.source,
            vec!["/hello.txt".to_string(), "/hello2.txt".to_string()]
        );
        assert_eq!(cp.destination.unwrap(), "hello/");
    }
}
