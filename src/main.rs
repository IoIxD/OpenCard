use clap::Parser;
use std::{error::Error, fs::File, io::Read};

use hc_decode::block::{general::Block, stack::Stack};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("{}", &args.path);
    match File::open(&args.path) {
        Ok(mut a) => {
            let len = a.metadata()?.len();
            let b: &mut Vec<u8> = &mut vec![0; len as usize];
            match File::read(&mut a, b) {
                Ok(_) => {
                    let stack = Stack::from(&b).await?;
                    println!("{}", stack.script.replace("\u{000D}", "\n"));
                    for card in stack.cards {
                        println!("{}", card.script.replace("\u{000D}", "\n"));
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    Ok(())
}
