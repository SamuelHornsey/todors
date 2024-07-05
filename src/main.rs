use actions::{complete, delete};
use clap::{Parser, Subcommand};

use db::init_db;

mod todo;
mod db;
mod actions;

#[derive(Parser)]
#[derive(Debug)]
#[command(name="todors", author="Samuel Hornsey <me@samuelhornsey.com>", version="1.0", about="cli todo app")]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Add {
        value: String
    },
    Complete {
        value: String
    },
    Delete {
        value: String
    }
}

fn main() {
    println!(r#"
       __            __               
      / /_____  ____/ /___  __________
     / __/ __ \/ __  / __ \/ ___/ ___/
    / /_/ /_/ / /_/ / /_/ / /  (__  ) 
    \__/\____/\__,_/\____/_/  /____/ "#);

    println!("\n");

    // init the db
    let _ = init_db();

    let args = Args::parse();

    match &args.cmd {
        Commands::List => actions::list(),
        Commands::Add { value } => actions::add(value),
        Commands::Complete { value } => complete(value),
        Commands::Delete { value } => delete(value)
    }
}