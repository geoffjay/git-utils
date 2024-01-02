use std::env;
use git2::{Direction, Repository, RemoteCallbacks};
// use std::process;

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

// pub fn git_credentials_callback(
//     _user: &str,
//     _user_from_url: Option<&str>,
//     _cred: git2::CredentialType,
// ) -> Result<git2::Cred, git2::Error> {
//     match env::var("GPM_SSH_KEY") {
//         Ok(k) => {
//             // debug!("authenticate with private key located in {}", k);
//             git2::Cred::ssh_key("git", None, std::path::Path::new(&k), None)
//         },
//         _ => Err(git2::Error::from_str("unable to get private key from GPM_SSH_KEY")),
//     }
// }

pub fn git_credentials_callback(
    _user: &str,
    _user_from_url: Option<&str>,
    _cred: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
    let user = _user_from_url.unwrap_or("git");

    if _cred.contains(git2::CredentialType::USERNAME) {
        return git2::Cred::username(user);
    }

    match env::var("GPM_SSH_KEY") {
        Ok(k) => {
            // debug!("authenticate with user {} and private key located in {}", user, k);
            git2::Cred::ssh_key(user, None, std::path::Path::new(&k), None)
        },
        _ => Err(git2::Error::from_str("unable to get private key from GPM_SSH_KEY")),
    }
}

pub fn get_default_branch2() -> Result<String, git2::Error> {
    // let repo = match git2::Repository::open(".") {
    //     Ok(repo) => repo,
    //     Err(e) => {
    //         eprintln!("failed: {}", e.message());
    //         process::exit(1)
    //     }
    // };
    //
    // let mut remote = match repo.find_remote("origin") {
    //     Ok(remote) => remote,
    //     Err(e) => {
    //         eprintln!("failed: {}", e.message());
    //         process::exit(1)
    //     }
    // };
    //
    // match remote.connect(git2::Direction::Fetch) {
    //     Ok(_) => {
    //         let head = remote.default_branch().unwrap();
    //         head.as_str().unwrap().to_string()
    //     }
    //     Err(e) => {
    //         eprintln!("failed: {}", e.message());
    //         process::exit(1)
    //     }
    // }

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(git_credentials_callback);
    let repo = Repository::open(".")?;
    let remote = "origin";
    let mut remote = repo
        .find_remote(remote)
        .or_else(|_| repo.remote_anonymous(remote))?;

    let connection = remote.connect_auth(Direction::Fetch, Some(callbacks), None)?;

    for head in connection.list()?.iter() {
        println!("{}\t{}", head.oid(), head.name());
    }

    Ok("test".to_string())
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
