use std::path::PathBuf;

use clap::Parser;

// https://github.com/clap-rs/clap/tree/v3.0.0/examples/tutorial_derive
// https://github.com/clap-rs/clap/blob/v3.0.0/examples/derive_ref/README.md#arg-attributes

#[derive(Parser, Debug)]
#[clap(name = "retry", about, version)]
pub struct Arguments {
    /// maximum number of script executions before giving up
    #[clap(name = "count", long)]
    pub count: Option<u64>,

    /// maximum duration in seconds before giving up
    #[clap(name = "duration", long)]
    pub duration_in_seconds: Option<u64>,

    /// send system notification on exit
    #[clap(long, takes_value = false)]
    pub notify: bool,

    /// delay between script executions in seconds
    #[clap(name = "delay", long)]
    pub delay_in_seconds: u64,

    /// path to the script to retry
    #[clap(name = "script")]
    pub script: PathBuf,
}

pub fn arguments() -> Arguments {
    Arguments::parse()
}

#[cfg(test)]
mod tests {
    use clap::ErrorKind;
    use clap::Parser;
    use spectral::prelude::*;

    use crate::Arguments;

    #[test]
    fn delay_and_script_are_mandatory_parameters() {
        let no_arguments: Vec<String> = vec![];
        let parsed: Result<Arguments, clap::Error> = Arguments::try_parse_from(no_arguments);

        assert!(parsed.is_err());
        let err = parsed.err().unwrap();

        assert_that!(err.kind).is_equal_to(ErrorKind::MissingRequiredArgument);
        assert_that!(err.info)
            .is_equal_to(vec![String::from("--delay <delay>"), String::from("<script>")]);
    }
}
