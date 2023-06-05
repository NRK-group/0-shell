use rustyline::{error::ReadlineError, DefaultEditor};
use shell::{cleanup, get_prompt, run_process};
fn main() -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new().unwrap();
    let dir = std::env::var("HOME").unwrap_or(".".to_string()) + "/.zero_shell_history";
    if rl.load_history(&dir).is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(&get_prompt());
        rl.save_history(&dir)?;
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.clone())?;
                run_process(&line).unwrap();
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    // Clean up any resources or processes before exiting
    cleanup();
}
