use core::interprinter::CommandInterprinter;
use std::io;

use utils::{banner::print_welcome_banner, console::separator};

use crate::utils::{console::{clear_screen, console_read}, version::print_version};

mod utils;
mod core;

#[tokio::main]
async fn main() {
    print_welcome_banner();
    print_version();
    separator(5);
    // todo check connection

    loop
    {
        print!(">>> ");
        let input = console_read();
        CommandInterprinter::execute(input).await;
    }
}
