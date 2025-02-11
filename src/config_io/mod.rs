// FILE LAYOUT
// gitcmd.conf
// scope {
//      global
// }
//
// gitconfig {
//      user.email=corniedj@mail.uc.edu
//      user.name=Derek Corniello
//      init.defaultbranch=main
//      core.editor=nvim
// }
//
// gitcmd {
//      st=status
//      a=add -A
//      gac=add -A && commit
// }

use crate::input_handler::InputHandler;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

const CONFIG_FILE: &str = "~/.config/gitcmd/gitcmd.conf";

#[derive(Debug, Clone)]
pub struct GitCmdAlias {
    pub identifier: String,
    pub command: String,
}

impl GitCmdAlias {
    pub fn new(id: &str, comm: &str) -> Self {
        GitCmdAlias {
            identifier: id.to_string(),
            command: comm.to_string(),
        }
    }

    pub fn to_entry(&self) -> String {
        format!("{}={}", self.identifier, self.command)
    }

    pub fn from_entry(entry: &str) -> Option<Self> {
        let parts: Vec<&str> = entry.splitn(2, '=').collect();
        if parts.len() == 2 {
            Some(GitCmdAlias::new(parts[0].trim(), parts[1].trim()))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct GitConfig {
    pub identifier: String,
    pub value: String,
}

impl GitConfig {
    pub fn new(id: &str, val: &str) -> Self {
        GitConfig {
            identifier: id.to_string(),
            value: val.to_string(),
        }
    }

    pub fn to_entry(&self) -> String {
        format!("{}={}", self.identifier, self.value)
    }

    pub fn to_command(&self, scope: &str) -> String {
        format!(
            "git config --{} {} \"{}\"",
            scope, self.identifier, self.value
        )
    }

    pub fn from_entry(entry: &str) -> Option<Self> {
        let parts: Vec<&str> = entry.splitn(2, '=').collect();
        if parts.len() == 2 {
            Some(GitConfig::new(parts[0].trim(), parts[1].trim()))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct GitCmdConfig {
    pub scope: String,
    pub git_configs: Vec<GitConfig>,
    pub git_cmds: Vec<GitCmdAlias>,
}

impl GitCmdConfig {
    fn new() -> Self {
        GitCmdConfig {
            scope: String::new(),
            git_configs: Vec::new(),
            git_cmds: Vec::new(),
        }
    }

    fn parse_line(&mut self, line: &str) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            return;
        }

        if !line.contains('{') && !line.contains('}') && !line.contains('=') {
            // Must be a scope value
            self.scope = line.trim().to_string();
            return;
        }

        if line.contains('=') {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                if key.contains('.') {
                    if let Some(config) = GitConfig::from_entry(line) {
                        self.git_configs.push(config);
                    }
                } else if let Some(cmd) = GitCmdAlias::from_entry(line) {
                    self.git_cmds.push(cmd);
                }
            }
        }
    }
}

pub fn source_command(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("FATAL: Failed to write terminal command.");

    let mut input_handler =
        InputHandler::new_raw().expect("FATAL: Failed to create input handler.");
    if !output.status.success() {
        input_handler
            .write_line(&format!(
                "Failed to execute command to get configs: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
            .expect("FATAL: Failed to write to stdout.");
    }
}

pub fn import_git_config(scopes: Vec<String>) -> Result<Vec<GitConfig>, std::io::Error> {
    if scopes.is_empty() {
        return Ok(Vec::new());
    }
    let mut configs: Vec<GitConfig> = Vec::new();
    let mut input_handler = InputHandler::new_raw()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    loop {
        let input = input_handler
            .read_line("Do you want to import your current git config settings (Y/N)? ")?
            .unwrap_or_default();
        let proceed = if input.trim().is_empty() || input.trim().to_uppercase() == "Y" {
            Some(true)
        } else if input.trim().to_uppercase() == "N" {
            Some(false)
        } else {
            None
        };

        match proceed {
            Some(true) => {
                for scope in &scopes {
                    let cmd = format!("git config --list --{}", scope);
                    let output = Command::new("sh").arg("-c").arg(cmd).output()?;

                    if !output.status.success() {
                        input_handler.write_line(&format!(
                            "Failed to execute command to get configs: {}",
                            String::from_utf8_lossy(&output.stderr)
                        ))?;
                    } else {
                        let config_output = String::from_utf8_lossy(&output.stdout);
                        for line in config_output.lines() {
                            if let Some(config) = GitConfig::from_entry(line) {
                                configs.push(config);
                            }
                        }
                    }
                }
                return Ok(configs);
            }
            Some(false) => {
                return Ok(Vec::new());
            }
            None => {
                input_handler
                    .write_line("\nInvalid Input, please input 'Y' or 'N'.")
                    .expect("FATAL: Failed to write to stdout.");
            }
        }
    }
}

pub fn read_gitcmd_conf() -> Result<GitCmdConfig, io::Error> {
    let file = File::open(CONFIG_FILE)?;
    let reader = BufReader::new(file);
    let mut config = GitCmdConfig::new();

    for line in reader.lines() {
        config.parse_line(&line?);
    }

    Ok(config)
}

pub fn write_gitcmd_conf(config: &GitCmdConfig) -> Option<io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(CONFIG_FILE);

    match file {
        Ok(mut file) => {
            let mut content = String::new();

            // Write scope section
            content.push_str("scope {\n");
            content.push_str(&format!("\t{}\n", config.scope));
            content.push_str("}\n\n");

            // Write gitconfig section
            content.push_str("gitconfig {\n");
            for git_config in &config.git_configs {
                content.push_str(&format!("\t{}\n", git_config.to_entry()));
            }
            content.push_str("}\n\n");

            // Write gitcmd section
            content.push_str("gitcmd {\n");
            for cmd in &config.git_cmds {
                content.push_str(&format!("\t{}\n", cmd.to_entry()));
            }
            content.push_str("}\n");

            if let Err(e) = file.write_all(content.as_bytes()) {
                return Some(e);
            }
        }
        Err(e) => return Some(e),
    }

    None
}

pub fn source_gitcmd_conf() -> Option<io::Error> {
    match read_gitcmd_conf() {
        Ok(config) => {
            // Apply git configurations
            for git_config in &config.git_configs {
                let command = git_config.to_command(&config.scope);
                source_command(&command);
            }

            None
        }
        Err(e) => Some(e),
    }
}
