#![feature(exit_status)]

extern crate clap;

use std::env;

use clap::{App, Arg, SubCommand};

mod cli;

#[allow(dead_code)]
fn main() {
    // pull the version from Cargo.toml
    let version = format!("{}.{}.{}{}",
                          env!("CARGO_PKG_VERSION_MAJOR"),
                          env!("CARGO_PKG_VERSION_MINOR"),
                          env!("CARGO_PKG_VERSION_PATCH"),
                          option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));
    
    let matches = App::new("corectl")
                        .about("commands for running applications on CoreOS")
                        .version(&version[..])
                        .subcommand(SubCommand::new("deploy")
                            .about("Commands for deploying CoreOS apps")
                            .arg(Arg::new("APPTAG")
                                .help("What to deploy (i.e. myorg/myapp:21d1f49)")
                                .index(1)
                                .required(true)))
                        .subcommand(SubCommand::new("service")
                            .about("Commands for controling services")
                            .subcommand(SubCommand::new("add")
                                .about("Used for adding CoreOS apps")
                                .arg(Arg::new("APP")
                                    .help("The app to add")
                                    .index(1)
                                    .required(true))
                                .arg(Arg::new("UNITFILE")
                                    .help("The unit file with the configuration")
                                    .required(true)
                                    .index(2)))
                            .subcommand(SubCommand::new("scale")
                                .about("Used to scale CoreOS app instances")
                                .arg(Arg::new("APP")
                                    .help("The app to scale")
                                    .index(1)
                                    .required(true))
                                .arg(Arg::new("AMMOUNT")
                                    .help("The ammount to scale")
                                    .required(true)
                                    .index(2)))
                            .subcommand(SubCommand::new("remove")
                                .about("Used to remove CoreOS apps")
                                .arg(Arg::new("APP")
                                    .help("The app to remove")
                                    .index(1)
                                    .required(true))))
                        .get_matches();
    
    match cli::run(&matches) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            env::set_exit_status(1);
            println!("{}", error);
        }
    }
}
