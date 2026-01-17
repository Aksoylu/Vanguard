use clap::Subcommand;

use crate::commands::{
    add_http_route::AddHttpRouteArgs, add_iws_route::AddIwsRouteArgs, delete_http_route::DeleteHttpRouteArgs, echo::EchoArgs, get_route_list::GetRouteListArgs
};

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Ping,
    /// Gracefully shutsdowns CLI
    Exit,
    /// Prints version of current cli and engine build versions & package names & OS information
    Version,
    // Clears entire terminal buffer and display
    Clear,
    /// Sends a message to engine and prints the response. Expected same message content
    Echo(EchoArgs),
    AddHttpRoute(AddHttpRouteArgs),
    DeleteHttpRoute(DeleteHttpRouteArgs),
    GetRouteList(GetRouteListArgs),
    Status,
    /// 
    AddIwsRoute(AddIwsRouteArgs)
}

pub mod add_http_route;
pub mod delete_http_route;
pub mod echo;
pub mod get_route_list;
pub mod clear_terminal;
pub mod version;
pub mod exit;
pub mod get_status;
pub mod add_iws_route;