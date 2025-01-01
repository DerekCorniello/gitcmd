use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn execute_line(line: String) -> bool {
    // Split the input line by whitespace
    let args: Vec<&str> = line.split_whitespace().collect();

    // Check if any of the words is "git"
    if args.contains(&"git") {
        println!("This is the gitcmd line, don't need that funny word");
        return false;
    }

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
            if output.status.success() {
                // Print stdout to terminal including colors
                io::stdout()
                    .write_all(&output.stdout)
                    .expect("Failed to write stdout");
                io::stdout().flush().expect("Failed to flush stdout");
            } else {
                eprintln!(
                    "Command failed with error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to run the command: {}", e);
        }
    }

    true
}
