use std::sync::Arc;

use anyhow::Context;
use clap::{ArgMatches, Command};
use git_url_parse::GitUrl;
use gitea_client::apis::configuration::Configuration;
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

    fn get_base_branch(&self) -> anyhow::Result<Option<String>> {
        match std::process::Command::new("git")
            .args(["remote", "show", "origin"])
            .output()
        {
            Ok(output) => {
                let output = std::str::from_utf8(&output.stdout)?;
                for line in output.lines() {
                    if line.trim().starts_with("HEAD branch: ") {
                        return Ok(Some(
                            line.trim().trim_start_matches("HEAD branch: ").to_string(),
                        ));
                    }
                }

                tracing::info!("found no head branch for origin");
                Ok(None)
            }
            Err(_) => {
                tracing::warn!("failed to query git");
                Ok(None)
            }
        }
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
                object_format_name: None,
            }),
        )
        .await
        .context("failed to create repository")
    }

    pub async fn create_pull_request(
        &self,
        owner: &str,
        repo: &str,
        head: &str,
        base: &str,
        title: &str,
        text: &str,
    ) -> anyhow::Result<gitea_client::models::PullRequest> {
        gitea_client::apis::repository_api::repo_create_pull_request(
            &self.config,
            owner,
            repo,
            Some(gitea_client::models::CreatePullRequestOption {
                assignee: None,
                assignees: None,
                base: Some(base.into()),
                body: Some(text.into()),
                due_date: None,
                head: Some(head.into()),
                labels: None,
                milestone: None,
                title: Some(title.into()),
            }),
        )
        .await
        .context("failed to create pull request")
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

    pub async fn get_repos(
        &self,
        owner: &str,
        limit: i32,
    ) -> anyhow::Result<Vec<gitea_client::models::Repository>> {
        let mut repos = self.get_repos_inner(owner, 1).await?.unwrap_or(Vec::new());
        let mut current_page = 2;

        while let Some(mut new_repos) = self.get_repos_inner(owner, current_page).await? {
            current_page += 1;
            repos.append(&mut new_repos);

            if repos.len() > (limit as usize) {
                tracing::info!("reached limit, aborting");
                break;
            }
        }

        repos.sort_by(|a, b| a.full_name.cmp(&b.full_name));

        Ok(repos)
    }

    async fn get_repos_inner(
        &self,
        owner: &str,
        page: i32,
    ) -> anyhow::Result<Option<Vec<gitea_client::models::Repository>>> {
        tracing::info!("fetching repos for {} at page: {}", owner, page);
        let repos =
            gitea_client::apis::user_api::user_list_repos(&self.config, owner, Some(page), None)
                .await
                .context("failed to list repositories for owner")?;

        if repos.is_empty() {
            Ok(None)
        } else {
            Ok(Some(repos))
        }
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
            .subcommands(&[
                Self::repo_create(),
                Self::repo_view(),
                Self::repo_clone(),
                Self::repo_list(),
            ])
            .subcommand_required(true)
            .subcommand_help_heading("repo")
    }

    fn repo_create() -> Command {
        clap::Command::new("create")
            .arg(clap::Arg::new("visibility").long("visibility"))
            .arg(clap::Arg::new("name").long("name"))
    }

    fn repo_clone() -> Command {
        clap::Command::new("clone")
            .arg(clap::Arg::new("repo"))
            .arg(clap::Arg::new("name"))
    }

    fn repo_list() -> Command {
        clap::Command::new("list")
            .arg(clap::Arg::new("owner"))
            //.arg(clap::Arg::new("visibility").long("visibility"))
            .arg(clap::Arg::new("limit").long("limit"))
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
                            .args(["remote", "add", &remote_name, ssh])
                            .spawn()?
                            .wait()?;
                    } else if let Some(http) = &repo.clone_url {
                        std::process::Command::new("git")
                            .args(["remote", "add", &remote_name, http])
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
            Some(("list", args)) => {
                let owner = args
                    .get_one::<String>("owner")
                    .map(|n| Ok(n.clone()))
                    .unwrap_or_else(|| {
                        inquire::Text::new("owner")
                            .with_help_message("the owner to list repositories from")
                            .prompt()
                            .map(|x| x.to_string())
                    })?;

                // let visibility = args
                //     .get_one::<String>("visibility")
                //     .map(|n| Ok(n.clone()))
                //     .unwrap_or_else(|| {
                //         inquire::Select::new("name", vec!["private", "public"])
                //             .with_vim_mode(true)
                //             .with_help_message("the visibility of the repository")
                //             .prompt()
                //             .map(|x| x.to_string())
                //     })?
                //     .into();

                let limit = args
                    .get_one::<String>("limit")
                    .unwrap_or(&"100".to_string())
                    .to_owned()
                    .parse::<i32>()?;

                let repos = self.client.get_repos(&owner, limit).await?;

                for repo in repos {
                    println!("{}", repo.full_name.unwrap());
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
                let name = args.get_one::<String>("name");

                let repo = match repository.split_once("/") {
                    Some((owner, repository)) => self.client.get_repo(owner, repository).await?,
                    None => {
                        self.client
                            .get_repo(
                                std::env::var("COFFEE_OWNER")
                                    .context("failed to find the COFFEE_OWNER variable, use that or set a fully qualified name: kjuulh/coffee eg.")?
                                    .as_str(),
                                &repository,
                            )
                            .await?
                    }
                };

                if let Some(ssh_url) = repo.ssh_url {
                    let mut args = vec!["clone", &ssh_url];

                    if let Some(name) = name {
                        args.push(name.as_str());
                    }

                    let output = tokio::process::Command::new("git")
                        .args(args.as_slice())
                        .output()
                        .await?;

                    let stdout = output.stdout;
                    let stdout = std::str::from_utf8(stdout.as_slice())?;
                    let stderr = output.stderr;
                    let stderr = std::str::from_utf8(stderr.as_slice())?;
                    if !stdout.is_empty() {
                        println!("{}", stdout);
                    }
                    if !stderr.is_empty() {
                        println!("{}", stderr);
                    }

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
            .arg(clap::Arg::new("base").short('b').long("base"))
            .arg(clap::Arg::new("head").short('H').long("head"))
            .arg(clap::Arg::new("repo").help("<owner>/<repo>").long("repo"))
            .arg(clap::Arg::new("title").long("title"))
            .arg(clap::Arg::new("body").long("body"))
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
            Some(("create", args)) => {
                let base = if let Some(base) = args.get_one::<String>("base") {
                    base.clone()
                } else if let Some(base) = self.client.get_base_branch()? {
                    base
                } else {
                    inquire::prompt_text("your base branch name, usually main or master")
                        .context("failed to get base from user")?
                };
                let head = if let Some(branch) = args.get_one::<String>("head") {
                    branch.clone()
                } else {
                    inquire::prompt_text("your branch name")
                        .context("failed to get head from user")?
                };

                let repo = if let Some(repo) = args.get_one::<String>("repo") {
                    repo.split_once("/")
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                } else {
                    self.client.get_git()?
                };

                let (owner, repo) = repo.ok_or(anyhow::anyhow!("failed to find repo for user"))?;

                let title = if let Some(title) = args.get_one::<String>("title") {
                    inquire::Text::new("title")
                        .with_help_message("title for the pull request")
                        .with_initial_value(title)
                        .prompt()
                        .context("failed to get title for the pullrequest")?
                } else {
                    inquire::prompt_text("title for the pull request")
                        .context("failed to get title for the pullrequest")?
                };
                let description = if let Some(description) = args.get_one::<String>("body") {
                    let mut description = description.trim().to_string();
                    if description.contains("diff --git") {
                        tracing::debug!("cleaning up git diff from message");

                        let mut output = Vec::new();
                        for line in description.lines() {
                            if line.starts_with("diff --git") {
                                break;
                            }

                            output.push(line)
                        }

                        description = output.join("\n").trim().to_string();
                    }

                    inquire::Editor::new("description for the pull request")
                        .with_predefined_text(&description)
                        .prompt()
                        .context("failed to get description for the pullrequest")?
                } else {
                    inquire::prompt_text("description for the pull request")
                        .context("failed to get description for the pullrequest")?
                };

                let pr = self
                    .client
                    .create_pull_request(&owner, &repo, &head, &base, &title, &description)
                    .await?;

                tracing::info!(
                    "created pull request at: {}",
                    pr.html_url.clone().unwrap_or_default()
                );

                if let Some(url) = pr.html_url {
                    webbrowser::open(&url)?;
                }

                if let Some(url) = pr.url {
                    webbrowser::open(&url)?;
                }
            }
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
