use crate::{cleanup, command::*};
use std::path::{Path, PathBuf};

//
pub fn run_process(command: &str) -> Result<(), ()> {
    if let Ok(zc) = ZeroShellCommands::from_str(&command) {
        zc.execute().unwrap();
        return Ok(());
    }

    let command = Command::new(command);

    let bin = match find_binary(&command) {
        Ok(bin) => bin,
        Err(_) => {
            println!("Command not found {:?}", &command);
            return Ok(());
        }
    };

    let mut child = std::process::Command::new(bin)
        .args(command.0.split_whitespace().skip(1))
        .envs(std::env::vars())
        .spawn()
        .expect("Failed to execute command");

    let status = child.wait().expect("Failed to wait for command");
    if !status.success() {
        println!(
            "Command failed with exit code: {}",
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}

fn find_binary(command: &Command) -> Result<PathBuf, std::io::Error> {
    fn search(target: &str, path: &Path) -> Result<(), std::io::Error> {
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                search(target, &path)?;
            } else if path.is_file() {
                if let Some(filename) = path.file_name() {
                    if filename == target {
                        if path.is_symlink() {
                            let path = path.read_link()?;
                            return search(target, &path);
                        } else {
                            return Ok(());
                        }
                    }
                }
            }
        }
        Err(std::io::ErrorKind::NotFound.into())
    }

    let target = command.bin_path();

    if let Ok(mut dir) = std::env::current_dir() {
        if let Ok(()) = search(target, &dir) {
            dir.push(target);
            return Ok(dir);
        }
    }

    for entry in std::env::var("PATH").unwrap_or(String::new()).split(":") {
        let mut path = PathBuf::from(entry);
        if let Ok(()) = search(target, &path) {
            path.push(target);
            return Ok(path);
        }
    }

    Err(std::io::ErrorKind::NotFound.into())
}

pub trait Execute {
    fn execute(&self) -> Result<(), ZeroShellCommandsError<String>>;
}

impl Execute for ZeroShellCommands {
    fn execute(&self) -> Result<(), ZeroShellCommandsError<String>> {
        match self {
            ZeroShellCommands::Cd(cd) => cd.execute(),
            ZeroShellCommands::Ls(ls) => ls.execute(),
            ZeroShellCommands::Pwd(pwd) => pwd.execute(),
            ZeroShellCommands::Cat(cat) => cat.execute(),
            ZeroShellCommands::Echo(echo) => echo.execute(),
            ZeroShellCommands::Rm(rm) => rm.execute(),
            ZeroShellCommands::Cp(cp) => cp.execute(),
            ZeroShellCommands::Mkdir(mkdir) => mkdir.execute(),
            ZeroShellCommands::Mv(mv) => mv.execute(),
            ZeroShellCommands::Exit => cleanup(),
        }
    }
}
