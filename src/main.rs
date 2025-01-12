use std::env;
use std::process;

mod config;
mod config_io;
mod input_handler;
mod input_parser;
mod terminal;

fn print_usage() {
    println!("GitCmd - Git Configuration Manager");
    println!("\nUsage:");
    println!("  gitcmd");
    println!("      Starts the gitcmd terminal");
    println!("  gitcmd <command>");
    println!("      setup     Run the configuration wizard");
    println!("      help      Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        terminal::terminal_loop().unwrap();
    } else {
        match args[1].as_str() {
            "setup" => {
                config::setup_git_conf_profile();
            }
            "help" => {
                print_usage();
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                print_usage();
                process::exit(1);
            }
        }
    }

    process::exit(0)
}
