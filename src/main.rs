#[macro_use]
extern crate serde_derive;
extern crate glob;
extern crate serde;
extern crate serde_yaml;
use std::process;
mod command;
mod config;
mod parse;

fn main() {
    let conf = config::from_file();
    let hook = command::get_hook();
    println!("Running hook: {}", hook);
    let commands = match conf.get_commands(&hook) {
        Some(c) => c,
        None => {
            println!("No commands for: {}", hook);
            process::exit(0);
        }
    };
    let files = command::get_changed_files();
    // command::run_commands(hook, files)
}
