use demand::Input;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::process::Command;
use std::process::Stdio;

use owo_colors::OwoColorize;

fn main() {
    let path = "/Users/nasir/cloud-sql-proxy";
    let port = "9000";

    let instance = prompt_connection().expect("Failed to prompt for instance");
    start_proxy(path, &instance, port);
}

fn prompt_connection() -> Result<String> {
    let instance_str_validator = |s: &str| {
        if s.is_empty() {
            return Err("instance name can not be empty");
        }

        Ok(())
    };

    let prompt = Input::new("Which gcloud sql instance would you like to connect to?")
        .prompt("Instance: ")
        .placeholder("project:location:instance")
        .validation(instance_str_validator);

    prompt.run()
}

fn start_proxy(path: &str, instance: &str, port: &str) {
    let mut child = Command::new(path)
        .args([instance, "--port", port])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("cloud-sql-proxy should have started:");

    if let Some(stdout) = child.stdout.take() {
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(l) => println!("info: {}", l.green()),
                    Err(e) => eprintln!("error: {}", e.red()),
                }
            }
        });
    }

    child.wait_with_output().expect("proxy should have run");
}

// Things that I want to support
// check for settings file
// if it exists then we should load them in
// if not then lets create a config file in a default .config place
//
