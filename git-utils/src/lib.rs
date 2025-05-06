use git2::{Config, Direction, Error, ProxyOptions, RemoteCallbacks, Repository};
use regex::Regex;

mod errors;
mod utils;

use crate::errors::CommandError;
use crate::utils::with_authentication;

pub struct Command {
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

impl Clone for Command {
    fn clone(&self) -> Self {
        let repo = &self.repo;

        Command {
            config: repo.config().unwrap(),
            name: self.name.clone(),
            repo: Repository::open_from_env().expect("Couldn't open repository"),
        }
    }
}

impl Command {
    pub fn new(name: String) -> Command {
        Command {
            config: Config::open_default().unwrap(),
            name,
            repo: Repository::open_from_env().expect("Couldn't open repository"),
        }
    }

    /// Retrieve the name of the default branch from the remote. This does the
    /// equivalent of `git remote show origin | grep HEAD | awk '{print $3}'`
    pub fn default_branch(self: Command) -> Result<String, Error> {
        let mut remote = self
            .repo
            .find_remote("origin")
            .expect("Couldn't find remote 'origin'");
        let r = remote.clone();
        let url = r.url().unwrap();
        let config = &self.config;

        let result = with_authentication(url, config, |f| {
            let mut proxy_options = ProxyOptions::new();
            proxy_options.auto();

            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(f);

            let _ = remote
                .connect_auth(Direction::Fetch, Some(callbacks), Some(proxy_options))
                .map_err(CommandError::GitError);

            match remote.default_branch() {
                Ok(head) => {
                    let branch = head.as_str().unwrap().strip_prefix("refs/heads/").unwrap();
                    Ok(branch.to_string())
                }
                Err(e) => Err(CommandError::GitError(e)),
            }
        });

        // TODO: I think I can get rid of this match if I replace the return
        // type with `Result<String, CommandError>`
        match result {
            Ok(branch) => Ok(branch),
            Err(CommandError::GitError(e)) => Err(e),
        }
    }

    /// Retrive the name of the current branch for the repository. This does the
    /// equivalent of `git rev-parse --abbrev-ref HEAD`
    pub fn current_branch(self: Command) -> Result<String, Error> {
        let head = self.repo.head().unwrap();
        let branch = head.shorthand().unwrap();
        Ok(branch.to_string())
    }

    /// Print the url of the repository using local config. This does the equivalent of
    /// `git config --get remote.origin.url`
    pub fn repo_url(self: Command) -> Result<String, Error> {
        let url = self
            .repo
            .config()
            .unwrap()
            .get_string("remote.origin.url")
            .expect("Invalid key: 'remote.origin.url'");

        Ok(url)
    }

    /// Print the owner and name of the repository. This does the equivalent of
    /// `git remote show origin | grep Fetch | sed "s/^.*\:\(.*\)\.git/\1/"`
    ///
    /// This should support both ssh and https urls:
    ///
    /// let url = "git@github.com:geoffjay/git-utils.git".to_string();
    /// let url = "https://github.com/geoffjay/git-utils.git".to_string();
    ///
    /// at some point adding a mocking library should be done to test each.
    pub fn repo_title(self: Command) -> Result<String, Error> {
        let url = self.repo_url()?;

        // match the owner and repository name from the url
        let re = Regex::new(
            r"(?x)
            ^((https://)|(git@))
            ([\w\.]*)
            ([:\/])
            (?P<owner>[\w\-_\.]*)
            \/
            (?P<name>[\w\-_\.]*)
            (\.git)$
        ",
        )
        .unwrap();
        let res = re.replace_all(&url, "$owner/$name");

        Ok(res.to_string())
    }

    /// Synchronize the local with the remote repository. This will perform:
    ///
    /// - git fetch --no-tags --all
    /// - git merge --ff-only
    /// - remove any branches that contain ": gone]" from `git branch -vv`
    /// - if the current branch is not the default branch, `git fetch origin default_branch:default_branch`
    pub fn sync(self: Command) -> Result<(), Error> {
        let default_branch = self.clone().default_branch()?;
        let current_branch = self.clone().current_branch()?;
        let config = &self.repo.config().unwrap();

        // fetch all branches
        let mut remote = self
            .repo
            .find_remote("origin")
            .expect("Couldn't find remote 'origin'");
        let r = remote.clone();
        let url = r.url().unwrap();

        log::debug!("fetching from {}", url);

        let result = with_authentication(url, config, |f| {
            let mut proxy_options = ProxyOptions::new();
            proxy_options.auto();

            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(f);

            log::debug!("connecting to remote");
            let mut fetch_options = git2::FetchOptions::new();
            fetch_options
                .remote_callbacks(callbacks)
                .proxy_options(proxy_options);

            log::debug!("fetching from remote");
            // equivalent to: x-fetch-all = fetch --no-tags --all -p
            let fetch_result = remote.fetch(
                &["--no-tags", "--all", "-p"],
                Some(&mut fetch_options),
                None,
            )?;
            log::debug!("fetch result: {:?}", fetch_result);

            // Perform fast-forward merge
            let current_branch = self.repo.head()?.shorthand().unwrap().to_string();
            let upstream_branch = format!("refs/remotes/origin/{}", current_branch);

            log::debug!("upstream branch: {}", upstream_branch);

            // equivalent to: x-merge-ff = merge --ff-only || true
            if let Ok(upstream_oid) = self.repo.refname_to_id(&upstream_branch) {
                let annotated_commit = self.repo.find_annotated_commit(upstream_oid)?;
                // Attempt fast-forward merge
                match self.repo.merge_analysis(&[&annotated_commit]) {
                    Ok((analysis, _)) => {
                        if analysis.is_fast_forward() {
                            let mut reference = self.repo.find_reference("HEAD")?;
                            reference.set_target(upstream_oid, "Fast-forward merge")?;
                            self.repo
                                .checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;
                        }
                        // If not fast-forward, do nothing (equivalent to `|| true`)
                    }
                    Err(_) => {} // Ignore merge analysis errors
                }
            }

            // Remove branches that contain ": gone]"
            // equivalent to: x-branch-tidy = "!f() { git branch -vv | grep ': gone]' | awk '{print $1}' | xargs -n 1 git branch -D; }; f"
            log::debug!("cleaning up branches");
            let branches = self.repo.branches(Some(git2::BranchType::Local))?;
            for branch_result in branches {
                let (mut branch, _) = branch_result?;
                let branch_name = branch.name()?.unwrap_or("").to_string();

                // Skip the current branch
                if branch_name == current_branch {
                    continue;
                }

                let upstream = match branch.upstream() {
                    Ok(_) => true,
                    Err(_) => false,
                };

                // If upstream doesn't exist (which means it's "gone"), delete the branch
                if !upstream {
                    log::debug!("removing branch {}", branch_name);
                    branch.delete()?;
                }
            }

            Ok(())
        });

        // If we successfully performed the first part, check if we need to fetch the default branch
        // equivalent to: x-fetch-branch = !"f() { git fetch origin $1:$1; }; f"
        // equivalent to: if [ "$current" != "$default" ]; then git x-fetch-branch $default; fi;
        if result.is_ok() && current_branch != default_branch {
            log::debug!("fetching default branch: {}", default_branch);

            // Create a new authentication call to fetch the default branch
            let default_branch_result = with_authentication(url, config, |f| {
                let mut callbacks = RemoteCallbacks::new();
                callbacks.credentials(f);

                let mut proxy_options = ProxyOptions::new();
                proxy_options.auto();

                let mut fetch_options = git2::FetchOptions::new();
                fetch_options
                    .remote_callbacks(callbacks)
                    .proxy_options(proxy_options);

                let refspec = format!("{}:{}", default_branch, default_branch);
                log::debug!("refspec: {}", refspec);

                let fetch_result = remote.fetch(&[&refspec], Some(&mut fetch_options), None)?;
                Ok(fetch_result)
            });

            if let Err(CommandError::GitError(e)) = default_branch_result {
                return Err(e);
            }
        }

        match result {
            Ok(_) => Ok(()),
            Err(CommandError::GitError(e)) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_branch() {
        let command = Command::new("git-default-branch".to_string());
        let branch = command.default_branch().unwrap();

        assert_eq!(branch, "main");
    }

    #[test]
    fn test_current_branch() {
        let command = Command::new("git-current-branch".to_string());
        let branch = command.current_branch().unwrap();

        assert_eq!(branch, "main");
    }

    #[test]
    fn test_repo_url() {
        let command = Command::new("git-repo-url".to_string());
        let url = command.repo_url().unwrap();

        assert_eq!(url, "git@github.com:geoffjay/git-utils.git");
    }

    #[test]
    fn test_repo_title() {
        let command = Command::new("git-repo-title".to_string());
        let title = command.repo_title().unwrap();

        assert_eq!(title, "geoffjay/git-utils");
    }
}
