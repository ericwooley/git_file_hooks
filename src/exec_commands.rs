use run_script::ScriptOptions;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
pub fn exec_commands(commands: Vec<Vec<String>>) {
    for command_group in commands {
        let finished_command_count = Arc::new(Mutex::new(0));
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        let len = command_group.len();
        println!("");
        println!("🚀 Executing Command Group: {:?}", command_group);
        print_separator();
        for cmd in command_group {
            let finished_command_count = Arc::clone(&finished_command_count);
            handles.push(thread::spawn(move || {
                println!("🤖 Executing: {}", cmd);
                let mut options = ScriptOptions::new();
                options.runner = None; // The script runner, for example bash. By default for windows it's cmd.exe and for other systems it is sh.
                options.capture_output = true; // True to capture and return the output. False will print it to the parent process output.
                options.exit_on_error = false; // Adds set -e option (not available for windows)
                options.print_commands = false; // Adds set -x option (not available for windows)

                let args = vec![];

                let (code, output, error) = run_script::run(&cmd, &args, &options).unwrap();
                let mut finished_command_count = finished_command_count.lock().unwrap();
                *finished_command_count += 1;

                println!("Command: {}", cmd);
                if output.trim().len() > 0 {
                    println!("stdout: {}", output.trim());
                }
                if error.len() > 0 {
                    eprintln!("stderr: {}", error);
                }
                println!("👷 jobs done: {}/{}", finished_command_count, len);
                print_separator();
                if code != 0 {
                    eprintln!("⚠️  ");
                    eprintln!("⚠️  Error: `{}` exited with code {} 😟", cmd, code);
                    eprintln!("⚠️  ");
                    process::exit(code);
                }
            }));
        }
        for handle in handles {
            handle.join().expect("Error rejoining threads");
        }
    }
    println!("💪 Jobs Done! 🙌")
}

fn print_separator() {
    println!("");
    println!("----------------------------------------------------------------------------------------------");
}

#[cfg(test)]
mod test {
    use super::*;

    // this test really just makes sure there are no panics. Hard to test this as it's basically
    // a giant side affect
    #[test]
    fn it_should_parse_commands() {
        exec_commands(vec![vec![String::from("echo \"hello world\" ")]]);
    }
}
