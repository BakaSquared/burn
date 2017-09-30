#[macro_use]
extern crate clap;
extern crate ansi_term;

use ansi_term::{Style, enable_ansi_support, Colour};

fn valid_verbosity(arg: String) -> Result<(), String> {
    if arg.contains("minimal") | arg.contains("full") | arg.contains("debug") {
        return Ok(());
    }
    Err(String::from("Invalid verbosity level specified. Must be either 'minimal', 'full', or 'debug'."))
}

fn main() {
    let cli_args = clap_app!(burn_app =>
        (version: "0.1.0")
        (author: "Carter Ford <bakasquared2@gmail.com>")
        (about: "Task executor for application development")
        (@arg VERBOSITY: -v --verbosity +takes_value {valid_verbosity} "Set the level of verbosity. Is one of 'minimal', 'full', or 'debug'. Defaults to 'minimal'.")
        (@arg KINDLING: -f --file +takes_value "Valid kindling to burn. If none is specified, this will default to './kindling'.")
        (@arg EXTINGUISHED: -e --extinguished "Burn the kindle in 'extinguished' mode - validate syntax, but don't actually run anything.")
    ).get_matches();

    let extinguished = cli_args.is_present("EXTINGUISHED"); //Check if this instance is extinguished.
    let verbosity = cli_args.value_of("VERBOSITY").unwrap_or("minimal"); //Check verbosity level, or default to minimal.
    let kindling = cli_args.value_of("kindling").unwrap_or("./kindling"); //Check specified file, or default to ./kindling

    //TODO: Implement actual functionality.
}