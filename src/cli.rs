use clap::ArgMatches;

pub fn run(matches: &ArgMatches) -> Result<String, String> {
    match matches.subcommand() {
        ("deploy", Some(matches))  => deploy(matches),
        ("service", Some(matches)) => service(matches),
        ("", None) | _             => Err(format!("{}\nPlease re-run with --help for more information.", matches.usage())),
    }
}

fn deploy(matches: &ArgMatches) -> Result<String, String> {
    Ok(format!("Deployed: {}", matches.value_of("APPTAG").unwrap()))
}

fn service(matches: &ArgMatches) -> Result<String, String> {
    match matches.subcommand() {
        ("scale", Some(matches))  => service_scale(matches),
        ("remove", Some(matches)) => service_remove(matches),
        ("add", Some(matches))    => service_add(matches),
        ("", None) | _            => Err(format!("{}\nPlease re-run with --help for more information", matches.usage())),
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
