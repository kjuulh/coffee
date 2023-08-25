use std::sync::Arc;

use anyhow::Context;
use clap::{ArgMatches, Command};
use git_url_parse::GitUrl;
use gitea_client::apis::configuration::{ApiKey, Configuration};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

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
        Some(("repo", args)) => repo.handle_repo(args).await?,
        Some(_) => {}
        None => {}
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Visibility {
    Public,
    Private,
}

impl From<&str> for Visibility {
    fn from(s: &str) -> Self {
        match s {
            "public" => Self::Public,
            "private" => Self::Private,
            _ => panic!("option is not possible: {}", s),
        }
    }
}

impl From<String> for Visibility {
    #[inline(always)]
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

struct GiteaClient {
    config: Configuration,
}

impl GiteaClient {
    pub fn new(url: String, token: String) -> Self {
        let mut config = Configuration::new();

        config.base_path = url;

        Self { config }
    }

    fn get_branch(&self) -> anyhow::Result<Option<String>> {
        match std::process::Command::new("git")
            .args(&["branch", "--show-current"])
            .output()
        {
            Ok(output) => {
                let output = std::str::from_utf8(&output.stdout)?;
                return Ok(Some(output.to_owned()));
            }
            Err(e) => {
                tracing::debug!("failed to query git, returning none: {}", e.to_string());

                return Ok(None);
            }
        }
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
                        let git_url = GitUrl::parse(remote_url)
                            .map_err(|e| anyhow::anyhow!("{}", e.to_string()))?;

                        return Ok(Some((git_url.owner.unwrap(), git_url.name)));
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

    pub async fn create_repo(
        &self,
        name: &str,
        visibility: Visibility,
    ) -> anyhow::Result<gitea_client::models::Repository> {
        gitea_client::apis::user_api::create_current_user_repo(
            &self.config,
            Some(gitea_client::models::CreateRepoOption {
                auto_init: None,
                default_branch: Some("main".into()),
                description: None,
                gitignores: None,
                issue_labels: None,
                license: None,
                name: name.to_string(),
                private: Some(visibility == Visibility::Private),
                readme: None,
                template: None,
                trust_model: None,
            }),
        )
        .await
        .context("failed to create repository")
    }

    pub async fn get_repo(
        &self,
        owner: &str,
        name: &str,
    ) -> anyhow::Result<gitea_client::models::Repository> {
        gitea_client::apis::repository_api::repo_get(&self.config, owner, name)
            .await
            .context("failed to get repository")
    }

    pub async fn list_open_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> anyhow::Result<Vec<gitea_client::models::PullRequest>> {
        self.list_pull_requests(owner, repo, Some("open")).await
    }

    pub async fn list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
        state: Option<&str>,
    ) -> anyhow::Result<Vec<gitea_client::models::PullRequest>> {
        let pr = gitea_client::apis::repository_api::repo_list_pull_requests(
            &self.config,
            owner,
            repo,
            state,
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

struct Repo {
    client: Arc<GiteaClient>,
}

impl Repo {
    pub fn new(gitea_client: Arc<GiteaClient>) -> Self {
        Self {
            client: gitea_client,
        }
    }

    fn repo() -> Command {
        clap::Command::new("repo")
            .subcommands(&[Self::repo_create(), Self::repo_view(), Self::repo_clone()])
            .subcommand_required(true)
            .subcommand_help_heading("repo")
    }

    fn repo_create() -> Command {
        clap::Command::new("create")
            .arg(clap::Arg::new("visibility").long("visibility"))
            .arg(clap::Arg::new("name").long("name"))
    }

    fn repo_clone() -> Command {
        clap::Command::new("clone").arg(clap::Arg::new("repo"))
    }

    fn repo_view() -> Command {
        clap::Command::new("view")
            .arg(
                clap::Arg::new("web")
                    .long("web")
                    .short('w')
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(clap::Arg::new("repository").alias("repo"))
    }

    async fn handle_repo(&self, args: &ArgMatches) -> anyhow::Result<()> {
        tracing::debug!("command: repo");

        match args.subcommand() {
            Some(("create", args)) => {
                let name = args
                    .get_one::<String>("name")
                    .map(|n| Ok(n.clone()))
                    .unwrap_or_else(|| {
                        inquire::Text::new("name")
                            .with_help_message("the name of the repository you want to create")
                            .prompt()
                    })?;
                let visibility = args
                    .get_one::<String>("visibility")
                    .map(|n| Ok(n.clone()))
                    .unwrap_or_else(|| {
                        inquire::Select::new("name", vec!["private", "public"])
                            .with_vim_mode(true)
                            .with_help_message("the visibility of the repository")
                            .prompt()
                            .map(|x| x.to_string())
                    })?
                    .into();

                let repo = self.client.create_repo(&name, visibility).await?;
                println!("created repo: {}", &repo.full_name.unwrap());

                if inquire::Confirm::new("set remote")
                    .with_default(true)
                    .prompt()?
                {
                    let remote_name = inquire::Text::new("remote name")
                        .with_default("origin")
                        .prompt()?;

                    if let Some(ssh) = &repo.ssh_url {
                        std::process::Command::new("git")
                            .args(&["remote", "add", &remote_name, &ssh])
                            .spawn()?
                            .wait()?;
                    } else if let Some(http) = &repo.clone_url {
                        std::process::Command::new("git")
                            .args(&["remote", "add", &remote_name, &http])
                            .spawn()?
                            .wait()?;
                    }
                }
            }
            Some(("view", args)) => {
                let git_repo = self.client.get_git()?;
                let (repo_owner, repo_name) = git_repo
                    .map(|(owner, name)| (Some(owner), Some(name)))
                    .unwrap_or((None, None));

                let repository = args.get_one::<String>("repository");
                let repository = default_or(repo_name.as_ref(), repository)?;
                let web = args.get_one::<bool>("web");

                let repo = self
                    .client
                    .get_repo(&repo_owner.unwrap(), &repository)
                    .await?;

                match repo.html_url {
                    Some(link) => webbrowser::open(&link)?,
                    None => anyhow::bail!("failed to find a valid link to repository"),
                }
            }

            Some(("clone", args)) => {
                let repository = args.get_one::<String>("repo");
                let repository = match repository {
                    None => inquire::Text::new("repo")
                        .with_help_message(
                            "Please entire repository owner and name (kjuulh/coffee or just coffee) eg.",
                        )
                        .prompt()?,
                    Some(repo) => repo.clone(),
                };

                let repo = match repository.split_once("/") {
                    Some((owner, repository)) => self.client.get_repo(owner, repository).await?,
                    None => {
                        self.client
                            .get_repo(std::env::var("COFFEE_OWNER")?.as_str(), &repository)
                            .await?
                    }
                };

                if let Some(ssh_url) = repo.ssh_url {
                    let output = tokio::process::Command::new("git")
                        .arg("clone")
                        .arg(&ssh_url)
                        .output()
                        .await?;

                    let stdout = output.stdout;
                    let stdout = std::str::from_utf8(stdout.as_slice())?;
                    let stderr = output.stderr;
                    let stderr = std::str::from_utf8(stderr.as_slice())?;
                    println!("{}", stdout);
                    println!("{}", stderr);

                    if !output.status.success() {
                        anyhow::bail!("failed to clone repository using link: {ssh_url}");
                    }

                    println!("successfully cloned repository {}", repo.name.unwrap());
                }
            }

            _ => todo!(),
        }

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
            .subcommands(&[
                Self::pull_request_create(),
                Self::pull_request_list(),
                Self::pull_request_view(),
            ])
            .subcommand_help_heading("pull-request")
    }

    fn pull_request_create() -> Command {
        clap::Command::new("create")
    }

    fn pull_request_list() -> Command {
        clap::Command::new("list").alias("ls")
    }

    fn pull_request_view() -> Command {
        clap::Command::new("view")
            .alias("v")
            .arg(clap::Arg::new("branch").short('b').long("branch"))
            .arg(
                clap::Arg::new("web")
                    .short('w')
                    .long("web")
                    .num_args(0)
                    .required(false),
            )
    }

    async fn handle_pr(&self, args: &ArgMatches) -> anyhow::Result<()> {
        tracing::debug!("command: pr");
        let git = self.client.get_git()?;
        let git = git.as_ref();

        let owner = args.get_one::<String>("org");
        let owner = default_or(owner, git.map(|(owner, _)| owner))?;
        let repo = args.get_one::<String>("repository");
        let repo = default_or(repo, git.map(|(_, repo)| repo))?;

        match args.subcommand() {
            Some(("create", _args)) => todo!(),
            Some(("list", _args)) => {
                let pull_requests = self.client.list_open_pull_requests(&owner, &repo).await?;
                for pull_request in pull_requests {
                    println!(
                        "{}: {}",
                        pull_request.title.unwrap_or_default(),
                        pull_request.html_url.unwrap_or_default()
                    );
                }
            }
            Some(("view", args)) => {
                let git_branch = self.client.get_branch()?;
                let git_branch = git_branch.as_ref();

                let branch = args.get_one::<String>("branch");
                let branch = default_or(branch, git_branch)?;
                let web = args.get_one::<bool>("web");

                let pull_requests = self.client.list_pull_requests(&owner, &repo, None).await?;
                if pull_requests.is_empty() {
                    tracing::info!("no pull request found")
                }
                for pull_request in pull_requests {
                    if let Some(head) = pull_request.head {
                        if let Some(label) = head.label {
                            if label == branch.trim() {
                                match web {
                                    Some(true) => {
                                        webbrowser::open(&pull_request.html_url.unwrap())?
                                    }
                                    Some(false) => {
                                        println!(
                                            "{}: {}",
                                            pull_request.title.unwrap_or_default(),
                                            pull_request.html_url.unwrap_or_default()
                                        );
                                    }
                                    None => {
                                        println!(
                                            "{}: {}",
                                            pull_request.title.unwrap_or_default(),
                                            pull_request.html_url.unwrap_or_default()
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(_) => todo!(),
            None => todo!(),
        }
        Ok(())
    }
}

fn default_or(arg: Option<&String>, fallback: Option<&String>) -> anyhow::Result<String> {
    let arg = match arg {
        None => match fallback {
            Some(fallback) => fallback,
            None => {
                anyhow::bail!("failed to find fallback git remote");
            }
        },
        Some(arg) => arg,
    };
    Ok(arg.clone())
}
