use clap::Subcommand;

use crate::commands::{add_http_route::AddHttpRouteArgs, delete_http_route::DeleteHttpRouteArgs, echo::EchoArgs};

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Ping,
    Echo(EchoArgs),
    AddHttpRoute(AddHttpRouteArgs),
    DeleteHttpRoute(DeleteHttpRouteArgs),
    GetHttpRouteList
}

pub mod echo;
pub mod add_http_route;
pub mod delete_http_route;
pub mod get_http_route_list;