use clap::Parser;
use std::{fs::File, io::Read, error::Error};

pub mod block;
pub mod woba;
pub mod byte;
pub mod data_layout;

use data_layout::StackDataLayout;

use crate::block::stack::Stack;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: String
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("{}",&args.path);
    match File::open(&args.path) {
        Ok(mut a) => {
            let len = a.metadata()?.len();
            let b: &mut Vec<u8> = &mut vec![0; len as usize];
            match File::read(&mut a, b) {
                Ok(a) => {
                    println!("{:#?}",Stack::from(&b));
                },
                Err(err) => {
                    println!("{}",err);
                }
            }

        },
        Err(err) => {
            println!("{}",err);
        }
    }
    Ok(())


}
