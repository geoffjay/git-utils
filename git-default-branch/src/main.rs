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
