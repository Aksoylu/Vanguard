use clap::Args;

#[derive(Debug, Args)]
pub struct EchoArgs {
    pub message: Vec<String>,
}

pub async fn echo(args: EchoArgs) {
    let output = args.message.join(" ");
    println!("echo response: {}", output);
}