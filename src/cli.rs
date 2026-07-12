use crate::config::{KupoConfig, load_config};
use crate::error::{KupoError, Result};
use crate::stash::Stash;

use std::env;

pub enum KupoCommand {
    Stash(KupoStashAction),
}

pub enum KupoStashAction {
    Open,
    Status,
    Close,
}

pub fn run() -> Result<()> {
    let command = parse_args()?;
    let config = load_config()?;

    match command {
        KupoCommand::Stash(action) => stash(config, action),
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
        "status" => Ok(KupoStashAction::Status),
        "close" => Ok(KupoStashAction::Close),
        _ => Err(KupoError::UnknownStashAction(action.to_owned())),
    }
}

fn stash(config: KupoConfig, action: KupoStashAction) -> Result<()> {
    let stash = Stash::new(config.stash.slug, config.stash.label);

    match action {
        KupoStashAction::Open => stash.open()?,
        KupoStashAction::Status => {
            println!("checking the status now, kupo!");
            stash.inspect()?;
        }
        KupoStashAction::Close => stash.close()?,
    };

    Ok(())
}
