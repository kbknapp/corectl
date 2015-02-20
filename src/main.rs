#![feature(collections, env)]

extern crate getopts;

use std::env;

mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    let str_args: Vec<&str> = args.iter().map(|s| &s[]).collect();
    println!("{}", cli::run(str_args));
}
