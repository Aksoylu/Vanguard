use clap::Args;

#[derive(Debug, Args)]
pub struct LogsArgs {
    pub command: Option<String>,
}

pub async fn logs(args: LogsArgs) {
    if args.command.is_none() {
        show_logs_config().await;
        return;
    }

    let command = args.command.unwrap().trim().to_lowercase();

    if command == "trail" {
        println!("trail");
    } else if command == "show" {
        println!("show");
    }
}

pub async fn show_logs_config() {
    println!("show_logs_config");
}
