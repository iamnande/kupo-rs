mod cli;
mod config;
mod error;
mod stash;

fn main() {
    if let Err(err) = cli::run() {
        eprintln!("KupoError: {err}");
        std::process::exit(1);
    }
}
