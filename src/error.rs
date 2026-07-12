use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, KupoError>;

#[derive(Debug)]
pub enum KupoError {
    Usage,
    UnknownStashAction(String),
    StashOpen,
    StashClosed,
    MountFailed(std::process::ExitStatus),
    UmountFailed(std::process::ExitStatus),
    Io(std::io::Error),
}

impl fmt::Display for KupoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage => {
                write!(f, "usage: kupo stash <open|status|close>")
            }
            Self::UnknownStashAction(action) => {
                write!(f, "unknown stash action: {action}")
            }
            Self::StashOpen => {
                write!(f, "stash is open, kupo!")
            }
            Self::StashClosed => {
                write!(f, "stash is closed, kupo!")
            }
            Self::MountFailed(status) => {
                write!(f, "mount has failed to appear, kupo! ({status})")
            }
            Self::UmountFailed(status) => {
                write!(f, "umount has failed to dissappear, kupo! ({status})")
            }
            Self::Io(err) => {
                write!(f, "{err}")
            }
        }
    }
}

impl Error for KupoError {}

impl From<std::io::Error> for KupoError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
