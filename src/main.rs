use clap::Parser;
use hashstream::Args;

pub struct Hashes {
    hashes: Args,
}
fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}
