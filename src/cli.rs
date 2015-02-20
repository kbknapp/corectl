use std::env;

use getopts;

pub fn run(args: Vec<&str>) -> String {
    let options = getopts::Options::new();
    let matches = options.parse(args.tail());

    match options.parse(args.tail()) {
        Ok(matches) => {
            options.usage("Usage: corectl [options]")
        },
        Err(message) => {
            env::set_exit_status(1);
            format!("{}", message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn it_prints_help_with_no_args() {
        let output = run(vec!["corectl"]);

        assert!(output.starts_with("Usage: corectl [options]\n\nOptions:"));
    }
}
