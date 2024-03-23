use std::env::{current_dir, set_current_dir};
use std::io;
use std::io::{stdout, Write};
use std::process::{Command, Stdio};
use std::str;

fn main() {
    loop {
        print!("ccsh> ");
        stdout().flush();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let input = buffer.trim();
        if input == "exit" {
            break
        }
        let pipe_parts: Vec<String> = input.split("|").map(|s| s.trim().to_string()).collect();
        let mut addition: String = String::new();
        for (index, pipe_part) in pipe_parts.iter().enumerate() {
            let parts: Vec<String> = pipe_part.split(" ").map(|s| s.to_string()).collect();
            let result = handle_command_with_args(parts, addition.to_string(), index == pipe_parts.len() - 1);
            match result {
                Ok(stdout) => {
                    let r = str::from_utf8(&stdout);
                    addition = r.unwrap().to_string();
                }
                Err(err) => {
                    println!("Something went wrong: {}", err)
                }
            }
        }
    }
}

fn handle_command_with_args(parts: Vec<String>, prior_output: String, is_last: bool) -> Result<Vec<u8>, String> {
    match parts.split_first() {
        None => {
            Err("Invalid command".to_string())
        }
        Some(splitAtFirst) => {
            let (command, args) = splitAtFirst;
            match command.as_str() {
                "cd" => {
                    cd(args)
                }
                "pwd" => {
                    println!("{}", current_dir().unwrap().to_str().unwrap());
                    Ok(Vec::new())
                }
                _ => {
                    run_command(command, args, prior_output, is_last)
                }
            }
        }
    }
}

fn cd(args: &[String]) -> Result<Vec<u8>, String> {
    match args.first() {
        None => {
            Err("Invalid command".to_string())
        }
        Some(arg) => {
            let new_path = current_dir().unwrap().as_path().join(arg);
            if new_path.exists() {
                set_current_dir(new_path);
                Ok(Vec::new())
            } else {
                Err(format!("Path does not exist {:?}", new_path))
            }
        }
    }
}

fn run_command(command: &String, args: &[String], prior_input: String, is_last: bool) -> Result<Vec<u8>, String> {
    // TODO: clean up duplication
    if is_last {
        let child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .spawn();
        let mut child2 = child.unwrap();
        if !prior_input.is_empty() {
            let stdin = child2.stdin.as_mut().unwrap();
            stdin.write_all(prior_input.as_bytes());
        }
        let output = child2.wait_with_output();
        match output {
            Ok(_) => Ok(Vec::new()),
            Err(_) => Err("Command didn't complete successfully".to_string())
        }
    } else {
        let child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();
        let mut child2 = child.unwrap();
        if !prior_input.is_empty() {
            let stdin = child2.stdin.as_mut().unwrap();
            stdin.write_all(prior_input.as_bytes());
        }
        let output = child2.wait_with_output();
        match output {
            Ok(output) => Ok(output.stdout),
            Err(_) => Err("Command didn't complete successfully".to_string())
        }
    }
}
