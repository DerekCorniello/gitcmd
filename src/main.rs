use std::io::{self, Write};

mod parser;
use crate::parser::execute_line;

fn main() {
    let mut prev_commands: Vec<String> = Vec::new();
    loop {
        print!("gitcmd > ");

        // ensure the prompt is immediately visible
        // partial writes will not be sent to terminal due
        // to buffering, we must "flush it" to the terminal
        if let Err(e) = io::stdout().flush() {
            eprintln!("Error: Failed to display prompt: {}", e);
            continue; // continue the loop even if flush fails,
                      // this is not necessarily a "fatal" err
        }

        // reads in the user input
        let mut input_line = String::new();

        // breakdown:
        // read_line returns a `Result` type which is an
        // enum for Ok(_) and Err(e). Unlike go, it will
        // return only one, and we must match it and its type.
        // this can be done more verbosely using a
        // match statement, but using `if let` allows
        // us to match the type to the return type,
        // and still execute the code in the assignment
        // it only will check if there is an error or not
        if let Err(e) = io::stdin().read_line(&mut input_line) {
            eprintln!("Error: Failed to read input: {}", e);
            continue;
        }

        let input = input_line.trim(); // trim the input

        // check for exit command
        if input == "exit" {
            break;
        }
        println!();
        if execute_line(input.to_string()) {
            prev_commands.push(input.to_string())
        }
    }
}
