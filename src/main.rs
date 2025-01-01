use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod parser;
use crate::parser::parse_and_execute_line;

fn main() -> io::Result<()> {
    let mut prev_commands: Vec<String> = Vec::new();
    let mut history_index: Option<usize> = None;

    let stdin = io::stdin();
    let stdout = io::stdout().into_raw_mode()?; // Enable raw mode
    let mut stdout = io::BufWriter::new(stdout); // Use buffered output for efficiency
    let mut keys = stdin.keys(); // Create the keys iterator once

    loop {
        write!(stdout, "\r\ngitcmd > ")?;
        stdout.flush()?;

        let mut input_line = String::new();

        for key in keys.by_ref() {
            match key? {
                Key::Char('\n') => {
                    // Enter key: process the command
                    writeln!(stdout)?;
                    break;
                }
                Key::Char(c) => {
                    // Regular character: add to input
                    input_line.push(c);
                    write!(stdout, "{}", c)?;
                    stdout.flush()?;
                }
                Key::Backspace => {
                    // Backspace: remove the last character
                    if !input_line.is_empty() {
                        input_line.pop();
                        write!(stdout, "\x08 \x08")?; // Move cursor back, overwrite with space, move back again
                        stdout.flush()?;
                    }
                }
                Key::Up => {
                    // Up arrow: browse history
                    if let Some(index) = history_index {
                        if index > 0 {
                            history_index = Some(index - 1);
                        }
                    } else if !prev_commands.is_empty() {
                        history_index = Some(prev_commands.len() - 1);
                    }
                    if let Some(index) = history_index {
                        input_line = prev_commands[index].clone();
                        write!(stdout, "\r\x1b[Kgitcmd > {}", input_line)?; // Clear line and reprint prompt with input
                        stdout.flush()?;
                    }
                }
                Key::Down => {
                    // Down arrow: browse history forward
                    if let Some(index) = history_index {
                        if index + 1 < prev_commands.len() {
                            history_index = Some(index + 1);
                        } else {
                            history_index = None;
                        }
                    }
                    input_line = history_index
                        .map(|index| prev_commands[index].clone())
                        .unwrap_or_default();
                    write!(stdout, "\r\x1b[Kgitcmd > {}", input_line)?; // Clear line and reprint prompt with input
                    stdout.flush()?;
                }
                Key::Ctrl('c') => {
                    // Handle Ctrl+C to exit
                    writeln!(stdout, "\r\nExiting...")?;
                    return Ok(());
                }
                _ => {}
            }
        }

        let input = input_line.trim();
        if input == "exit" {
            writeln!(stdout, "\r\nExiting...")?;
            break;
        }

        if !input.is_empty() {
            // Clear the current prompt line before executing commands
            write!(stdout, "\r\x1b[K")?; // This clears the line properly
            stdout.flush()?;

            // Execute the command line and print output line by line
            if parse_and_execute_line(input.to_string()) {
                prev_commands.push(input.to_string());
            }
        }

        history_index = None; // Reset history browsing index after command execution
    }

    Ok(())
}
