use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;

use owo_colors::OwoColorize;

fn main() {
    let path = "/Users/nasir/cloud-sql-proxy";
    let instance = "badg-r:us-central1:staging";
    let port = "9000";
    start_proxy(path, instance, port);
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
