use crate::{core::methods::echo::echo, utils::console::clear_screen};

use super::{
    methods::{
        engine::engine, exit::exit, global_help::global_help, status::status, version::version,
    },
    rpc_client::RpcClient,
};

pub struct CommandInterprinter {
    pub rpc_client: Option<RpcClient>,
    is_start_up: bool,
}

impl CommandInterprinter {
    //const EE: &'static str = "../variables/.session.json";
    const DEFAULT_OPERATION: &'static str = "help";

    pub fn new() -> Self {
        Self {
            rpc_client: None,
            is_start_up: false,
        }
    }

    pub async fn execute(&mut self, input: String) {
        let (command, params) = self.parse_input(&input);

        if self.rpc_client.is_none() && self.is_start_up {
            // try autoconnect

            println!("Could not connect to Reprox Engine.\nDo you want to establish connection manually ? ");
            //self.rpc_client = establish_rpc(&params).await;
            // @todo
        }

        if command.eq("clean") || command.eq("clear") || command.eq("cls") {
            clear_screen();
        } else if command.eq("version") {
            version(self.rpc_client.as_ref()).await;
        } else if command.eq("status") {
            status(self.rpc_client.as_ref()).await;
        } else if command.eq("exit") {
            exit();
        } else if command.eq("engine") {
            engine(self, params).await;
        } else if command.eq("help") {
            global_help();
        } else {
            println!("'{}' is not a recognized method", command);
        }
    }

    fn parse_input(&self, input: &str) -> (String, Vec<String>) {
        let mut parts = input.split_whitespace();

        let command = parts.next().unwrap_or_default().to_string();

        let parameters: Vec<String> = parts.map(|s| s.to_string()).collect();

        let parameters: Vec<String> = parameters.into_iter().filter(|s| !s.is_empty()).collect();

        (command, parameters)
    }
}
