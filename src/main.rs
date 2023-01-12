use clap::Parser;
use std::fs::File;

pub mod block;
pub mod parse;
pub mod woba;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: String
}
fn main() {
    let args = Args::parse();
    match File::open(args.path) {
        Ok(a) => {
            //
        },
        Err(err) => {
            println!("{}",err);
            return;
        }
    }

}
