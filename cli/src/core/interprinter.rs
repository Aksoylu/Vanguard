use clap::{command, Parser};
use shlex::Shlex;

use crate::commands::{
    add_http_route::add_http_route, add_https_route::add_https_route, add_iws_route::add_iws_route,
    clear_terminal::clear_terminal, delete_http_route::delete_http_route,
    delete_https_route::delete_https_route, echo::echo, exit::exit, get_route_list::get_route_list,
    get_status::get_status, version::version, Commands,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

pub struct Interprinter;

impl Interprinter {
    pub fn new() -> Interprinter {
        Interprinter {}
    }

    async fn execute_command(cli: Cli) {
        match cli.command {
            Commands::Start => start().await,
            Commands::Ping => ping().await,
            Commands::Exit => exit().await,
            Commands::Version => version().await,
            Commands::Clear => clear_terminal().await,
            Commands::Echo(args) => echo(args).await,
            Commands::AddHttpRoute(args) => add_http_route(args).await,
            Commands::AddHttpsRoute(args) => add_https_route(args).await,
            Commands::DeleteHttpRoute(args) => delete_http_route(args).await,
            Commands::DeleteHttpsRoute(args) => delete_https_route(args).await,
            Commands::GetRouteList(args) => get_route_list(args).await,
            Commands::Status => get_status().await,
            Commands::AddIwsRoute(args) => add_iws_route(args).await,
        }
    }
    pub async fn run(&self, input: String) {
        let args = Shlex::new(&input).collect::<Vec<_>>();

        if args.is_empty() {
            return;
        }

        let mut clap_args = vec!["execute".to_string()];
        clap_args.extend(args);

        match Cli::try_parse_from(clap_args) {
            Ok(cli) => {
                Self::execute_command(cli).await;
            }
            Err(e) => {
                eprintln!("{}", e);
            }
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
