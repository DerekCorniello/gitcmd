use std::io::{self, Write};

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
                    command: String::from("git config --global user.name '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Git User Email"),
                    example: String::from("youremail@example.com"),
                    possibles: vec![String::from("youremail@example.com")],
                    command: String::from("git config --global user.email '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Git Editor"),
                    example: String::from("vim"),
                    possibles: vec![String::from("vim"), String::from("nano"), String::from("code")],
                    command: String::from("git config --global core.editor '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Core Auto CRLF"),
                    example: String::from("false"),
                    possibles: vec![String::from("true"), String::from("false"), String::from("input")],
                    command: String::from("git config --global core.autocrlf '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Core Ignore Case"),
                    example: String::from("true"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config --global core.ignorecase '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Core File Mode"),
                    example: String::from("true"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config --global core.filemode '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Merge Tool"),
                    example: String::from("vimdiff"),
                    possibles: vec![String::from("vimdiff"), String::from("meld"), String::from("kdiff3")],
                    command: String::from("git config --global merge.tool '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Diff Tool"),
                    example: String::from("vimdiff"),
                    possibles: vec![String::from("vimdiff"), String::from("meld"), String::from("kdiff3")],
                    command: String::from("git config --global diff.tool '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Commit GPG Sign"),
                    example: String::from("false"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config --global commit.gpgSign '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Init Default Branch"),
                    example: String::from("main"),
                    possibles: vec![String::from("main"), String::from("master")],
                    command: String::from("git config --global init.defaultBranch '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Push Default"),
                    example: String::from("simple"),
                    possibles: vec![String::from("simple"), String::from("current"), String::from("matching")],
                    command: String::from("git config --global push.default '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Fetch Prune"),
                    example: String::from("false"),
                    possibles: vec![String::from("true"), String::from("false")],
                    command: String::from("git config --global fetch.prune '{}'"),
                },
                GitConfigEntry {
                    title: String::from("Core Pager"),
                    example: String::from("less"),
                    possibles: vec![String::from("less"), String::from("more")],
                    command: String::from("git config --global core.pager '{}'"),
                },
            ]
        }
    }

    pub fn display(&self) {
        for setting in &self.settings {
            // Display the setting title
            println!("\rSetting: {}", setting.title);
            // Show example value
            println!("\rExample: {}", setting.example);
            // Show possible values
            println!("\rPossible values: {:?}", setting.possibles);
            // Prompt user for input (can extend this to handle input)
            println!("\rPlease choose a value (or press Enter to use default '{}'): ", setting.example);
            io::stdout().flush().unwrap(); // Make sure the prompt is displayed

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            // If user entered something, use it; otherwise, use the default value
            let value_to_use = if input.is_empty() {
                setting.example.clone()
            } else {
                input.to_string()
            };

            // Run the Git command with the user input
            let command = setting.command.replace("{}", &value_to_use);
            println!("\rRunning: {}", command);
            // Here you can run the actual Git command (uncomment the line below to do so)
            // std::process::Command::new("sh").arg("-c").arg(command).output().unwrap();
            println!();
        }
    }
}

pub fn setup_git_conf_profile() {
    let config = GitConfig::new();
    config.display();
}
