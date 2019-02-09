use run_script::ScriptOptions;
use spinner::SpinnerBuilder;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
pub fn exec_commands(commands: Vec<Vec<String>>) {
    for command_group in commands {
        let sp = Arc::new(Mutex::new(
            SpinnerBuilder::new("spawning commands".into()).start(),
        ));
        let finished_command_count = Arc::new(Mutex::new(0));
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        let len = command_group.len();
        for cmd in command_group {
            let sp = Arc::clone(&sp);
            let finished_command_count = Arc::clone(&finished_command_count);
            handles.push(thread::spawn(move || {
                let mut options = ScriptOptions::new();
                options.runner = None; // The script runner, for example bash. By default for windows it's cmd.exe and for other systems it is sh.
                options.capture_output = true; // True to capture and return the output. False will print it to the parent process output.
                options.exit_on_error = false; // Adds set -e option (not available for windows)
                options.print_commands = false; // Adds set -x option (not available for windows)

                let args = vec![];

                let (code, output, error) = run_script::run(&cmd, &args, &options).unwrap();
                let mut finished_command_count = finished_command_count.lock().unwrap();
                *finished_command_count += 1;
                // println!("\n----------------------------------------------------------------------------------------------");
                println!("\nCommand: {}", cmd);
                println!("stdout: {}", output);
                if error.len() > 0 {
                eprintln!("stderr: {}", error);}
                if(code != 0) {
                    eprint!("Hook Error: `{}` exited with code {}", cmd, code);
                    process::exit(code);
                }
                println!("\n----------------------------------------------------------------------------------------------\n");
                let sp = sp.lock().unwrap();
                sp.update(format!("{}/{}: finished `{}`", finished_command_count, len, cmd).into());
            }));
        }
        for handle in handles {
            handle.join();
        }
    }
    println!("Jobs Done!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_parse_commands() {
        exec_commands(vec![vec![String::from("echo \"hello world\" ")]]);
    }
}
