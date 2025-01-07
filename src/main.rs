use std::io;
mod config;
mod input_handler;
mod parser;
use crate::input_handler::InputHandler;
use crate::parser::parse_and_execute_line;

fn main() -> io::Result<()> {
    let mut prev_commands: Vec<String> = Vec::new();
    let mut history_index: Option<usize> = None;
    let mut input_handler = InputHandler::new_raw()?;

    loop {
        println!();
        let input = match input_handler.read_line_with_history(
            "\rgitcmd > ",
            &prev_commands,
            &mut history_index,
        )? {
            Some(input) => {
                prev_commands.push(input.clone());
                input
            },
            None => {
                input_handler.write_line("\r\nExiting...\r\n")?;
                break;
            }
        };

        let input = input.trim();
        if input == "exit" || input == "quit" {
            input_handler.write_line("\r\nExiting...\r\n")?;
            break;
        }
        if !input.is_empty() && parse_and_execute_line(input.to_string()) {
            prev_commands.push(input.to_string());
        }
        history_index = None;
    }

    Ok(())
}
