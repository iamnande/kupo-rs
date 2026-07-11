use std::{env, error::Error, fmt, path::PathBuf};

type Result<T> = std::result::Result<T, KupoError>;

#[derive(Debug)]
enum KupoError {
    Usage,
    UnknownStashAction(String),
    Io(std::io::Error),
}

impl fmt::Display for KupoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage => {
                write!(f, "usage: kupo stash open|close")
            }
            Self::UnknownStashAction(action) => {
                write!(f, "unknown stash action: {action}")
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

enum KupoCommand {
    Stash(KupoStashAction),
}

enum KupoStashAction {
    Open,
    Close,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("KupoError: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let command = parse_args()?;

    match command {
        KupoCommand::Stash(action) => stash(action),
    }
}

fn parse_args() -> Result<KupoCommand> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, command, action] if command == "stash" => {
            Ok(KupoCommand::Stash(parse_stash_action(action)?))
        }
        _ => Err(KupoError::Usage),
    }
}

fn parse_stash_action(action: &str) -> Result<KupoStashAction> {
    match action {
        "open" => Ok(KupoStashAction::Open),
        "close" => Ok(KupoStashAction::Close),
        _ => Err(KupoError::UnknownStashAction(action.to_owned())),
    }
}

struct Stash {
    name: String,              // "skate (SanDisk Extreme Pro - 64GB - 280MB/s R, 100MB/s W)"
    block_device_name: String, // "sda1"
}

impl Stash {
    fn open(&self) -> Result<()> {
        println!("opening stash, kupo!");
        println!("- name: {}", self.name);
        println!("- block_device_name: {:?}", self.block_device_name);
        println!("- mount_path: {:?}", self.mount_path());
        Ok(())
    }
    fn close(&self) -> Result<()> {
        println!("closing stash, kupo!");
        Ok(())
    }
    fn mount_path(&self) -> PathBuf {
        PathBuf::from(format!("/mnt/sd-card_{}", self.name))
    }
}

fn stash(action: KupoStashAction) -> Result<()> {
    let stash = Stash {
        // SanDisk Extreme Pro, 64GB, 280MB/100MB
        name: "skate".into(),
        block_device_name: "sda1".into(),
    };
    match action {
        KupoStashAction::Open => stash.open()?,
        KupoStashAction::Close => stash.close()?,
    };

    Ok(())
}
