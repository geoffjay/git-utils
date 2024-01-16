use log::{debug, error};

use git_utils::{log_prepare, Command};

fn main() {
    log_prepare(module_path!(), "git-repo-title");
    let command = Command::new("git-repo-title".to_string());

    match command.repo_title() {
        Ok(title) => println!("{}", title),
        Err(e) => {
            error!("error: {}", e);
            std::process::exit(1);
        }
    }

    debug!("git repo-title completed successfully");
}
