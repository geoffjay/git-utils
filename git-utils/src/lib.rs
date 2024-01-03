use auth_git2::GitAuthenticator;
use git2::{Direction, Error, RemoteCallbacks, Repository};

pub fn log_level(verbose: u8) -> log::LevelFilter {
    match verbose {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        2.. => log::LevelFilter::Trace,
    }
}

/// Do the equivalent of `git remote show origin | grep HEAD | awk '{print $3}'`
pub fn get_default_branch() -> String {
    let output = std::process::Command::new("git")
        .arg("remote")
        .arg("show")
        .arg("origin")
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    let output = output.split("\n").collect::<Vec<&str>>();
    let output = output
        .iter()
        .filter(|&x| x.contains("HEAD branch"))
        .collect::<Vec<&&str>>();
    let output = output[0].split(" ").collect::<Vec<&str>>();
    let output = output
        .iter()
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&&str>>();
    let output = output[2];
    output.to_string()
}

/// Do the equivalent of `git remote show origin | grep HEAD | awk '{print $3}'`
pub fn default_branch() -> Result<String, Error> {
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    let mut remote = repo
        .find_remote("origin")
        .expect("Couldn't find remote 'origin'");
    let authenticator = GitAuthenticator::default();
    let git_config = git2::Config::open_default().unwrap();
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(authenticator.credentials(&git_config));

    // let connection = remote
    //     .connect_auth(Direction::Fetch, Some(callbacks), None)
    //     .expect("Couldn't connect to remote");

    // for head in connection.list()?.iter() {
    //     debug!("{}\t{}", head.oid(), head.name());
    // }

    let result = match remote.connect_auth(Direction::Fetch, Some(callbacks), None) {
        Ok(connection) => {
            // let head = remote.default_branch().unwrap();
            // Ok(head.as_str().unwrap().to_string())
            let head = connection.default_branch().unwrap();
            Ok(head.as_str().unwrap().to_string())
        }
        Err(e) => Err(e)
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_default_branch();
        assert_eq!(result, "main");
    }
}
