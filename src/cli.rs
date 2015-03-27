use clap::ArgMatches;

pub fn run(matches: &ArgMatches) -> Result<String, String> {
    match matches.subcommand_name() {
        Some("deploy")  => deploy(matches.subcommand_matches("deploy").unwrap()),
        Some("service") => service(matches.subcommand_matches("service").unwrap()),
        _               => { Err(format!("Unknown command: Run `corectl` for help.")) }
    }
}

fn deploy(matches: &ArgMatches) -> Result<String, String> {
    Ok(format!("Deployed: {}", matches.value_of("APPTAG").unwrap()))
}

fn service(matches: &ArgMatches) -> Result<String, String> {
    match matches.subcommand_name() {
        Some("scale")  => service_scale(matches.subcommand_matches("scale").unwrap()),
        Some("remove") => service_remove(matches.subcommand_matches("remove").unwrap()),
        Some("add")    => service_add(matches.subcommand_matches("add").unwrap()),
        _              => Err(format!("Ambiguous command, please re-run with --help or help"))
    }
}

fn service_scale(matches: &ArgMatches) -> Result<String, String> {
    Ok(format!("Scaling {} to {} instances", matches.value_of("APP").unwrap(), matches.value_of("AMMOUNT").unwrap()))
}

fn service_remove(matches: &ArgMatches) -> Result<String, String> {
    Ok(format!("Removing {}", matches.value_of("APP").unwrap()))
}

fn service_add(matches: &ArgMatches) -> Result<String, String> {
    Ok(format!("Adding {} with unit file {}", matches.value_of("APP").unwrap(), matches.value_of("UNITFILE").unwrap()))
}

#[cfg(test)]
mod tests {

}
