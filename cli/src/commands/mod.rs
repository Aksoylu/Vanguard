use clap::Subcommand;

use crate::commands::{add_http_route::AddHttpRouteArgs, echo::EchoArgs};

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Ping,
    Echo(EchoArgs),
    AddHttpRoute(AddHttpRouteArgs)
}

pub mod echo;
pub mod add_http_route;