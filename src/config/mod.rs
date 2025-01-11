use crate::input_handler::InputHandler;
use std::{io, process::Command};

struct GitConfigEntry {
    title: String,
    example: String,
    description: String,
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
                    command: String::from("git config --[scope] user.name '[value]'"),
                    description: String::from("Sets the name to be associated with Git commits."),
                },
                GitConfigEntry {
                    title: String::from("Git User Email"),
                    example: String::from("youremail@example.com"),
                    possibles: vec![String::from("youremail@example.com")],
                    command: String::from("git config --[scope] user.email '[value]'"),
                    description: String::from("Sets the email to be associated with Git commits."),
                },
                GitConfigEntry {
                    title: String::from("Git Pull Rebase"),
                    example: String::from("true"),
                    possibles: vec![
                        String::from("true"),
                        String::from("false"),
                        String::from("ff-only"),
                    ],
                    command: String::from("git config --[scope] pull.rebase '[value]'"),
                    description: String::from(
                        "Controls whether `git pull` uses rebase instead of merge.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Git Editor"),
                    example: String::from("vim"),
                    possibles: vec![
                        String::from("vim"),
                        String::from("nano"),
                        String::from("code"),
                    ],
                    command: String::from("git config --[scope] core.editor '[value]'"),
                    description: String::from("Sets the default editor for Git commands."),
                },
                GitConfigEntry {
                    title: String::from("Core Auto CRLF"),
                    example: String::from("false"),
                    possibles: vec![
                        String::from("true"),
                        String::from("false"),
                        String::from("input"),
                    ],
                    command: String::from("git config --[scope] core.autocrlf '[value]'"),
                    description: String::from(
                        "Controls automatic conversion of CRLF line endings.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Core Ignore Case"),
                    example: String::from("true"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config --[scope] core.ignorecase '[value]'"),
                    description: String::from(
                        "Configures case sensitivity in file names on the filesystem.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Core File Mode"),
                    example: String::from("true"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config --[scope] core.filemode '[value]'"),
                    description: String::from(
                        "Controls the tracking of file mode changes (permissions).",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Merge Tool"),
                    example: String::from("vimdiff"),
                    possibles: vec![
                        String::from("vimdiff"),
                        String::from("meld"),
                        String::from("kdiff3"),
                    ],
                    command: String::from("git config --[scope] merge.tool '[value]'"),
                    description: String::from("Sets the merge tool used for resolving conflicts."),
                },
                GitConfigEntry {
                    title: String::from("Diff Tool"),
                    example: String::from("vimdiff"),
                    possibles: vec![
                        String::from("vimdiff"),
                        String::from("meld"),
                        String::from("kdiff3"),
                    ],
                    command: String::from("git config --[scope] diff.tool '[value]'"),
                    description: String::from("Sets the diff tool used for comparing changes."),
                },
                GitConfigEntry {
                    title: String::from("Commit GPG Sign"),
                    example: String::from("false"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config --[scope] commit.gpgSign '[value]'"),
                    description: String::from("Configures whether to sign commits with a GPG key."),
                },
                GitConfigEntry {
                    title: String::from("Init Default Branch"),
                    example: String::from("main"),
                    possibles: vec![String::from("main"), String::from("master")],
                    command: String::from("git config --[scope] init.defaultBranch '[value]'"),
                    description: String::from(
                        "Sets the default branch name when initializing a new repository.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Push Default"),
                    example: String::from("simple"),
                    possibles: vec![
                        String::from("simple"),
                        String::from("current"),
                        String::from("matching"),
                    ],
                    command: String::from("git config push.default '--[scope]'"),
                    description: String::from(
                        "Determines the behavior of `git push` when no refspec is specified.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Fetch Prune"),
                    example: String::from("false"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config fetch.prune '--[scope]'"),
                    description: String::from(
                        "Controls whether `git fetch` automatically prunes deleted branches.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Core Pager"),
                    example: String::from("less"),
                    possibles: vec![String::from("less"), String::from("more")],
                    command: String::from("git config core.pager '--[scope]'"),
                    description: String::from("Sets the pager program used to view Git output."),
                },
            ],
        }
    }

    pub fn display(&self) -> io::Result<()> {
        let mut input_handler = InputHandler::new_raw()?;
        let mut commands: Vec<String> = Vec::new();
        let mut config_scope: String = String::new();

        input_handler.clear_screen().unwrap();
        input_handler.write_str(
            "Choose the scope of your setup:\r\n\
            \tlocal: the config is local to this git repo\r\n\
            \tglobal: the config for your user, across all repos \r\n\
            \tsystem: the config for all users, across all users and repos\r\n\n",
        ).unwrap();

        loop {
            let scope_input =
                input_handler.read_line("Please input `local`, `global`, or `system`!");

            match scope_input {
                Ok(Some(scope)) => {
                    let scope = scope.trim();
                    if ["local", "global", "system"].contains(&scope) {
                        config_scope = scope.to_string();
                        break;
                    } else {
                        input_handler.write_str(&format!("Invalid input `{}`.\r\n", config_scope)).unwrap();
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        for setting in &self.settings {
            input_handler.write_line("<-- gitcmd Setup -->\n")?;
            input_handler.write_line(&format!("\rSetting: {}", setting.title))?;
            input_handler.write_line(&format!("\rDescription: {}", setting.description))?;
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
                Err(_) => return Ok(()),   // Fail gracefully if there's an error in reading input
            };

            let value_to_use = if value_to_use.trim().is_empty() {
                setting.example.clone()
            } else {
                value_to_use
            };

            let command = setting
                .command
                .replace("[value]", &value_to_use)
                .replace("[scope]", &config_scope);
            commands.push(command.clone());
            input_handler.clear_screen().unwrap();
        }

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
                        let output = Command::new("sh").arg("-c").arg(comm).output();

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
