use crate::config::setup_git_conf_profile;
use regex::Regex;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn parse_and_execute_line(line: String) -> bool {
    // Split the input line by && operator and trim whitespace
    let commands: Vec<&str> = line.split("&&").map(|s| s.trim()).collect();

    for cmd in commands {
        let mut line = cmd.to_string();
        // Check if the line starts with "git "
        if let Some(trimmed) = line.strip_prefix("git ") {
            println!("Note: gitcmd does not require the 'git' prefix.");
            line = trimmed.to_string();
        }

        // Regex to match either:
        // - Non-whitespace characters (word arguments)
        // - Or a quoted string (arguments inside double quotes)
        let re = Regex::new(r#""([^"]*)"|\S+"#).expect("FATAL: Failed to execute regex operation.");

        // Find all the matches of words or quoted strings
        let args: Vec<String> = re
            .find_iter(&line)
            .map(|mat| mat.as_str().trim_matches('"').to_string())
            .collect();

        // If there is no command, skip to next command
        if args.is_empty() {
            println!("\r\n");
            continue;
        } else if args.len() == 1 && args[0] == "setup" {
            setup_git_conf_profile();
            continue;
        }

        // Create the command and add the common arguments
        let mut command = Command::new("git");
        command.arg("-c").arg("color.ui=always").arg("--no-pager");

        // Add each argument from the line
        for arg in args {
            command.arg(arg);
        }

        // Execute the git command
        let output = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match output {
            Ok(output) => {
                // Ensure the cursor is at the beginning of the line before printing
                if let Err(e) = write!(io::stdout(), "\r\n\x1b[K") {
                    eprintln!("Failed to write to stdout: {}", e);
                }

                if !output.status.success() {
                    // If any command fails, print the error and stop execution
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    for line in stderr.lines() {
                        if let Err(e) = writeln!(io::stdout(), "\r\x1b[K{}", line) {
                            eprintln!("Failed to write to stdout: {}", e);
                        }
                    }
                    if let Err(e) = io::stdout().flush() {
                        eprintln!("Failed to flush stdout: {}", e);
                    }
                    // Return false to indicate failure
                    return false;
                }

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
            }
            Err(e) => {
                // Handle the error if the command fails to execute
                if let Err(write_err) =
                    writeln!(io::stdout(), "\r\x1b[KFailed to run the command: {}", e)
                {
                    eprintln!("Failed to write to stdout: {}", write_err);
                }
                return false;
            }
        }
    }
    true
}
