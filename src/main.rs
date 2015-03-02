#![feature(core, exit_status)]

extern crate getopts;

use std::env;

mod cli;

#[allow(dead_code)]
fn main() {
    let args = env::args().collect();

    match cli::run(args) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            env::set_exit_status(1);
            println!("{}", error);
        }
    }
}
