use core::{interprinter::CommandInterprinter, methods::version::print_cli_version};

use utils::{banner::print_welcome_banner, console::separator};

use crate::utils::console::console_read;

mod build;
mod core;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    print_welcome_banner();
    print_cli_version();
    separator(5);

    let mut interprinter = CommandInterprinter::new();

    loop {
        let input: String = console_read(">>>");
        interprinter.execute(input).await;
    }
}
