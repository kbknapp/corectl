use std::env;
use std::ffi::AsOsStr;
use std::iter::IntoIterator;

use getopts;

pub fn run<T: IntoIterator + IteratorExt>(args: T) -> String
    where <T as Iterator>::Item: AsOsStr
{
    let options = getopts::Options::new();

    match options.parse(args.skip(1)) {
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
        let output = run(vec!["corectl".to_string()].iter());

        assert!(output.starts_with("Usage: corectl [options]\n\nOptions:"));
    }
}
