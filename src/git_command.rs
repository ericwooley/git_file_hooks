use crate::parse;
use glob;
use std::env;
use std::path::PathBuf;
use std::process;
use std::process::Command;
fn get_argv() -> Vec<String> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 3 {
        panic!("arg1 and arg2 should be the commits that changed.")
    }
    argv
}
pub fn get_hook() -> String {
    let argv = get_argv();
    let cmd = &argv[0];
    if argv.len() > 3 {
        return argv[3].clone();
    }
    arg0_to_hook(cmd)
}
fn arg0_to_hook(cmd: &String) -> String {
    let cmd = PathBuf::from(cmd);

    cmd.file_name()
        .expect("Unable to determine hook")
        .to_string_lossy()
        .to_string()
}

pub fn get_changed_files() -> Vec<String> {
    let argv = get_argv();
    let git_hash_1 = &argv[1].clone();
    let git_hash_2 = &argv[2].clone();
    run_git_diff_files(git_hash_1, git_hash_2)
}

fn run_git_diff_files(sha1: &String, sha2: &String) -> Vec<String> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(sha1)
        .arg(sha2)
        .output()
        .expect(&format!(
            "Failed to get the files diff from {:?} to {:?}",
            sha1, sha2,
        ));
    assert!(output.status.success(), "git diff was unsuccessful");
    let stdout = String::from_utf8_lossy(&output.stdout).clone();
    stdout.split("\n").map(|s| s.to_string()).collect()
}

pub fn filter_commands_by_files(
    commands: &Vec<parse::Command>,
    files: &Vec<String>,
) -> Vec<String> {
    commands
        .iter()
        .filter(|command| {
            // O(n^2), i know, but these are hand written files, it's not going to matter
            for file in files {
                let patterns = &command.patterns;
                for pattern in patterns {
                    let pattern = glob::Pattern::new(pattern).unwrap_or_else(|err| {
                        println!("Malformed pattern: {}", pattern);
                        process::exit(0)
                    });
                    if pattern.matches(file) {
                        return true;
                    }
                }
            }
            return false;
        })
        .flat_map(|command| command.commands.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_get_hook() {
        assert_eq!(
            arg0_to_hook(&String::from(
                "/users/home/wherever/.git/hooks/post-checkout"
            )),
            "post-checkout"
        );
    }
    #[test]
    fn it_should_get_the_changed_files() {
        assert_eq!(
            run_git_diff_files(
                &String::from("19b2ea5c076971433d3a8e13a3f602eaf939380e"),
                &String::from("397dfdfcb846076d0423f9ab5ce3bae80133b551"),
            ),
            [
                ".file_hooks.yml",
                "Cargo.lock",
                "Cargo.toml",
                "files_for_testing/test-1.test",
                "files_for_testing/test-2.bin",
                "files_for_testing/test-3.txt",
                "files_for_testing/test4",
                "src/command.rs",
                "src/config.rs",
                "src/main.rs",
                "src/parse.rs",
                "tests/parse_test.rs",
                ""
            ]
        );
    }
    #[test]
    #[should_panic]
    fn it_should_panic_without_valid_hashes() {
        run_git_diff_files(&String::from("nonsense"), &String::from("Nonsense"));
    }
    #[test]
    fn it_should_filter_files_by_pattern() {
        let non_matching_command = parse::Command {
            patterns: vec![String::from("**/*.js")],
            commands: vec![String::from("should never be executed")],
        };
        let echo_command = String::from("echo \"Hello\"");
        let command = parse::Command {
            patterns: vec![String::from("**/*.rs")],
            commands: vec![echo_command.clone()],
        };
        let commands = vec![non_matching_command, command];
        let files = vec![
            String::from(".file_hooks.yml"),
            String::from("Cargo.lock"),
            String::from("Cargo.toml"),
            String::from("files_for_testing/test-1.test"),
            String::from("files_for_testing/test-2.bin"),
            String::from("files_for_testing/test-3.txt"),
            String::from("files_for_testing/test4"),
            String::from("src/command.rs"),
            String::from("src/config.rs"),
            String::from("src/main.rs"),
            String::from("src/parse.rs"),
            String::from("tests/parse_test.rs"),
            String::from(""),
        ];
        let commands = filter_commands_by_files(&commands, &files);
        println!("Filtered files: {:?}", files);
        assert_eq!(
            commands.len(),
            1,
            "non matching command should be filtered out"
        );
        assert_eq!(
            commands.get(0).expect("Expected first command to be rust"),
            &echo_command,
            "rs files exist, so the command should execute"
        )
    }
}
