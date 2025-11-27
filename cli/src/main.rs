mod assets;
mod boot;
mod commands;
mod constants;
mod core;
mod models;
mod utils;

use crate::assets::{banner::print_banner, startup_disclaimer::print_startup_disclaimer};

use utils::console::separator;

use crate::utils::console::console_read;

#[tokio::main]
async fn main() {
    print_startup_disclaimer();
    print_banner();
    separator(5);

    let boot_result = Boot::init();

    /*
    let mut interprinter = Interprinter::new();

    loop {
        let input: String = console_read(">>>");
        interprinter.execute(input).await;
    }
     */
}
