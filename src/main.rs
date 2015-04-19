#[macro_use]
extern crate clap;

use clap::{App, SubCommand};

mod cli;

#[allow(dead_code)]
fn main() {
    let matches = App::new("corectl")
                        .about("commands for running applications on CoreOS")
                        // Pull version from Cargo.toml
                        .version(&format!("v{}", crate_version!())[..])
                        .subcommand(SubCommand::new("deploy")
                            .about("Commands for deploying CoreOS apps")
                            .arg_from_usage("<APPTAG> 'What to deploy (i.e. myorg/myapp:21d1f49)'"))
                        .subcommand(SubCommand::new("service")
                            .about("Commands for controling services")
                            .subcommand(SubCommand::new("add")
                                .about("Used for adding CoreOS apps")
                                .args_from_usage("<APP> 'The app to add'
                                                  <UNIT_FILE> 'The unit file with the configuration'"))
                            .subcommand(SubCommand::new("scale")
                                .about("Used to scale CoreOS app instances")
                                .args_from_usage("<APP> 'The app to scale'
                                                  <AMMOUNT> 'The ammount to scale (i.e. 4)'"))
                            .subcommand(SubCommand::new("remove")
                                .about("Used to remove CoreOS apps")
                                .arg_from_usage("<APP> 'The app to remove'")))
                        .get_matches();
    
    match cli::run(&matches) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    }
}
