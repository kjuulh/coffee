use clap::{ArgMatches, Command};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let cli = Command::new("coffee")
        .subcommands(&[repo(), pull_request()])
        .subcommand_required(true)
        .get_matches();

    match cli.subcommand() {
        Some(("pull-request", args)) => handle_pr(args)?,
        Some(("repo", args)) => handle_repo(args)?,
        Some(_) => {}
        None => {}
    }

    Ok(())
}

fn repo() -> Command {
    clap::Command::new("repo")
        .subcommands(&[repo_create()])
        .subcommand_required(true)
        .subcommand_help_heading("repo")
}

fn repo_create() -> Command {
    clap::Command::new("create")
}

fn handle_repo(args: &ArgMatches) -> anyhow::Result<()> {
    tracing::debug!("command: repo");
    Ok(())
}

fn pull_request() -> Command {
    clap::Command::new("pull-request")
        .alias("pr")
        .subcommand_required(true)
        .subcommands(&[pull_request_create()])
        .subcommand_help_heading("pull-request")
}

fn pull_request_create() -> Command {
    clap::Command::new("create")
}

fn handle_pr(args: &ArgMatches) -> anyhow::Result<()> {
    tracing::debug!("command: pr");
    Ok(())
}
