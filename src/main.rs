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
        let child = Command::new(input)
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
}
