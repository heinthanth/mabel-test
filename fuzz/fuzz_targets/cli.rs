#![no_main]

use clap::{ArgMatches, Command};
use libfuzzer_sys::fuzz_target;
use mabel::clap_utils;
use termcolor::{ColorChoice, StandardStream};

/// Fuzz testing for CLI subcommand error handling
/// Mabel tried to cover all possible errors that could be
/// returned from clap and handle them accordingly.
/// But in production, generic error message will be shown.
/// But in fuzzing mode, it will panic.
///
/// # Arguments
///
/// * `matches` - The command line arguments.
/// * `app` - The root command.
fn handle_matches(
	matches: Result<ArgMatches, clap::Error>,
	app: Command,
)
{
	matches
		.map_err(|error| {
			clap_utils::handle_clap_error(
				error,
				app,
				&mut StandardStream::stdout(ColorChoice::Auto),
				&mut StandardStream::stderr(ColorChoice::Auto),
			)
		})
		.map_err(|_| ())
		.map(|_| ())
		.unwrap_or_else(|_| ())
}

// Fuzz testing for CLI error handling
// Provides a vector of strings to the CLI parser.
fuzz_target!(|data: Vec<String>| {
	let mut args = vec!["mabel".to_string()];
	args.extend(data);

	let app = clap_utils::new_clap_app();
	let matches = app.clone().try_get_matches_from(args);

	handle_matches(matches, app);
});
