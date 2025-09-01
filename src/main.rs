use std::{
    env,
    error::Error,
    fs,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use clap::Parser;
use commands::{Cli, Commands};
use mouse::{MouseButton, MouseDevice};
use nix::{
    sys::signal::{Signal, kill},
    unistd::Pid,
};
use profiles::{Config, Mode};

mod commands;
mod mouse;
mod profiles;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Cli::parse();

    match arguments.command {
        Commands::Init => init_config(),
        Commands::List => {
            let config = Config::load()?;

            println!("Profiles list");

            for profile in config.profile.iter() {
                println!();
                println!("{}", profile.name);
                println!(
                    "button: {}",
                    if profile.button == MouseButton::Left {
                        "Left"
                    } else {
                        "Right"
                    }
                );
                match profile.mode {
                    Mode::Click => {
                        println!("mode: Click");
                        println!("interval: {}", profile.interval.unwrap());
                        println!("repeat: {}", profile.repeat.unwrap());
                    }
                    Mode::Hold => println!("mode: Hold"),
                }
            }

            Ok(())
        }
        Commands::Start { name, __run } => {
            let config = Config::load()?;

            if let Some(profile) = config.profile.iter().find(|e| e.name == name).cloned() {
                if __run {
                    println!("Running profile: {}", profile.name);

                    let mut mouse = MouseDevice::new()?;

                    match profile.mode {
                        Mode::Click => loop {
                            for _ in 0..=profile.repeat.unwrap() {
                                let _ = mouse.click(&profile.button);
                            }

                            sleep(Duration::from_millis(profile.interval.unwrap()));
                        },
                        Mode::Hold => {
                            ctrlc::set_handler(move || {
                                match MouseDevice::new() {
                                    Ok(mut mouse) => {
                                        let _ = mouse.release(&profile.button);
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to create MouseDevice: {}", e);
                                    }
                                }
                                std::process::exit(0);
                            })?;

                            let _ = mouse.hold(&profile.button);

                            loop {
                                sleep(Duration::from_secs(1)); // keep process alive
                            }
                        }
                    }
                } else {
                    spawn(&profile.name)?;
                }
            } else {
                eprintln!("profile not found: {}", name);
            }

            Ok(())
        }
        Commands::Stop { name } => {
            stop(&name)?;
            Ok(())
        }
    }
}

fn init_config() -> Result<(), Box<dyn Error>> {
    let config_path = Config::path()?;
    let config_file = config_path.join("profiles.toml");

    if config_file.exists() {
        println!("Config already exists at: {}", config_file.display());
        return Ok(());
    }

    let template = r#"
[[profile]]
name = "click"
interval = 1
button = "left"
repeat = 1
mode = "click"
"#;

    fs::write(&config_file, template)?;

    println!("Config file created at: {}", config_file.display());

    Ok(())
}

fn spawn(profile: &str) -> Result<(), Box<dyn Error>> {
    let config_path = Config::path()?;
    let pid_path = config_path.join(".pids");
    fs::create_dir_all(&pid_path)?;

    let pid_file = pid_path.join(format!("{}.pid", profile));

    if pid_file.exists() {
        eprintln!("autoclicker: {}, is already running", profile);
        return Ok(());
    }

    let command = Command::new(env::current_exe()?)
        .arg("start")
        .arg(&profile)
        .arg("--run")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    fs::write(&pid_file, command.id().to_string())?;
    println!("started autoclicker for: {}", profile);

    Ok(())
}

fn stop(profile: &str) -> Result<(), Box<dyn Error>> {
    let pid_file = Config::path()?
        .join(".pids")
        .join(format!("{}.pid", profile));

    if !pid_file.exists() {
        println!("no autoclicker running for: {}", profile);
        return Ok(());
    }

    let pid = fs::read_to_string(&pid_file)?.trim().parse::<u32>()?;

    let result = kill(Pid::from_raw(pid as i32), Signal::SIGTERM);
    match result {
        Ok(_) => {}
        Err(nix::Error::ESRCH) => {
            println!("no running process found for: {}", profile);
        }
        Err(e) => return Err(Box::new(e)),
    }

    fs::remove_file(pid_file)?;

    println!("stopped process: {}", profile);

    Ok(())
}
