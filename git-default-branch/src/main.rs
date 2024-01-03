use auth_git2::GitAuthenticator;
use git2::{Direction, Error, RemoteCallbacks, Repository};
use log::{debug, error, info};

use git_utils::{default_branch, log_level};

fn main() {
    // TODO: get options from clap and pass down a verbose flag
    let log_level = log_level(1);
    env_logger::builder()
        .parse_default_env()
        .filter_module(module_path!(), log_level)
        .filter_module("git-default-branch", log_level)
        .init();

    if let Err(e) = run_git_command() {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_git_command() -> Result<(), Error> {
    match default_branch() {
        Ok(branch) => info!("{}", branch),
        Err(e) => error!("error: {}", e),
    }

    debug!("git default-branch completed successfully");

    Ok(())
}

fn run_git_test() -> Result<(), Error> {
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    let mut remote = repo
        .find_remote("origin")
        .expect("Couldn't find remote 'origin'");
    let authenticator = GitAuthenticator::default();
    let git_config = git2::Config::open_default().unwrap();
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(authenticator.credentials(&git_config));

    let connection = remote
        .connect_auth(Direction::Fetch, Some(callbacks), None)
        .expect("Couldn't connect to remote");

    for head in connection.list()?.iter() {
        debug!("{}\t{}", head.oid(), head.name());
    }

    Ok(())
}
