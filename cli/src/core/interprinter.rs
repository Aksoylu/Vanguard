use clap::{command, Parser};
use shlex::Shlex;

use crate::commands::{
    Commands, 
    add_http_route::add_http_route, 
    delete_http_route::delete_http_route, 
    echo::echo, 
    get_http_route_list::get_http_route_list
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
            Commands::Echo(args) => echo(args).await,
            Commands::AddHttpRoute(args) => add_http_route(args).await,
            Commands::DeleteHttpRoute(args) => delete_http_route(args).await,
            Commands::GetHttpRouteList => get_http_route_list().await
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
