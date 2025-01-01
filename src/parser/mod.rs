use std::io::{self, Write};

use std::process::{Command, Stdio};
use regex::Regex;

pub fn parse_and_execute_line(mut line: String) -> bool {
    // Check if the line starts with "git "
    if let Some(trimmed) = line.strip_prefix("git ") {
        println!("Note: gitcmd does not require the 'git' prefix.");
        line = trimmed.to_string(); // Update `line` to be the string without the "git " prefix
    }

    // Regex to match either:
    // - Non-whitespace characters (word arguments)
    // - Or a quoted string (arguments inside double quotes)
    let re = Regex::new(r#""([^"]*)"|\S+"#).unwrap();
    
    // Find all the matches of words or quoted strings
    let args: Vec<String> = re
        .find_iter(&line)
        .map(|mat| mat.as_str().trim_matches('"').to_string()) // Remove quotes from matched strings
        .collect();

    // If there is no command, just return
    if args.is_empty() {
        println!("\r\n");
        return false;
    }

    // Create the command and add the common arguments
    let mut command = Command::new("git");
    command
        .arg("-c")
        .arg("color.ui=always") // Force git to always use color
        .arg("--no-pager"); // Prevent pager from stripping color

    // Add each argument from the line
    for arg in args {
        command.arg(arg);
    }

    // Execute the git command
    let output = command.stdout(Stdio::piped()).stderr(Stdio::piped()).output();

    match output {
        Ok(output) => {
            // Ensure the cursor is at the beginning of the line before printing
            if let Err(e) = write!(io::stdout(), "\r\n\x1b[K") {
                eprintln!("Failed to write to stdout: {}", e);
            }

            if output.status.success() {
                // Print stdout to terminal including colors
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if let Err(e) = writeln!(io::stdout(), "\r\x1b[K{}", line) {
                        eprintln!("Failed to write to stdout: {}", e);
                    }
                }
                if let Err(e) = io::stdout().flush() {
                    eprintln!("Failed to flush stdout: {}", e);
                }
            } else {
                // If command fails, print the stderr output
                eprintln!(
                    "Command failed with error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            // Handle the error if the command fails to execute
            eprintln!("Failed to run the command: {}", e);
        }
    }

    true
}
