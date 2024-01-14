use log::{debug, error};

use git_utils::{log_prepare, Command};

fn main() {
    log_prepare(module_path!(), "git-current-branch");
    let command = Command::new("git-current-branch".to_string());

    match command.current_branch() {
        Ok(branch) => println!("{}", branch),
        Err(e) => {
            error!("error: {}", e);
            std::process::exit(1);
        }
    }

    debug!("git current-branch completed successfully");
}
