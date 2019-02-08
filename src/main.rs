#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
mod config;
fn main() {
    config::resolve_config_file();
}
