use std::env;
use std::path::PathBuf;
use std::process::Command;
pub fn get_hook() -> String {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[0];
    let cmd = PathBuf::from(cmd);
    if args.len() > 3 {
        return args[3].clone();
    }
    cmd.file_name()
        .expect("Unable to determine hook")
        .to_string_lossy()
        .to_string()
}

pub fn get_changed_files()
// -> Vec<String>
{
    let args: Vec<String> = env::args().collect();
    let git_hash_1 = &args[1].clone();
    let git_hash_2 = &args[2].clone();
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(git_hash_1)
        .arg(git_hash_2)
        .output()
        .expect(&format!(
            "Failed to get the files diff from {:?} to {:?}",
            git_hash_1, git_hash_2,
        ));
    println!("files: {:?}", output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_get_hook() {
        assert!(
            get_hook().len() > 1,
            "A command should be extracted from the args"
        );
    }
}
