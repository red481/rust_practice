use std::io::{ self, Write };
use std::process::Command;

fn main() {
    loop {
        print!("RestShell$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = &parts[1..];

        let output = Command::new(command)
            .args(args)
            .output()
            .expect("Failed to execute command");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
