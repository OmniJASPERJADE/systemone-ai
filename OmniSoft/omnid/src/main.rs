use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{collections::HashSet, fs, path::PathBuf, process::Stdio};
use tokio::{process::Command, task::JoinSet, time::{sleep, Duration}};

#[derive(Parser)]
#[command(name = "omnid", about = "System-One service supervisor")]
struct Cli {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    Validate { config: PathBuf },
    Run { config: PathBuf },
}

#[derive(Debug, Deserialize)]
struct Config {
    services: Vec<Service>,
}

#[derive(Debug, Clone, Deserialize)]
struct Service {
    name: String,
    command: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default = "enabled")]
    enabled: bool,
    #[serde(default)]
    restart: Restart,
    #[serde(default = "default_delay")]
    restart_delay_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
enum Restart {
    #[default]
    Never,
    Always,
    OnFailure,
}

fn enabled() -> bool { true }
fn default_delay() -> u64 { 1000 }

fn load(path: &PathBuf) -> Result<Config> {
    let input = fs::read_to_string(path)
        .with_context(|| format!("Cannot read {}", path.display()))?;
    Ok(toml::from_str(&input)?)
}

fn validate(config: &Config) -> Result<()> {
    let mut names = HashSet::new();

    for service in &config.services {
        if service.name.trim().is_empty() {
            bail!("Service name cannot be empty");
        }
        if !names.insert(&service.name) {
            bail!("Duplicate service: {}", service.name);
        }
        if !service.command.starts_with('/') {
            bail!("Service command must be an absolute path: {}", service.name);
        }
        if service.restart_delay_ms == 0 {
            bail!("Restart delay must be positive: {}", service.name);
        }
    }

    Ok(())
}

async fn supervise(service: Service) -> Result<()> {
    if !service.enabled {
        return Ok(());
    }

    loop {
        println!("omnid: starting {}", service.name);

        let mut child = Command::new(&service.command)
            .args(&service.args)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .with_context(|| format!("Cannot start {}", service.name))?;

        let status = child.wait().await?;
        let success = status.success();

        println!("omnid: {} exited: {}", service.name, status);

        let restart = match service.restart {
            Restart::Never => false,
            Restart::Always => true,
            Restart::OnFailure => !success,
        };

        if !restart {
            break;
        }

        sleep(Duration::from_millis(service.restart_delay_ms)).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Action::Validate { config } => {
            let cfg = load(&config)?;
            validate(&cfg)?;
            println!("System-One service configuration valid.");
        }

        Action::Run { config } => {
            let cfg = load(&config)?;
            validate(&cfg)?;

            let mut tasks = JoinSet::new();

            for service in cfg.services {
                if service.enabled {
                    tasks.spawn(supervise(service));
                }
            }

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("omnid: shutting down");
                    tasks.abort_all();
                }
                result = tasks.join_next() => {
                    if let Some(result) = result {
                        result??;
                    }
                    tasks.abort_all();
                }
            }
        }
    }

    Ok(())
}
