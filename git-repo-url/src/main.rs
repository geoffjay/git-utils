use log::{debug, error};

use git_utils::{log_prepare, Command};

fn main() {
    log_prepare(module_path!(), "git-repo-url");
    let command = Command::new("git-repo-url".to_string());

    match command.repo_url2() {
        Ok(url) => println!("{}", url),
        Err(e) => {
            error!("error: {}", e);
            std::process::exit(1);
        }
    }

    debug!("git repo-url completed successfully");
}
