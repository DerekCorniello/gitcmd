use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn parse_and_execute_line(mut line: String) -> bool {
    // Check if the line starts with "git "
    if let Some(trimmed) = line.strip_prefix("git ") {
        println!("Note: gitcmd does not require the 'git' prefix.");
        line = trimmed.to_string(); // Update `line` to be the string without the "git " prefix
    }

    // Split the input line by whitespace into a vector of arguments
    let args: Vec<&str> = line.split_whitespace().collect();

    // If there is no command, just newline
    if args.is_empty() {
        println!("\r\n");
        return false;
    }

    // Execute the git command
    let command = Command::new("git")
        .arg("-c")
        .arg("color.ui=always") // Force git to always use color
        .arg("--no-pager") // Prevent pager from stripping color
        .arg(&line) // Add the line (git subcommand, e.g., "status")
        .stdout(Stdio::piped()) // Capture the output
        .stderr(Stdio::piped()) // Capture error output
        .output();

    match command {
        Ok(output) => {
            // Ensure the cursor is at the beginning of the line before printing
            if let Err(e) = write!(io::stdout(), "\r\n\x1b[K") {
                eprintln!("Failed to write to stdout: {}", e);
                return false;
            }

            if output.status.success() {
                // Print stdout to terminal including colors
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if let Err(e) = writeln!(io::stdout(), "\r\x1b[K{}", line) {
                        eprintln!("Failed to write to stdout: {}", e);
                        return false;
                    }
                }
                if let Err(e) = io::stdout().flush() {
                    eprintln!("Failed to flush stdout: {}", e);
                    return false;
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
            return false;
        }
    }

    true
}
