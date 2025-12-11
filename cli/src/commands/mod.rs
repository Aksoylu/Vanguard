use clap::Subcommand;

use crate::commands::{
    add_http_route::AddHttpRouteArgs, delete_http_route::DeleteHttpRouteArgs, echo::EchoArgs,
    get_route_list::GetRouteListArgs
};

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Ping,
    Echo(EchoArgs),
    AddHttpRoute(AddHttpRouteArgs),
    DeleteHttpRoute(DeleteHttpRouteArgs),
    GetRouteList(GetRouteListArgs),
}

pub mod add_http_route;
pub mod delete_http_route;
pub mod echo;
pub mod get_route_list;
