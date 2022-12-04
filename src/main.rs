use clap::{Args, Parser, Subcommand};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Parser)]
#[command(name = "Advent of Code")]
#[command(author = "Peter <peterwj@cs.stanford.edu>")]
#[command(version = "1.0")]
#[command(about = "Solves advent of code problems", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    P1(P1),
}

#[derive(Args)]
struct P1 {
    #[arg(short, long)]
    input: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::P1(args) => {
            let path = &args.input;
            let f = File::open(path)?;
            let reader = BufReader::new(f);

            for line in reader.lines() {
                println!("{}", line?);
            }
        }
    }
    Ok(())
}
