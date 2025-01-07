use crate::input_handler::InputHandler;
use std::{io, process::Command};

struct GitConfigEntry {
    title: String,
    example: String,
    possibles: Vec<String>,
    command: String, // Command template with `{}` placeholder
}

struct GitConfig {
    settings: Vec<GitConfigEntry>,
}

impl GitConfig {
    pub fn new() -> Self {
        GitConfig {
            settings: vec![
                GitConfigEntry {
                    title: String::from("Git User Name"),
                    example: String::from("Your Name"),
                    possibles: vec![String::from("Your Name")],
                    command: String::from("git config user.name '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Git User Email"),
                    example: String::from("youremail@example.com"),
                    possibles: vec![String::from("youremail@example.com")],
                    command: String::from("git config user.email '{}'"),
                },/*
                GitConfigEntry {
                    title: String::from("Git Pull Rebase"),
                    example: String::from("true"),
                    possibles: vec![
                        String::from("true"),
                        String::from("false"),
                        String::from("ff-only"),
                    ],
                    command: String::from("git config pull.rebase '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Git Editor"),
                    example: String::from("vim"),
                    possibles: vec![
                        String::from("vim"),
                        String::from("nano"),
                        String::from("code"),
                    ],
                    command: String::from("git config core.editor '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Core Auto CRLF"),
                    example: String::from("false"),
                    possibles: vec![
                        String::from("true"),
                        String::from("false"),
                        String::from("input"),
                    ],
                    command: String::from("git config core.autocrlf '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Core Ignore Case"),
                    example: String::from("true"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config core.ignorecase '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Core File Mode"),
                    example: String::from("true"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config core.filemode '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Merge Tool"),
                    example: String::from("vimdiff"),
                    possibles: vec![
                        String::from("vimdiff"),
                        String::from("meld"),
                        String::from("kdiff3"),
                    ],
                    command: String::from("git config merge.tool '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Diff Tool"),
                    example: String::from("vimdiff"),
                    possibles: vec![
                        String::from("vimdiff"),
                        String::from("meld"),
                        String::from("kdiff3"),
                    ],
                    command: String::from("git config diff.tool '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Commit GPG Sign"),
                    example: String::from("false"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config commit.gpgSign '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Init Default Branch"),
                    example: String::from("main"),
                    possibles: vec![String::from("main"), String::from("master")],
                    command: String::from("git config init.defaultBranch '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Push Default"),
                    example: String::from("simple"),
                    possibles: vec![
                        String::from("simple"),
                        String::from("current"),
                        String::from("matching"),
                    ],
                    command: String::from("git config push.default '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Fetch Prune"),
                    example: String::from("false"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config fetch.prune '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Core Pager"),
                    example: String::from("less"),
                    possibles: vec![String::from("less"), String::from("more")],
                    command: String::from("git config core.pager '{}'"),
                },*/
            ],
        }
    }

    pub fn display(&self) -> io::Result<()> {
        let mut input_handler = InputHandler::new_raw()?; // Initialize input handler
        let mut commands: Vec<String> = Vec::new(); // Vector to store commands

        input_handler.clear_screen().unwrap();
        for setting in &self.settings {
            input_handler.write_line("<-- gitcmd Setup -->\n")?;
            input_handler.write_line(&format!("\rSetting: {}", setting.title))?;
            input_handler.write_line(&format!("\rExample: {}", setting.example))?;
            input_handler.write_line(&format!("\rExample values: {:?}", setting.possibles))?;

            // Prompt the user for input
            let prompt = format!(
                "\rPlease choose a value (or press Enter to use default '{}'): ",
                setting.example
            );
            let input = input_handler.read_line(&prompt);

            let value_to_use = match input {
                Ok(Some(input)) => input,
                Ok(None) => return Ok(()), // Exit if the user presses Ctrl+C or cancels input
                Err(_) => return Ok(()), // Fail gracefully if there's an error in reading input
            };

            let value_to_use = if value_to_use.trim().is_empty() {
                setting.example.clone()
            } else {
                value_to_use
            };

            let command = setting.command.replace("{}", &value_to_use);
            commands.push(command.clone());
            input_handler.clear_screen().unwrap();
        }

        // Print the commands to be executed
        input_handler.write_line("The following commands will be run:\n")?;
        for comm in &commands {
            input_handler.write_line(&comm.to_string()).unwrap();
        }

        // Confirm before running the commands
        loop {
            let input = input_handler.read_line("\nNote: this will overwrite current gitconfig for any changed settings.\r\nProceed (Y/N)? ")?.unwrap_or_default();
            let proceed = if input.trim().is_empty() || input.trim().to_uppercase() == "Y" {
                Some(true)
            } else if input.trim().to_uppercase() == "N" {
                Some(false)
            } else {
                None
            };

            match proceed {
                Some(true) => {
                    for comm in &commands {
                        // Execute each command
                        let output = Command::new("sh") // Or "bash", depending on the shell available
                            .arg("-c")
                            .arg(comm)
                            .output();

                        match output {
                            Ok(output) => {
                                if !output.status.success() {
                                    input_handler.write_line(&format!(
                                        "Command failed: {}\nError: {}",
                                        comm,
                                        String::from_utf8_lossy(&output.stderr)
                                    ))?;
                                }
                            }
                            Err(e) => {
                                input_handler.write_line(&format!(
                                    "Failed to execute command: {}\nError: {}",
                                    comm, e
                                ))?;
                            }
                        }
                    }
                    input_handler.write_line("\nCommands Executed").unwrap();
                    break;
                }
                Some(false) => {
                    input_handler
                        .write_line("\nCommands not executed, Aborted.")
                        .unwrap();
                    break;
                }
                None => {
                    input_handler
                        .write_line("\nInvalid Input, please input 'Y' or 'N'.")
                        .unwrap();
                }
            }
        }

        Ok(())
    }
}

pub fn setup_git_conf_profile() {
    let config = GitConfig::new();
    config.display().unwrap();
}
