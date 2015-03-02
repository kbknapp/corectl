use getopts;

const USAGE: &'static str = "Usage: corectl [OPTIONS] COMMAND [COMMAND OPTIONS]

Commands:
    deploy      Deploy fleet units
    help        Output this help message
    version     Output corectl's version number";

pub fn run(args: Vec<String>) -> Result<String, String> {
    let mut options = getopts::Options::new();
    options.optflag("h", "help", "Output this help message");
    options.optflag("v", "version", "Output corectl's version number");
    let matches = match options.parse(args.iter().skip(1)) {
        Ok(matches) => matches,
        Err(fail) => return Err(fail.to_err_msg())
    };
    let usage = Ok(options.usage(USAGE));

    if matches.opt_present("v") {
        return version();
    } else if matches.opt_present("h") {
        return usage;
    }

    if matches.free.len() == 0 {
        return usage;
    }

    let command = matches.free[0].as_slice();

    match command {
        "version" => version(),
        "deploy" => deploy(matches.free.clone()),
        "help" => usage,
        unknown => { Err(format!("Unknown command: {}. Run `corectl` for help.", unknown)) }
    }
}

fn deploy(args: Vec<String>) -> Result<String, String> {
    Ok("Deployed stuff.".to_string())
}

fn version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn it_prints_help_with_no_args() {
        let output = run(vec!["corectl".to_string()]);

        assert!(output.unwrap().starts_with("Usage: corectl [OPTIONS] COMMAND [COMMAND OPTIONS]\n\nOptions:"));
    }
}
