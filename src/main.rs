use std::{env, error::Error, fmt, path::PathBuf};

type Result<T> = std::result::Result<T, KupoError>;

#[derive(Debug)]
enum KupoError {
    Usage,
    UnknownStashAction(String),
    StashOpen,
    StashClosed,
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
            Self::StashOpen => {
                write!(f, "stash is open, kupo!")
            }
            Self::StashClosed => {
                write!(f, "stash is closed, kupo!")
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

#[derive(Debug)]
enum StashStatus {
    Open,
    Closed,
}

impl fmt::Display for StashStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Closed => write!(f, "closed"),
        }
    }
}

struct Stash {
    name: String,              // "skate (SanDisk Extreme Pro - 64GB - 280MB/s R, 100MB/s W)"
    block_device_name: String, // "sda1"
}

impl Stash {
    fn new(name: String, block_device_name: String) -> Self {
        Self {
            name,
            block_device_name,
        }
    }

    fn ensure_mount_path_exists(&self) -> Result<()> {
        if let StashStatus::Open = self.status() {
            return Err(KupoError::StashOpen);
        }

        std::fs::create_dir_all(self.mount_path())?;

        Ok(())
    }

    fn ensure_mount_path_removed(&self) -> Result<()> {
        if let StashStatus::Closed = self.status() {
            return Err(KupoError::StashClosed);
        }

        std::fs::remove_dir(self.mount_path())?;

        Ok(())
    }

    fn ensure_mounted(&self) -> Result<()> {
        Ok(())
    }

    fn ensure_unmounted(&self) -> Result<()> {
        Ok(())
    }

    fn open(&self) -> Result<()> {
        println!("opening stash, kupo!");

        // see a path, make a path
        self.ensure_mount_path_exists()?;

        // "hey, socko!"
        self.ensure_mounted()?;

        println!("- name: {}", self.name);
        println!("- block_device_name: {:?}", self.block_device_name);
        println!("- block_device_path: {:?}", self.block_device_path());
        println!("- mount_path: {:?}", self.mount_path());
        println!("- status: {}", self.status());

        Ok(())
    }

    fn status(&self) -> StashStatus {
        if self.mount_path().exists() {
            StashStatus::Open
        } else {
            StashStatus::Closed
        }
    }

    fn close(&self) -> Result<()> {
        println!("closing stash, kupo!");

        // "goodbye, socko!"
        self.ensure_unmounted()?;

        // see a path, kill a path
        self.ensure_mount_path_removed()?;

        println!("- name: {}", self.name);
        println!("- block_device_name: {:?}", self.block_device_name);
        println!("- block_device_path: {:?}", self.block_device_path());
        println!("- mount_path: {:?}", self.mount_path());
        println!("- status: {}", self.status());
        Ok(())
    }

    fn block_device_path(&self) -> PathBuf {
        PathBuf::from("/dev").join(&self.block_device_name)
    }

    fn mount_path(&self) -> PathBuf {
        PathBuf::from("/mnt").join(format!("sd-card_{}", self.name))
    }
}

fn stash(action: KupoStashAction) -> Result<()> {
    let stash = Stash::new("skate".into(), "sda1".into());

    match action {
        KupoStashAction::Open => stash.open()?,
        KupoStashAction::Close => stash.close()?,
    };

    Ok(())
}
