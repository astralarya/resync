use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    command: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    // Push to configured remotes
    Push(PushCommand),

    // Pull from configured remotes
    Pull(PullCommand),

    // Clone another archive
    Clone(CloneCommand),
}

#[derive(Args, Debug)]
pub struct PushCommand {
    pub remote: Option<String>,
}

#[derive(Args, Debug)]
pub struct PullCommand {
    pub remote: Option<String>,
}

#[derive(Args, Debug)]
pub struct CloneCommand {
    pub repo: String,
}
