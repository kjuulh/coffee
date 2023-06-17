use clap::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli = Command::new("coffee").subcommands(&[repo()]).get_matches();

    match cli.subcommand() {
        Some(_) => {}
        None => {}
    }

    Ok(())
}

fn repo() -> Command {
    clap::Command::new("repo")
        .subcommands(&[repo_create()])
        .subcommand_help_heading("repo")
}

fn repo_create() -> Command {
    clap::Command::new("create")
}
