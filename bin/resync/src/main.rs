use clap::Parser;

mod args;

fn main() {
    let args = args::Cli::parse();
    println!("Args: {:?}", args);
}
