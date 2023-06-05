use ansi_term::Color;
use std::path::{Path, PathBuf};

pub fn read_file(path: &Path) -> std::io::Result<String> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn get_branch_name(contents: &String) -> Option<&str> {
    let contents = contents.trim();
    if !contents.starts_with("ref: refs/heads/") {
        return None;
    }
    Some(contents.trim_start_matches("ref: refs/heads/"))
}
pub fn find_git_repository_return_branch_name(mut path: PathBuf) -> Option<String> {
    fn is_git_repository(path: &Path) -> bool {
        let git_dir = path.join(".git");
        git_dir.exists() && git_dir.is_dir()
    }
    while !is_git_repository(&path) {
        if !path.pop() {
            return None;
        }
    }
    match read_file(&path.join(".git/HEAD")) {
        Ok(contents) => {
            // get the branch name
            if let Some(branch) = get_branch_name(&contents) {
                return Some(branch.to_string());
            }
            return None;
        }
        Err(_) => {
            return None;
        }
    };
}
pub fn get_prompt() -> String {
    let cwd = std::env::current_dir().unwrap();
    if let Some(branch) = find_git_repository_return_branch_name(cwd.clone()) {
        return git_prompt(&cwd, &branch);
    }
    normal_prompt(&cwd)
}
fn normal_prompt(cwd: &Path) -> String {
    format!(
        "🛠️⚙️ {} ✗ ",
        cwd.display().to_string().split("/").last().unwrap(),
    )
}
fn git_prompt(cwd: &Path, branch: &str) -> String {
    format!(
        "🛠️⚙️ 🤑 {} {}{}{} {} ",
        Color::Cyan
            .bold()
            .paint(cwd.display().to_string().split("/").last().unwrap()),
        Color::Blue.bold().paint("git:("),
        Color::Red.bold().paint(branch.split("/").last().unwrap()),
        Color::Blue.bold().paint(")"),
        Color::Yellow.bold().paint("✗")
    )
}
pub fn cleanup() -> ! {
    // Perform any necessary cleanup actions here
    println!("Cleaning up...");
    std::process::exit(0);
}
