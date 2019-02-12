#[macro_use]
extern crate serde_derive;
extern crate glob;
extern crate run_script;
extern crate serde;
extern crate serde_yaml;
use std::process;
mod config;
mod exec_commands;
mod git_command;
mod parse;

fn main() {
    config::exit_if_env_is_set();
    let conf = config::from_file();
    let hook = git_command::get_hook();
    println!("Running hook: {}", hook);
    let commands = match conf.hooks.get(&hook) {
        Some(c) => c,
        None => {
            println!("No commands for: {}", hook);
            process::exit(0);
        }
    };
    let files = git_command::get_changed_files();
    if files.len() == 0 {
        println!("No files too run for hook {}", hook);
        process::exit(0);
    }
    let commands = git_command::filter_commands_by_files(&commands, &files);

    exec_commands::exec_commands(commands);
}
