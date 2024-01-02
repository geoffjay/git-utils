use git2::Error;

use git_utils;

fn main() {
    if let Err(e) = run_git_command() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_git_command() -> Result<(), Error> {
    match git_utils::get_default_branch2() {
        Ok(branch) => println!("{}", branch),
        Err(e) => println!("error: {}", e),
    }

    println!("git default-branch completed successfully");
    Ok(())
}
