#![cfg(not(feature = "nobin"))]

mod args;
use args::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}
