use log::{debug, error};

use git_utils::{log_prepare, Command};

fn main() {
    log_prepare(module_path!(), "git-default-branch");
    let command = Command::new("git-default-branch".to_string());

    match command.default_branch() {
        Ok(branch) => println!("{}", branch),
        Err(e) => {
            error!("error: {}", e);
            std::process::exit(1);
        }
    }

    debug!("git default-branch completed successfully");
}
