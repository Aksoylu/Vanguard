use clap::Subcommand;

use crate::commands::{
    add_http_route::AddHttpRouteArgs, add_https_route::AddHttpsRouteArgs,
    add_iws_route::AddIwsRouteArgs, add_secure_iws_route::AddSecureIwsRouteArgs,
    delete_http_route::DeleteHttpRouteArgs, delete_https_route::DeleteHttpsRouteArgs,
    delete_iws_route::DeleteIwsRouteArgs, delete_secure_iws_route::DeleteSecureIwsRouteArgs,
    echo::EchoArgs, get_route_list::GetRouteListArgs, get_ssl_files::GetSslFilesArgs,
};

#[derive(Subcommand)]
pub enum Commands {
    /// Starts the engine service
    Start,
    /// Sends a ping request to check engine connectivity
    Ping,
    /// Gracefully shutsdowns CLI
    Exit,
    /// Prints version of current cli and engine build versions & package names & OS information
    Version,
    /// Clears entire terminal buffer and display
    Clear,
    /// Sends a message to engine and prints the response. Expected same message content
    Echo(EchoArgs),
    /// Adds a new HTTP route mapping from source to target
    AddHttpRoute(AddHttpRouteArgs),
    /// Adds a new HTTPS route with SSL certificate configuration
    AddHttpsRoute(AddHttpsRouteArgs),
    /// Removes an existing HTTP route by source path
    DeleteHttpRoute(DeleteHttpRouteArgs),
    /// Removes an existing HTTPS route by source path
    DeleteHttpsRoute(DeleteHttpsRouteArgs),
    /// Removes an existing IWS route by source path
    DeleteIwsRoute(DeleteIwsRouteArgs),
    /// Removes an existing Secure IWS route by source path
    DeleteSecureIwsRoute(DeleteSecureIwsRouteArgs),
    /// Retrieves and displays the list of configured routes
    GetRouteList(GetRouteListArgs),
    /// Shows current engine status and configuration details
    Status,
    /// Adds a new Internal Web Service route
    AddIwsRoute(AddIwsRouteArgs),
    /// Adds a new Secure Internal Web Service route with SSL certificate configuration
    AddSecureIwsRoute(AddSecureIwsRouteArgs),
    /// Retrieves and displays the list of uploaded SSL files
    GetSslFiles(GetSslFilesArgs),
}

pub mod add_http_route;
pub mod add_https_route;
pub mod add_iws_route;
pub mod add_secure_iws_route;
pub mod clear_terminal;
pub mod delete_http_route;
pub mod delete_https_route;
pub mod delete_iws_route;
pub mod delete_secure_iws_route;
pub mod echo;
pub mod exit;
pub mod get_route_list;
pub mod get_ssl_files;
pub mod get_status;
pub mod version;
