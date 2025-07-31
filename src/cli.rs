use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    pub input: String,
}
