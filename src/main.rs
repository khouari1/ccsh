use std::env::{current_dir, set_current_dir};
use std::io;
use std::io::{stdout, Write};
use std::process::Command;

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
        let parts: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
        handle_command_with_args(parts);
    }
}

fn handle_command_with_args(parts: Vec<String>) {
    match parts.split_first() {
        None => {
            println!("Invalid command")
        }
        Some(splitAtFirst) => {
            let (command, args) = splitAtFirst;
            match command.as_str() {
                "cd" => {
                    cd(args);
                }
                "pwd" => {
                    println!("{}", current_dir().unwrap().to_str().unwrap());
                }
                _ => {
                    run_command(command, args);
                }
            }
        }
    }
}

fn cd(args: &[String]) {
    match args.first() {
        None => {
            println!("Invalid command")
        }
        Some(arg) => {
            let new_path = current_dir().unwrap().as_path().join(arg);
            if new_path.exists() {
                set_current_dir(new_path);
            } else {
                println!("Path does not exist: {:?}", new_path);
            }
        }
    }
}

fn run_command(command: &String, args: &[String]) {
    let child = Command::new(command)
        .args(args)
        .spawn();
    match child {
        Ok(child) => {
            child.wait_with_output();
        }
        Err(_) => {
            println!("Invalid command")
        }
    }
}
