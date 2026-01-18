use crate::config_io::{
    import_git_config, source_gitcmd_conf, write_gitcmd_conf, GitCmdAlias, GitCmdConfig, GitConfig,
};
use crate::input_handler::InputHandler;

use dirs::home_dir;
use std::io;
use std::path::PathBuf;

struct GitConfigEntry {
    title: String,
    example: String,
    description: String,
    possibles: Vec<String>,
    identifier: String,
}

struct GitConfigSettings {
    settings: Vec<GitConfigEntry>,
}

impl GitConfigSettings {
    pub fn new() -> Self {
        GitConfigSettings {
            settings: vec![
                GitConfigEntry {
                    title: String::from("Git User Name"),
                    example: String::from("Your Name"),
                    possibles: vec![String::from("Your Name")],
                    identifier: String::from("user.name"),
                    description: String::from("Sets the name to be associated with Git commits."),
                },
                GitConfigEntry {
                    title: String::from("Git User Email"),
                    example: String::from("youremail@example.com"),
                    possibles: vec![String::from("youremail@example.com")],
                    identifier: String::from("user.email"),
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
                    identifier: String::from("pull.rebase"),
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
                    identifier: String::from("core.editor"),
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
                    identifier: String::from("core.autocrlf"),
                    description: String::from(
                        "Controls automatic conversion of CRLF line endings.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Core Ignore Case"),
                    example: String::from("true"),
                    possibles: vec![String::from("true"), String::from("false")],
                    identifier: String::from("core.ignorecase"),
                    description: String::from(
                        "Configures case sensitivity in file names on the filesystem.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Core File Mode"),
                    example: String::from("true"),
                    possibles: vec![String::from("true"), String::from("false")],
                    identifier: String::from("core.filemode"),
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
                    identifier: String::from("merge.tool"),
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
                    identifier: String::from("diff.tool"),
                    description: String::from("Sets the diff tool used for comparing changes."),
                },
                GitConfigEntry {
                    title: String::from("Commit GPG Sign"),
                    example: String::from("false"),
                    possibles: vec![String::from("true"), String::from("false")],
                    identifier: String::from("commit.gpgSign"),
                    description: String::from("Configures whether to sign commits with a GPG key."),
                },
                GitConfigEntry {
                    title: String::from("Init Default Branch"),
                    example: String::from("main"),
                    possibles: vec![String::from("main"), String::from("master")],
                    identifier: String::from("init.defaultBranch"),
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
                    identifier: String::from("push.default"),
                    description: String::from(
                        "Determines the behavior of `git push` when no refspec is specified.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Fetch Prune"),
                    example: String::from("false"),
                    possibles: vec![String::from("true"), String::from("false")],
                    identifier: String::from("fetch.prune"),
                    description: String::from(
                        "Controls whether `git fetch` automatically prunes deleted branches.",
                    ),
                },
                GitConfigEntry {
                    title: String::from("Core Pager"),
                    example: String::from("less"),
                    possibles: vec![String::from("less"), String::from("more")],
                    identifier: String::from("core.pager"),
                    description: String::from("Sets the pager program used to view Git output."),
                },
            ],
        }
    }

    pub fn set_import_options(&self) -> Vec<String> {
        let mut input_handler =
            InputHandler::new_raw().expect("FATAL: Failed to create input handler.");
        let mut config_scope: Vec<String> = Vec::new(); // To store the selected scopes
        let mut input: String;

        input_handler
            .clear_screen()
            .expect("FATAL: Failed to clear screen.");
        input_handler
            .write_str(
                "Would you like to import your existing .gitconfig files?\r\n\
                Enter a list of the following choices:\r\n\
                \t1. local\r\n\
                \t2. global\r\n\
                \t3. system\r\n\
                \t4. do not import\r\n",
            )
            .unwrap();

        loop {
            let raw_input = input_handler
                .read_line("Please enter a choice (1, 2, 3 or 4): ")
                .expect("FATAL: Failed to read from stdin.");

            match raw_input {
                Some(val) => {
                    input = val;
                }
                None => {
                    return Vec::new();
                }
            }

            let choices = input.split(',').map(|s| s.trim()).collect::<Vec<&str>>();

            // Ensure that all the choices are valid
            let mut valid_choices = true;
            for choice in &choices {
                match *choice {
                    "1" => config_scope.push("local".to_string()),
                    "2" => config_scope.push("global".to_string()),
                    "3" => config_scope.push("system".to_string()),
                    "4" => config_scope.push("none".to_string()),
                    _ => {
                        valid_choices = false;
                        break;
                    }
                }
            }

            if valid_choices {
                break;
            } else {
                input_handler.write_str("Invalid input. Please enter a valid list (1, 2, 3) or press Enter to start fresh.\r\n").expect("FATAL: Failed to write to stdout.");
            }
        }

        if config_scope.is_empty() {
            input_handler
                .write_str("Starting fresh without importing any .gitconfig files.\r\n")
                .expect("FATAL: Failed to write to stdout.");
        } else {
            input_handler
                .write_str(&format!(
                    "You selected the following scopes: {:?}\r\n",
                    config_scope
                ))
                .expect("FATAL: Failed to write to stdout.");
        }

        config_scope
    }

    pub fn set_creation_scope(&self) -> String {
        let mut config_scope: String = String::new();
        let mut input_handler =
            InputHandler::new_raw().expect("FATAL: Failed to create input handler.");

        input_handler
            .clear_screen()
            .expect("FATAL: Failed to clear screen.");
        input_handler
            .write_str(
                "Choose the scope of your setup:\r\n\
            \tlocal: the config is local to this git repo\r\n\
            \tglobal: the config for your user, across all repos \r\n\
            \tsystem: the config for all users, across all users and repos\r\n\n",
            )
            .expect("FATAL: Failed to write to stdout.");

        loop {
            let scope_input =
                input_handler.read_line("Please input `local`, `global`, or `system`! ");

            match scope_input {
                Ok(Some(scope)) => {
                    let scope = scope.trim();
                    if ["local", "global", "system"].contains(&scope) {
                        config_scope = scope.to_string();
                        break;
                    } else {
                        input_handler
                            .write_str(&format!("Invalid input `{}`.\r\n", config_scope))
                            .expect("FATAL: Failed to write to stdout.");
                    }
                }
                Ok(None) => {
                    return String::new();
                }
                _ => {
                    continue;
                }
            }
        }
        config_scope
    }
    pub fn display(&self, edit_mode: bool, target_scope: &str) -> io::Result<()> {
        use crate::config_io::get_git_config_value;

        let mut input_handler = InputHandler::new_raw()?;
        let mut git_configs: Vec<GitConfig> = Vec::new();
        let mut git_cmd_aliases: Vec<GitCmdAlias> = Vec::new();

        let config_scope: String;

        if edit_mode {
            if target_scope.is_empty() {
                config_scope = self.set_creation_scope();
                if config_scope.is_empty() {
                    return Ok(());
                }
            } else {
                config_scope = target_scope.to_string();
            }
        } else {
            let mut import_scope = self.set_import_options();
            if import_scope.is_empty() {
                return Ok(());
            }
            if import_scope.contains(&"none".to_string()) {
                import_scope = Vec::new();
            }
            let _import_settings = import_git_config(import_scope).unwrap_or_default();
            config_scope = self.set_creation_scope();
            if config_scope.is_empty() {
                return Ok(());
            }
        }

        input_handler.clear_screen()?;

        for setting in &self.settings {
            let header = if edit_mode {
                "<-- gitcmd Edit -->"
            } else {
                "<-- gitcmd Setup -->"
            };
            input_handler.write_line(header)?;
            input_handler.write_line(&format!("\rSetting: {}", setting.title))?;
            input_handler.write_line(&format!("\rDescription: {}", setting.description))?;

            let current_value = get_git_config_value(&config_scope, &setting.identifier);

            if let Some(ref val) = current_value {
                input_handler.write_line(&format!("\rCurrent value: {}", val))?;
            }

            input_handler.write_line(&format!("\rExample: {}", setting.example))?;
            input_handler.write_line(&format!("\rExample values: {:?}", setting.possibles))?;

            let default_value = current_value
                .as_ref()
                .cloned()
                .unwrap_or_else(|| setting.example.clone());

            if edit_mode {
                let prompt = format!("\r[Enter] to keep '{}', or type new value: ", default_value);
                let input = input_handler.read_line(&prompt)?;

                let value_to_use = match input {
                    Some(input) => {
                        if input.trim().is_empty() {
                            if let Some(ref val) = current_value {
                                val.clone()
                            } else {
                                setting.example.clone()
                            }
                        } else {
                            input.trim().to_string()
                        }
                    }
                    None => return Ok(()),
                };

                if value_to_use != default_value || current_value.is_none() {
                    git_configs.push(GitConfig::new(&setting.identifier, &value_to_use));
                }
            } else {
                let prompt = format!(
                    "\rPlease choose a value (or press Enter to use '{}'): ",
                    default_value
                );
                let input = input_handler.read_line(&prompt)?;

                let value_to_use = match input {
                    Some(input) => {
                        if input.trim().is_empty() {
                            default_value
                        } else {
                            input
                        }
                    }
                    None => return Ok(()),
                };

                git_configs.push(GitConfig::new(&setting.identifier, &value_to_use));
            }
            input_handler.clear_screen()?;
        }

        if let Ok(Some(input)) =
            input_handler.read_line("Would you like to set up git aliases? (Y/N): ")
        {
            if input.trim().to_uppercase() == "Y" {
                loop {
                    input_handler
                        .write_line("\nEnter alias details (or press Enter to finish):")?;

                    let alias = match input_handler.read_line("Alias (e.g., 'st' for status): ")? {
                        Some(a) if !a.trim().is_empty() => a.trim().to_string(),
                        _ => break,
                    };

                    let command = match input_handler.read_line("Command (e.g., 'status'): ")? {
                        Some(c) if !c.trim().is_empty() => c.trim().to_string(),
                        _ => break,
                    };

                    git_cmd_aliases.push(GitCmdAlias::new(&alias, &command));
                }
            }
        }

        let config = GitCmdConfig {
            scope: config_scope.clone(),
            git_configs,
            git_cmds: git_cmd_aliases,
        };

        loop {
            let input = input_handler
                .read_line("\nNote: this will update your git configuration. Proceed (Y/N)? ")?
                .unwrap_or_default();

            match input.trim().to_uppercase().as_str() {
                "Y" | "" => {
                    if let Some(err) = write_gitcmd_conf(&config) {
                        input_handler.write_line(&format!("Error writing config: {}", err))?;
                        return Ok(());
                    }

                    if let Some(err) = source_gitcmd_conf() {
                        input_handler.write_line(&format!("Error applying config: {}", err))?;
                        return Ok(());
                    }

                    input_handler.write_line("\nConfiguration successfully applied.")?;
                    break;
                }
                "N" => {
                    input_handler.write_line("\nConfiguration not applied. Aborted.")?;
                    break;
                }
                _ => {
                    input_handler.write_line("\nInvalid input. Please enter 'Y' or 'N'.")?;
                }
            }
        }

        Ok(())
    }
}

pub fn setup_git_conf_profile() {
    let home_dir = home_dir().expect("Unable to determine home directory");
    let config_path: PathBuf = home_dir.join(".config/gitcmd/gitcmd.conf");

    if config_path.exists() {
        let mut input_handler =
            InputHandler::new_raw().expect("FATAL: Failed to create input handler.");

        input_handler
            .clear_screen()
            .expect("FATAL: Failed to clear screen.");
        input_handler
            .write_str(
                "Found existing gitcmd configuration.\r\n\
            \r\n\
            (E)dit existing configuration\r\n\
            (S)tart fresh (clear and create new)\r\n\
            \r\n",
            )
            .expect("FATAL: Failed to write to stdout.");

        loop {
            let input = input_handler
                .read_line("Please enter 'E' or 'S': ")
                .expect("FATAL: Failed to read from stdin.");

            match input {
                Some(val) => {
                    let val = val.trim().to_uppercase();
                    if val == "E" {
                        let config = GitConfigSettings::new();
                        config
                            .display(true, "")
                            .expect("FATAL: Unable to write config to stdout.");
                        return;
                    } else if val == "S" {
                        let config = GitConfigSettings::new();
                        config
                            .display(false, "")
                            .expect("FATAL: Unable to write config to stdout.");
                        return;
                    } else {
                        input_handler
                            .write_str("Invalid input. Please enter 'E' or 'S'.\r\n")
                            .expect("FATAL: Failed to write to stdout.");
                    }
                }
                None => {
                    return;
                }
            }
        }
    }

    let config = GitConfigSettings::new();
    config
        .display(false, "")
        .expect("FATAL: Unable to write config to stdout.");
}

pub fn edit_git_conf_profile() {
    let config = GitConfigSettings::new();
    config
        .display(true, "")
        .expect("FATAL: Unable to edit config to stdout.");
}
