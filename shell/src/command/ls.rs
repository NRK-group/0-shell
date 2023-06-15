use std::{fs, os::unix::fs::MetadataExt, str::FromStr};

use chrono::{DateTime, Local};
use users::{get_group_by_gid, get_user_by_uid};

use super::ZeroShellCommandsError;
use crate::utils::{format_permissions, identify_file_type, parse_generic_command};
#[derive(Debug, PartialEq, Eq)]
pub struct Ls {
    pub command: String,
    pub args: Vec<String>,
    pub file: Vec<String>,
}

impl Ls {
    pub fn from_str(command: &str) -> Self {
        let (command, args) = parse_generic_command(command);
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
            command,
            args: new_args,
            file,
        }
    }

    pub fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        let show_hidden = self.args.contains(&String::from("-a"));
        let long_format = self.args.contains(&String::from("-l"));
        let show_symbols = self.args.contains(&String::from("-F"));
        let mut files: Vec<String> = self.file.clone();
        let len_files = &files.len();
        if files.is_empty() {
            files.push(".".to_string());
        }
        for (i, file) in files.iter().enumerate() {
            if len_files > &1 {
                println!("{}:", file);
            }
            let entries = match fs::read_dir(file) {
                Ok(entries) => entries,
                Err(_) => {
                    eprintln!("No such file or directory");
                    continue;
                }
            };
            let entries2 = match fs::read_dir(file) {
                Ok(entries) => entries,
                Err(_) => {
                    eprintln!("No such file or directory");
                    continue;
                }
            };
            let total_blocks = entries2
                .map(|entry| {
                    entry
                        .and_then(|entry| entry.metadata())
                        .map(|metadata| metadata.blocks())
                        .unwrap_or(0)
                })
                .sum::<u64>();
            if len_files > &1 {
                println!("total {}", total_blocks);
            }
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = match entry.file_name().into_string() {
                        Ok(file_name) => file_name,
                        Err(error) => {
                            eprintln!("{:?}", error);
                            continue;
                        }
                    };
                    if !show_hidden && file_name.starts_with('.') {
                        continue;
                    }
                    let metadata = match entry.metadata() {
                        Ok(metadata) => metadata,
                        Err(err) => {
                            return Err(ZeroShellCommandsError::Ls(err.to_string()));
                        }
                    };
                    let file_type = metadata.file_type();
                    if long_format {
                        // Owner and group names
                        let owner = match get_user_by_uid(metadata.uid()) {
                            Some(owner) => owner,
                            None => {
                                return Err(ZeroShellCommandsError::Ls(format!(
                                    "Error getting user by uid {}",
                                    metadata.uid()
                                )))
                            }
                        };
                        let group = match get_group_by_gid(metadata.gid()) {
                            Some(group) => group,
                            None => {
                                return Err(ZeroShellCommandsError::Ls(format!(
                                    "Error getting group by gid {}",
                                    metadata.gid()
                                )))
                            }
                        };
                        // Last modified time
                        let last_modified: DateTime<Local> =
                            DateTime::from(match metadata.modified() {
                                Ok(time) => time,
                                Err(err) => {
                                    return Err(ZeroShellCommandsError::Ls(err.to_string()))
                                }
                            });
                        // Print the file information
                        print!(
                            "{:<10} {:<3} {:<8} {:<8} {:<5} {} {}",
                            format_permissions(&metadata),
                            metadata.nlink(),
                            owner.name().to_string_lossy(),
                            group.name().to_string_lossy(),
                            metadata.len(),
                            last_modified.format("%b %e %H:%M").to_string(),
                            file_name
                        );
                    } else {
                        print!("{}", file_name);
                    }

                    if show_symbols {
                        // Symbolic indicators for file type
                        let symbolic_indicator = identify_file_type(&metadata, &file_type);
                        print!("{}", symbolic_indicator);
                    }
                }
                println!();
            }
            if i < files.len() - 1 {
                println!();
            }
        }
        Ok(())
    }
}

impl FromStr for Ls {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        Ok(Ls::from_str(command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_ls_from_str() {
        let ls = Ls::from_str("ls");
        assert_eq!(
            ls,
            Ls {
                command: String::from("ls"),
                args: vec![],
                file: vec![]
            }
        );
    }
    #[test]
    fn test_ls_from_str_with_opt() {
        let ls = Ls::from_str("ls -l -a -F");
        assert_eq!(
            ls,
            Ls {
                command: String::from("ls"),
                args: vec![String::from("-l"), String::from("-a"), String::from("-F")],
                file: vec![]
            }
        );
    }
    #[test]
    fn test_ls_from_str_with_opt_and_file() {
        let ls = Ls::from_str("ls -l -a -F /tmp");
        assert_eq!(
            ls,
            Ls {
                command: String::from("ls"),
                args: vec![String::from("-l"), String::from("-a"), String::from("-F")],
                file: vec![String::from("/tmp")]
            }
        );
    }

    #[test]
    fn test_ls_from_str_with_file_and_opt() {
        let ls = Ls::from_str("ls /tmp -l -a -F");
        assert_eq!(
            ls,
            Ls {
                command: String::from("ls"),
                args: vec![],
                file: vec![
                    String::from("/tmp"),
                    String::from("-l"),
                    String::from("-a"),
                    String::from("-F")
                ]
            }
        );
    }
    #[test]
    pub fn test_ls_execute_ok() {
        let ls = Ls::from_str("ls");
        let result = ls.execute();
        assert!(result.is_ok(), "Expected Ok result");
    }
    #[test]
    pub fn test_ls_execute_with_file_ok() {
        let ls = Ls::from_str("ls /tmp");
        let result = ls.execute();
        assert!(result.is_ok(), "Expected Ok result");
    }
    #[test]
    pub fn test_ls_execute_with_file_err() {
        let ls = Ls::from_str("ls l");
        let result = ls.execute();
        // this still prints the error message to stderr
        assert!(result.is_ok(), "Expected Ok result");
    }
}
