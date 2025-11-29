use clap::Subcommand;

use crate::commands::echo::EchoArgs;

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Ping,
    Echo(EchoArgs),
}


pub mod echo;
