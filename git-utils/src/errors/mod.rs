use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    GitError(git2::Error),
}

macro_rules! impl_from_error {
    ($error:ty, $variant:ident) => {
        impl From<$error> for CommandError {
            fn from(error: $error) -> Self {
                CommandError::$variant(error)
            }
        }
    };
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::GitError(ref error) => write!(f, "Git error: {}", error),
        }
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            CommandError::GitError(ref error) => Some(error),
        }
    }
}

impl_from_error!(git2::Error, GitError);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_error_git_error() {
        let error = CommandError::GitError(git2::Error::from_str("test"));
        assert_eq!(error.to_string(), "Git error: test");
    }

    #[test]
    fn test_command_error_git_error_cause() {
        let error = CommandError::GitError(git2::Error::from_str("test"));
        assert_eq!(error.source().unwrap().to_string(), "test");
    }
}
