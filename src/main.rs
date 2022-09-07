use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Github Repo
    #[clap(value_parser)]
    name: String,
}

fn main() {
    let cli = Cli::parse();
    let name = cli.name;

    Command::new("git")
        .arg("clone")
        .arg(format!("git@github.com:{}.git", name))
        .output()
        .expect("clone command failed");
}
