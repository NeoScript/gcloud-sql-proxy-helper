use config::Config;
use demand::Input;
use serde::Deserialize;
use serde::Serialize;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::process::Command;
use std::process::Stdio;

use owo_colors::OwoColorize;

#[derive(Serialize, Deserialize, Debug)]
struct ConnectionConfig {
    instance: String,
    port: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct StartProxConfig {
    proxy_exec_path: String,
    defaults: Option<Vec<ConnectionConfig>>,
}

fn main() {
    let config_path = "config.yml";

    let settings = Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
        .unwrap()
        .try_deserialize::<StartProxConfig>()
        .unwrap();

    println!("proxy exec path: {:?}", settings);

    let instance = prompt_instance(settings.defaults.unwrap_or_default())
        .expect("Failed prompt for sql connection name");
    let port = prompt_port().expect("Failed prompt for port number");
    start_proxy(&settings.proxy_exec_path, &instance, &port);
}

fn prompt_instance(default_options: Vec<ConnectionConfig>) -> Result<String> {
    let instance_str_validator = |s: &str| {
        if s.is_empty() {
            return Err("instance name can not be empty");
        }

        Ok(())
    };

    let completions: Vec<&str> = default_options
        .iter()
        .map(|config| config.instance.as_str())
        .collect();

    let prompt = Input::new("Which gcloud sql instance would you like to connect to?")
        .prompt("Instance: ")
        .placeholder("project:location:instance")
        .suggestions(&completions)
        .validation(instance_str_validator);

    prompt.run()
}

fn prompt_port() -> Result<String> {
    let port_validator = |s: &str| {
        if s.is_empty() {
            return Err("port can not be empty!");
        }
        Ok(())
    };

    let suggested_ports = vec!["5432", "9000"];
    let prompt = Input::new("Which port would you like for the sql proxy to use?")
        .prompt("Port #: ")
        .placeholder("5432")
        .suggestions(&suggested_ports)
        .validation(port_validator);

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
