mod cli;
mod http;
mod kv;

use clap::Parser;
use crate::cli::{Cli, Command};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Get(get) => {
            http::get(&get.url).await?;
        },
        Command::Post(post) => {
            http::post(&post.url, &post.body).await?;
        },
    }

    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
