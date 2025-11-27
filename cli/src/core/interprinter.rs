use clap::{Parser};

use crate::commands::{Commands, echo::echo};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

struct Interprinter {
    cli: Cli
}

impl Interprinter{
    pub fn new() -> Self {
        
        let cli = Cli::parse();
        Interprinter { cli }
    }

    pub async fn run(&self) {
        match &self.cli.command {
            Commands::Start => start().await,
            Commands::Ping => ping().await,
            Commands::Echo => echo().await,
        }
    }
}

async fn start() {
    println!("Starting async task...");
}

async fn ping() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("pong");
}