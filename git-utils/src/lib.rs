use auth_git2::GitAuthenticator;
use git2::{Config, Direction, Error, RemoteCallbacks, Repository};

mod errors;
mod utils;

use crate::errors::CommandError;
use crate::utils::with_authentication;

pub struct Command {
    pub authenticator: GitAuthenticator,
    pub config: Config,
    pub name: String,
    pub repo: Repository,
}

pub fn log_level(verbose: u8) -> log::LevelFilter {
    match verbose {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        2.. => log::LevelFilter::Trace,
    }
}

pub fn log_prepare(path: &str, command_name: &str) {
    let log_level = log_level(0);

    env_logger::builder()
        .parse_default_env()
        .filter_module(path, log_level)
        .filter_module(command_name, log_level)
        .init();
}

fn agent_callbacks() -> RemoteCallbacks<'static> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(move |_url, username_from_url, _allowed_types| {
        let username = username_from_url.unwrap_or("git");
        git2::Cred::ssh_key_from_agent(username)
    });

    callbacks
}

impl Command {
    pub fn new(name: String) -> Command {
        let root = std::env::args().nth(1).unwrap_or(".".to_string());

        Command {
            authenticator: GitAuthenticator::default(),
            config: Config::open_default().unwrap(),
            name,
            repo: Repository::open(root.as_str()).expect("Couldn't open repository"),
        }
    }

    /// Do the equivalent of `git remote show origin | grep HEAD | awk '{print $3}'`
    pub fn default_branch(self: Command) -> Result<String, Error> {
        let mut remote = self.repo
            .find_remote("origin")
            .expect("Couldn't find remote 'origin'");
        let mut callbacks = RemoteCallbacks::new();

        callbacks.credentials(self.authenticator.credentials(&self.config));

        let result = match remote.connect_auth(Direction::Fetch, Some(callbacks), None) {
            Ok(connection) => {
                let head = connection.default_branch().unwrap();
                let branch = head.as_str().unwrap().strip_prefix("refs/heads/").unwrap();
                Ok(branch.to_string())
            }
            Err(e) => Err(e)
        };

        result
    }

    /// Retrieve the name of the default branch from the remote.
    pub fn default_branch_auth(self: Command) -> Result<String, Error> {
        let mut remote = self.repo
            .find_remote("origin")
            .expect("Couldn't find remote 'origin'");
        let r = remote.clone();
        let url = r.url().unwrap();
        let config = &self.config;

        let result = with_authentication(url, config, |f| {
            let mut proxy_options = git2::ProxyOptions::new();
            proxy_options.auto();

            // let callbacks = agent_callbacks();
            let mut callbacks = git2::RemoteCallbacks::new();
            callbacks.credentials(f);

            let _ = remote
                .connect_auth(Direction::Fetch, Some(callbacks), Some(proxy_options))
                .map_err(CommandError::GitError);

            match remote.default_branch() {
                Ok(head) => {
                    let branch = head.as_str().unwrap().strip_prefix("refs/heads/").unwrap();
                    Ok(branch.to_string())
                }
                Err(e) => Err(CommandError::GitError(e))
            }
        });

        match result {
            Ok(branch) => Ok(branch),
            Err(CommandError::GitError(e)) => Err(e)
        }
    }
}
