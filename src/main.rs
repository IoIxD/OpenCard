use clap::Parser;
use std::{error::Error, fs::File, io::Read, path::Path};

use hc_decode::{stack::Stack, Block};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("{}", &args.path);
    match Stack::from_path(&Path::new(&args.path)) {
        Ok(stack) => {
            println!("{}", stack.script.replace("\u{000D}", "\n"));
            for card in stack.cards {
                println!("{}", card.script.replace("\u{000D}", "\n"));
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    Ok(())
}
