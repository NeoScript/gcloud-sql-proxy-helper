use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let mut child = Command::new("/home/nasir/cloud-sql-proxy")
        .args(["badg-r:us-central1:staging", "--port", "9000"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start proxy");

    if let Some(stdout) = child.stdout.take() {
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(l) => println!("{}", l),
                    Err(e) => eprintln!("stdout read error: {}", e),
                }
            }
        });
    }

    child.wait_with_output().expect("should do something");
}
