use cliclack::confirm;
use cliclack::input;
use cliclack::select;
use config::{Config, ConfigError};
use serde::Deserialize;
use serde::Serialize;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use std::process::exit;

use owo_colors::OwoColorize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct ConnectionConfig {
    instance: String,
    port: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct StartProxConfig {
    proxy_exec_path: String,
    defaults: Vec<ConnectionConfig>,
}

#[derive(Debug)]
enum MyConfigError {
    FailedToCreateFile,
    ConfigFileNotFound,
    HomeDirectoryNotFound,
    ParseConfigError(ConfigError),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("load_config error: {:?}", e);
            exit(1);
        }
    };

    let selected_config = prompt_instance(config.defaults).expect("should receive user input");
    let port = prompt_port(&selected_config.port).expect("should receive user input");
    start_proxy(&config.proxy_exec_path, &selected_config.instance, &port);
    Ok(())
}

fn load_config() -> Result<StartProxConfig, MyConfigError> {
    // Determine home path

    let home: String = match std::env::var("HOME") {
        Ok(home_path) => home_path,
        Err(std::env::VarError::NotPresent) => {
            eprintln!("{}", "Could not find HOME env var".red());
            exit(1);
        }
        Err(_) => {
            eprintln!("{}", "Failed to load $HOME".red());
            exit(1);
        }
    };

    // Now check for config file
    let config_path = format!("{home}/.config/startprox/config.yml");
    let config = Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build();

    let settings: Config = match config {
        Ok(c) => c,
        Err(ConfigError::Foreign(e)) => {
            if let Some(io_error) = e.downcast_ref::<std::io::Error>() {
                if io_error.kind() == std::io::ErrorKind::NotFound {
                    eprintln!(
                        "{}",
                        format!("Could not find config file: {}", config_path).red()
                    );
                    prompt_create_default_config(&config_path);
                }
            }

            eprintln!("{}", format!("io_error has occured: {}", e).red());
            exit(1)
        }
        Err(e) => {
            eprintln!("Config Error: {}", e);
            exit(1);
        }
    };

    match settings.try_deserialize::<StartProxConfig>() {
        Ok(settings) => {
            println!(
                "{}",
                format!("Successfully loaded config from {}", config_path).green()
            );
            Ok(settings)
        }
        Err(e) => Err(MyConfigError::ParseConfigError(e)),
    }
}

fn prompt_create_default_config(path: &str) {
    if confirm("Would you like to create a default config?")
        .initial_value(true)
        .interact()
        .unwrap()
    {
        create_config_file(path);
    }
}

fn create_config_file(path: &str) {
    let config_path = std::path::Path::new(path);
    let parent = config_path.parent().unwrap();

    std::fs::create_dir_all(parent).unwrap();
    std::fs::write(
        config_path,
        "
proxy_exec_path: cloud-sql-proxy # put path to cloud-sql-proxy here

defaults:
 - instance: project_id:region_id:instance_id
   port: 5432 # you can set a default port to connect to
",
    )
    .unwrap();

    println!(
        "{}",
        format!("Created default config file at: {path}").green()
    );
    exit(1);
}

fn prompt_instance(default_options: Vec<ConnectionConfig>) -> std::io::Result<ConnectionConfig> {
    let options: Vec<(ConnectionConfig, &str, &str)> = default_options
        .iter()
        .map(|c| (c.clone(), c.instance.as_str(), ""))
        .collect();

    select("Select instance ↑/↓: (type to filter)")
        .items(&options)
        .filter_mode()
        .interact()
}

fn prompt_port(default_port: &str) -> std::io::Result<String> {
    input("Which port would you like to connect on?")
        .default_input(default_port)
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Port cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact()
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
                    Err(e) => eprintln!("error reading stdout: {}", e.red()),
                }
            }
        });
    };

    if let Some(stderr) = child.stderr.take() {
        std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                match line {
                    Ok(l) => eprintln!("stderr: {}", l.yellow()),
                    Err(e) => eprintln!("error reading stderr: {}", e.red()),
                }
            }
        });
    };

    child.wait_with_output().expect("proxy should have run");
}

// Things that I want to support
// check for settings file
// if it exists then we should load them in
// if not then lets create a config file in a default .config place
//
