use log::{debug, error};

use git_utils::{log_prepare, Command};

fn main() {
    log_prepare(module_path!(), "git-up");
    let command = Command::new("git-up".to_string());

    match command.sync() {
        Ok(_) => (),
        Err(e) => {
            error!("error: {}", e);
            std::process::exit(1);
        }
    }

    debug!("git up completed successfully");
}