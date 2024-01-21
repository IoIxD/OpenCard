use clap::Parser;
use hypertalk::Script;
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
            let mas_script = Script::parse(stack.script);
            println!(
                "{}",
                format!("{:?}", mas_script.commands).replace("),", "),\n")
            );
            for card in stack.cards {
                let script = Script::parse(card.script);
                println!("{}", format!("{:?}", script.commands).replace("),", "),\n"));
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    Ok(())
}
