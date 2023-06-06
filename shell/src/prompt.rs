use ansi_term::Color;
use std::path::{Path, PathBuf};

// read the content of a file and return it as a string
pub fn read_file(path: &Path) -> std::io::Result<String> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
// get the branch name from the contents of the .git/HEAD file
pub fn get_branch_name(contents: &String) -> Option<&str> {
    let contents = contents.trim();
    if !contents.starts_with("ref: refs/heads/") {
        return None;
    }
    Some(contents.trim_start_matches("ref: refs/heads/"))
}
// find the git repository and return the branch name
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
// get the prompt
pub fn get_prompt() -> String {
    let cwd = std::env::current_dir().unwrap();
    if let Some(branch) = find_git_repository_return_branch_name(cwd.clone()) {
        return git_prompt(&cwd, &branch);
    }
    normal_prompt(&cwd)
}
// get the prompt for the directory that is not a git repository
fn normal_prompt(cwd: &Path) -> String {
    format!(
        "🛠️⚙️ {} ✗ ",
        cwd.display().to_string().split("/").last().unwrap(),
    )
}
// get the prompt for the directory that is a git repository
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
// cleanup the shell and exit
pub fn cleanup() -> ! {
    // Perform any necessary cleanup actions here
    println!("Cleaning up...");
    std::process::exit(0);
}
