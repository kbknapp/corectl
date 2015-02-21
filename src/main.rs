#![feature(env, std_misc)]

extern crate getopts;

use std::env;

mod cli;

#[allow(dead_code)]
fn main() {
    let args: env::Args = env::args();

    println!("{}", cli::run(args));
}
