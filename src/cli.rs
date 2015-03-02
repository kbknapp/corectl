use getopts;

pub fn run(args: Vec<String>) -> Result<String, String> {
    let options = getopts::Options::new();
    let matches = match options.parse(args.iter().skip(1)) {
        Ok(matches) => matches,
        Err(fail) => return Err(fail.to_err_msg())
    };
    let usage = Ok(options.usage("Usage: corectl COMMAND [OPTIONS]"));

    if matches.free.len() == 0 {
        return usage;
    }

    let command = matches.free[0].as_slice();

    match command {
        "help" => usage,
        unknown => { Err(format!("Unknown command: {}. Run `corectl help` for help.", unknown)) }
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn it_prints_help_with_no_args() {
        let output = run(vec!["corectl".to_string()]);

        assert!(output.unwrap().starts_with("Usage: corectl COMMAND [OPTIONS]\n\nOptions:"));
    }
}
