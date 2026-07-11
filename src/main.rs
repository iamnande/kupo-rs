use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, command, action] if command == "stash" => match parse_stash_cmd(action) {
            Ok(cmd) => stash(cmd),
            Err(err) => println!("{}", err),
        },
        _ => {
            println!("usage: kupo stash open|close")
        }
    }
}

fn parse_stash_cmd(action: &str) -> Result<StashAction, String> {
    match action {
        "open" => Ok(StashAction::Open),
        "close" => Ok(StashAction::Close),
        _ => Err(format!("unknown action: {}", action)),
    }
}

enum StashAction {
    Open,
    Close,
}

fn stash(action: StashAction) {
    match action {
        StashAction::Open => {
            println!("opening stash, kupo!");
        }
        StashAction::Close => {
            println!("closing stash, kupo!");
        }
    }
}
