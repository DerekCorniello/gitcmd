use std::io::{self, Write};
use std::time::{Duration, Instant};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
mod config;
mod parser;
use crate::parser::parse_and_execute_line;

fn main() -> io::Result<()> {
    let mut prev_commands: Vec<String> = Vec::new();
    let mut history_index: Option<usize> = None;
    let stdin = io::stdin();
    let stdout = io::stdout().into_raw_mode()?;
    let mut stdout = io::BufWriter::new(stdout);
    let mut keys = stdin.keys();

    // Add variables to track Ctrl-C timing
    let mut last_ctrl_c: Option<Instant> = None;
    let ctrl_c_timeout = Duration::from_millis(500); // Adjust timeout as needed

    loop {
        write!(stdout, "\r\ngitcmd > ")?;
        stdout.flush()?;
        let mut input_line = String::new();
        let mut cursor_pos = 0;

        for key in keys.by_ref() {
            match key? {
                Key::Char('\n') => {
                    writeln!(stdout)?;
                    break;
                }
                Key::Char(c) => {
                    // Insert character at cursor position
                    input_line.insert(cursor_pos, c);
                    cursor_pos += 1;
                    
                    // Redraw the line from cursor position
                    write!(stdout, "{}", &input_line[cursor_pos-1..])?;
                    
                    // Move cursor back to the correct position if we're not at the end
                    if cursor_pos < input_line.len() {
                        write!(stdout, "\x1b[{}D", input_line.len() - cursor_pos)?;
                    }
                    stdout.flush()?;
                }
                Key::Backspace => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        input_line.remove(cursor_pos);
                        
                        // Move cursor back and redraw the rest of the line
                        write!(stdout, "\x08")?;  // Move back one
                        write!(stdout, "{} ", &input_line[cursor_pos..])?;  // Redraw rest of line plus space
                        
                        // Move cursor back to the correct position
                        write!(stdout, "\x1b[{}D", input_line.len() - cursor_pos + 1)?;
                        stdout.flush()?;
                    }
                }
                Key::Left => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        write!(stdout, "\x1b[D")?;  // Move cursor left
                        stdout.flush()?;
                    }
                }
                Key::Right => {
                    if cursor_pos < input_line.len() {
                        cursor_pos += 1;
                        write!(stdout, "\x1b[C")?;  // Move cursor right
                        stdout.flush()?;
                    }
                }
                Key::Up => {
                    if let Some(index) = history_index {
                        if index > 0 {
                            history_index = Some(index - 1);
                        }
                    } else if !prev_commands.is_empty() {
                        history_index = Some(prev_commands.len() - 1);
                    }
                    if let Some(index) = history_index {
                        input_line = prev_commands[index].clone();
                        cursor_pos = input_line.len();  // Move cursor to end of line
                        write!(stdout, "\r\x1b[Kgitcmd > {}", input_line)?;
                        stdout.flush()?;
                    }
                }
                Key::Down => {
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
                    cursor_pos = input_line.len();  // Move cursor to end of line
                    write!(stdout, "\r\x1b[Kgitcmd > {}", input_line)?;
                    stdout.flush()?;
                }
                Key::Ctrl('c') => {
                    let now = Instant::now();
                    
                    if let Some(last_time) = last_ctrl_c {
                        if now.duration_since(last_time) < ctrl_c_timeout {
                            // Double Ctrl-C detected within timeout
                            writeln!(stdout, "\r\nExiting...\r\n")?;
                            return Ok(());
                        }
                    }
                    
                    // Update last Ctrl-C time
                    last_ctrl_c = Some(now);
                }
                _ => {}
            }
        }

        let input = input_line.trim();
        if input == "exit" || input == "quit" {
            writeln!(stdout, "\r\nExiting...\r\n")?;
            break;
        }
        if !input.is_empty() {
            write!(stdout, "\r\x1b[K")?;
            stdout.flush()?;
            if parse_and_execute_line(input.to_string()) {
                prev_commands.push(input.to_string());
            }
        }
        history_index = None;
    }
    Ok(())
}
