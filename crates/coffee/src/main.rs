use std::sync::Arc;

use clap::{ArgMatches, Command};
use git_url_parse::GitUrl;
use gitea_client::apis::configuration::{ApiKey, Configuration};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let cli = Command::new("coffee")
        .arg(
            clap::Arg::new("url")
                .long("url")
                .env("GITEA_URL")
                .required(true),
        )
        .arg(
            clap::Arg::new("token")
                .long("token")
                .env("GITEA_ACCESS_TOKEN")
                .required(true),
        )
        .subcommands(&[Repo::repo(), PullRequest::pull_request()])
        .subcommand_required(true)
        .get_matches();

    let url = cli.get_one::<String>("url").unwrap();
    let token = cli.get_one::<String>("token").unwrap();

    let gitea_client = Arc::new(GiteaClient::new(url.to_owned(), token.to_owned()));

    let repo = Repo::new(gitea_client.clone());
    let pull_request = PullRequest::new(gitea_client.clone());

    match cli.subcommand() {
        Some(("pull-request", args)) => pull_request.handle_pr(args).await?,
        Some(("repo", args)) => repo.handle_repo(args)?,
        Some(_) => {}
        None => {}
    }

    Ok(())
}

struct GiteaClient {
    config: Configuration,
}

impl GiteaClient {
    pub fn new(url: String, token: String) -> Self {
        let mut config = Configuration::new();
        config.bearer_access_token = Some(token);
        config.base_path = url;

        Self { config }
    }

    fn get_git(&self) -> anyhow::Result<Option<(String, String)>> {
        match std::process::Command::new("git")
            .args(&["remote", "-v", "show"])
            .output()
        {
            Ok(output) => {
                let domain = url::Url::parse(&self.config.base_path)?;
                let output = std::str::from_utf8(&output.stdout)?;

                for line in output.split("\n") {
                    let mut split = line.split_whitespace();
                    let _ = split.next();

                    if let Some(remote_url) = split.next() {
                        if remote_url.contains(domain.host_str().unwrap()) {
                            let git_url = GitUrl::parse(remote_url)
                                .map_err(|e| anyhow::anyhow!("{}", e.to_string()))?;

                            return Ok(Some((git_url.owner.unwrap(), git_url.name)));
                        }
                    }
                }
            }
            Err(e) => {
                tracing::debug!("failed to query git, returning none: {}", e.to_string());

                return Ok(None);
            }
        }

        Ok(None)
    }

    pub async fn list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> anyhow::Result<Vec<gitea_client::models::PullRequest>> {
        let pr = gitea_client::apis::repository_api::repo_list_pull_requests(
            &self.config,
            owner,
            repo,
            Some("open"),
            Some("recentupdate"),
            None,
            None,
            None,
            None,
        )
        .await?;

        Ok(pr)
    }
}

struct Repo {}

impl Repo {
    pub fn new(gitea_client: Arc<GiteaClient>) -> Self {
        Self {}
    }

    fn repo() -> Command {
        clap::Command::new("repo")
            .subcommands(&[Self::repo_create()])
            .subcommand_required(true)
            .subcommand_help_heading("repo")
    }

    fn repo_create() -> Command {
        clap::Command::new("create")
    }

    fn handle_repo(&self, args: &ArgMatches) -> anyhow::Result<()> {
        tracing::debug!("command: repo");
        Ok(())
    }
}

struct PullRequest {
    client: Arc<GiteaClient>,
}

impl PullRequest {
    pub fn new(gitea_client: Arc<GiteaClient>) -> Self {
        Self {
            client: gitea_client,
        }
    }

    fn pull_request() -> Command {
        clap::Command::new("pull-request")
            .alias("pr")
            .arg(
                clap::Arg::new("repository")
                    .alias("repo")
                    .short('r')
                    .long("repo"),
            )
            .arg(clap::Arg::new("org").long("org").env("GITEA_ORG"))
            .subcommand_required(true)
            .subcommands(&[Self::pull_request_create(), Self::pull_request_list()])
            .subcommand_help_heading("pull-request")
    }

    fn pull_request_create() -> Command {
        clap::Command::new("create")
    }

    fn pull_request_list() -> Command {
        clap::Command::new("list").alias("ls")
    }

    async fn handle_pr(&self, args: &ArgMatches) -> anyhow::Result<()> {
        tracing::debug!("command: pr");
        let git = self.client.get_git()?;
        let git = git.as_ref();

        let owner = args.get_one::<String>("org");
        let owner = Self::default_or(owner, git.map(|(owner, _)| owner))?;
        let repo = args.get_one::<String>("repository");
        let repo = Self::default_or(repo, git.map(|(_, repo)| repo))?;

        match args.subcommand() {
            Some(("create", _args)) => todo!(),
            Some(("list", _args)) => {
                let pull_requests = self.client.list_pull_requests(&owner, &repo).await?;
                for pull_request in pull_requests {
                    println!(
                        "{}: {}",
                        pull_request.title.unwrap_or_default(),
                        pull_request.html_url.unwrap_or_default()
                    );
                }
            }
            Some(_) => todo!(),
            None => todo!(),
        }
        Ok(())
    }

    fn default_or(arg: Option<&String>, fallback: Option<&String>) -> anyhow::Result<String> {
        let arg = match arg {
            None => match fallback {
                Some(fallback) => fallback,
                None => {
                    anyhow::bail!("missing field");
                }
            },
            Some(arg) => arg,
        };

        Ok(arg.clone())
    }
}
