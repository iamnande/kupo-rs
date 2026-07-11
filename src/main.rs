use std::env;

enum KupoCommand {
    Stash(KupoStashAction),
}

enum KupoStashAction {
    Open,
    Close,
}

fn main() {
    match parse_args() {
        Ok(command) => run(command),
        Err(err) => eprintln!("{}", err),
    }
}

fn parse_args() -> Result<KupoCommand, String> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, command, action] if command == "stash" => {
            Ok(KupoCommand::Stash(parse_stash_action(action)?))
        }
        _ => Err("usage: kupo stash open/close".into()),
    }
}

fn parse_stash_action(action: &str) -> Result<KupoStashAction, String> {
    match action {
        "open" => Ok(KupoStashAction::Open),
        "close" => Ok(KupoStashAction::Close),
        _ => Err(format!("unknown stash action: {}", action)),
    }
}

fn run(command: KupoCommand) {
    match command {
        KupoCommand::Stash(action) => stash(action),
    }
}

fn stash(action: KupoStashAction) {
    match action {
        KupoStashAction::Open => {
            println!("opening stash, kupo!")
        }
        KupoStashAction::Close => {
            println!("closing stash, kupo!")
        }
    }
}
