use std::{io::{self, Write}};

fn main() {
    let pkg_version = env!("CARGO_PKG_VERSION");
    println!("mimosa-tty ({})", pkg_version);

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush output.");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to get input.");

        let cleaned_input = input.trim();
        if cleaned_input.is_empty() { continue; }

        match handle_command(&cleaned_input) {
            Ok(result) => match result {
                CommandResult::Continue => (),
                CommandResult::Exit => break,
            },
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}

enum CommandResult {
    Continue,
    Exit
}

/// Parses input and runs a command
fn handle_command(user_input: &str) -> Result<CommandResult, String> {
    let mut parts = user_input.split_whitespace();
    let command = parts.next().unwrap();

    let mut result = CommandResult::Continue;

    match command {
        "exit" => {
            println!("Exiting the program...");
            result = CommandResult::Exit;
        },
        _ => println!("Unknown command")
    };

    Ok(result)
}