use std::io;
use crate::input_handler::InputHandler;
use crate::input_parser::parse_and_execute_line;

pub fn terminal_loop() -> io::Result<()> {
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
            }
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

        if input == "clear" || input == "cls" {
            input_handler.clear_screen().unwrap();
        }
        if !input.is_empty() && parse_and_execute_line(input.to_string()) {
            prev_commands.push(input.to_string());
        }
        history_index = None;
        let mut index = 1;
        while index < prev_commands.len() {
            if prev_commands[index] == prev_commands[index - 1] {
                // if an element is removed because it is the same,
                // the remaining elements shift left, so the next element
                // after the one that was removed will automatically be at
                // the same index, which ensures that it gets checked.
                // this is safe
                prev_commands.remove(index);
            } else {
                // only move forward when no removal happens
                index += 1;
            }
        }
    }

    Ok(())
}
