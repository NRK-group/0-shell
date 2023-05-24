use std::io::{self, Write};

use execute::Execute;
use zeroshellcommands::ZeroShellCommands;
mod zeroshellcommandserror;
mod execute;
mod utils;
mod zeroshellcommands;
fn main() {
    loop {
        print_prompt();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // Reached EOF (Ctrl + D)
                break;
            }
            Ok(_) => {
                // Process the user input
                match ZeroShellCommands::from_str(&input).execute() {
                    Ok(_) => {}
                    Err(error) => {
                        error.handle_error();
                        continue;
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                continue;
            }
        }
    }

    // Clean up any resources or processes before exiting
    cleanup();
    // Ok(())
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
fn cleanup() {
    // Perform any necessary cleanup actions here
    println!("Cleaning up...");
}
