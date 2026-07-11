use std::{env, error::Error, fmt};

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

fn main() -> Result<()> {
    let command = parse_args()?;
    run(command)
}

fn run(command: KupoCommand) -> Result<()> {
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

fn stash(action: KupoStashAction) -> Result<()> {
    match action {
        KupoStashAction::Open => {
            println!("opening stash, kupo!")
        }
        KupoStashAction::Close => {
            println!("closing stash, kupo!")
        }
    }

    Ok(())
}
