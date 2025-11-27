use clap::Subcommand;

pub mod echo;

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Ping,
    Echo
}