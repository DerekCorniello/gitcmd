![](https://img.shields.io/badge/Rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
# gitcmd - Git Configuration Manager

gitcmd is a terminal-based tool designed to simplify the management of Git configurations and settings. It provides an interactive shell for executing Git commands and features a configuration setup wizard to customize your Git experience.

gitcmd is made for people of all experience levels! Whether you need help setting up your config, want some shortcuts, or need a terminal to help speed up your development, everyone can use it!

## Features

- **Interactive Terminal**: A custom shell (`git >`) for running Git commands easily.
- **Configuration Wizard**: Quickly set up and manage your Git configurations.
- **User-Friendly Setup**: Automatically creates a configuration file at `~/.config/gitcmd/config` to store settings.
- **Command Line Support**: Includes commands for setup, help, and launching the interactive terminal.
- **Extendable Design**: Built with modularity in mind, making it easy to add new features and commands.

## Installation

### Prerequisites

1. [cargo](https://github.com/rust-lang/cargo)
2. [git](https://git-scm.com/)

### Steps

#### Using Git

1. Clone the repository:
   ```
   git clone https://github.com/DerekCorniello/gitcmd.git
   ```
2. Navigate to the project directory:
   ```
   cd gitcmd
   ```
3. Build the project using Cargo:
   ```
   cargo build --release
   ```
4. Add the executable to your `PATH`:
   ```
   export PATH=$PATH:/path/to/gitcmd/target/release
   ```
   Replace `/path/to/gitcmd` with the actual path to the project.

#### Using Cargo

Simply do:
    ```
    cargo install gitcmd
    ```

## Usage

Run `gitcmd` from your terminal:

### Interactive Mode
```
gitcmd
```
Starts the gitcmd terminal with a prompt (`git >`). Enter Git commands without the `git` prefix (e.g., `add -A` instead of `git add -A`).

### Commands
- **`setup`**: Launches the configuration wizard to create or update your gitcmd profile.
  ```
  gitcmd setup
  ```
- **`help`**: Displays usage instructions.
  ```
  gitcmd help
  ```

### Example Workflow
1. Start the terminal:
   ```
   gitcmd
   ```
2. Use commands like:
   ```
   git > status
   git > commit -m "Initial commit"
   ```

## Configuration File

By default, gitcmd creates a configuration file at `~/.config/gitcmd/config`. This file is used to store settings for the tool. If the file already exists, the tool will not overwrite it unless explicitly instructed during setup.

## Contribution

Contributions are welcome! Feel free to fork the repository and submit pull requests to enhance gitcmd.

## License

This project is licensed under the Apache2 License. See the LICENSE file for details.


## Connect with Me!
[![LinkedIn](https://img.shields.io/badge/LinkedIn-%230A66C2.svg?style=for-the-badge&logo=linkedin&logoColor=white)](https://www.linkedin.com/in/derek-corniello)
[![GitHub](https://img.shields.io/badge/GitHub-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/derekcorniello)
[![X](https://img.shields.io/badge/X-%231DA1F2.svg?style=for-the-badge&logo=x&logoColor=white)](https://x.com/derekcorniello)
