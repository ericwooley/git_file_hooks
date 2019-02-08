use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::path::PathBuf;
use std::process;
#[derive(Deserialize, Debug)]
struct Command {
    pattern: Vec<String>,
    commands: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Config {
    commands: Vec<Command>,
}

pub fn resolve_config_file() -> String {
    let mut config_file = env::current_dir().expect("Unable to resolve cwd");
    println!("Starting in: {:?}", config_file);
    let mut file = resolve_file_up_tree(&mut config_file);
    let mut r = String::new();
    file.read_to_string(&mut r)
        .expect("Could not read file stream");
    println!("Found config {}", r);
    r
}

fn append_file_hooks(path: &mut PathBuf) -> PathBuf {
    path.join(".file_hooks.yml")
}

fn resolve_file_up_tree(working_dir: &mut PathBuf) -> File {
    let config_path = append_file_hooks(working_dir);
    println!("Checking: {:?}", config_path);
    match File::open(config_path) {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let mut parent = working_dir.parent().unwrap().to_path_buf();
                let mut parent = append_file_hooks(&mut parent);
                if parent.to_string_lossy() == working_dir.to_string_lossy() {
                    println!("Could not resolve .file_hooks.yml");
                    process::exit(1);
                }
                resolve_file_up_tree(&mut parent)
            }
            _ => panic!("Unkown error {}", error),
        },
    }
}
