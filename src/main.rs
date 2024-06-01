#![cfg_attr(
	all(coverage_nightly, test),
	feature(coverage_attribute)
)]

use std::ffi::OsString;
use std::io::BufRead;
use std::path::PathBuf;

use clap::ArgMatches;
use mabel::common::config::{color_choice_from_string, ApplicationMode, Config};
use mabel::common::error::Error;
use mabel::common::utils::{is_color_output_disabled, is_color_output_force_enabled, read_content_from_buf_reader};
use mabel::common::{ExitCode, Source, SourceOrigin};
use mabel::compiler::invoke_application_mode;
use mabel::compiler::session_globals::SessionGlobals;
use mabel::ternary;
use termcolor::{ColorChoice, StandardStream, WriteColor};

// Obviously, we can't test the main function as it's using
// std::env::args_os(), std::process::exit() and std::io::*
fn main()
{
	let exit_code = parse_and_run_cli(std::env::args_os());
	std::process::exit(exit_code.into());
}

/// Run the CLI.
/// This function is the entry point for the CLI.
/// It parses the command line arguments and runs the
/// appropriate subcommand.
///
/// # Returns
///
/// The exit code.
#[cfg_attr(coverage_nightly, coverage(off))]
fn parse_and_run_cli<A, T>(argv: T) -> ExitCode
where
	T: IntoIterator<Item = A>,
	A: Into<OsString> + Clone,
{
	use mabel::clap_utils::{handle_clap_error, new_clap_app};

	let app = new_clap_app();

	app
		.clone()
		.try_get_matches_from(argv)
		.map(|matches| {
			let color_choice = matches
				.clone()
				.get_one::<String>("color")
				.map(|v| v.to_owned())
				.map(|v| color_choice_from_string(v))
				.unwrap_or_else(|| ColorChoice::Auto);

			let mut stdin = std::io::stdin().lock();
			let (mut stdout, mut stderr) =
				create_io_streams(Some(color_choice));

			map_subcommand(
				matches,
				&mut stdin,
				&mut stdout,
				&mut stderr,
			)
		})
		.map_err(|e| {
			let (mut stdout, mut stderr) =
				create_io_streams(None);

			handle_clap_error(
				e,
				app.clone(),
				&mut stdout,
				&mut stderr,
			)
		})
		.unwrap_or_else(|e| e)
}

/// Create the standard input and output streams.
/// This function creates the standard input and output
/// streams based on the color choice.
///
/// # Arguments
///
/// * `color_choice` - The color choice.
///
/// # Returns
///
/// The standard input and output streams.
fn create_io_streams(
	color_choice: Option<ColorChoice>,
) -> (StandardStream, StandardStream)
{
	let color_choice = ternary!(
		is_color_output_disabled(),
		ColorChoice::Never,
		ternary!(
			is_color_output_force_enabled(),
			ColorChoice::Always,
			color_choice.unwrap_or_else(|| ColorChoice::Auto)
		)
	);
	(
		StandardStream::stdout(color_choice),
		StandardStream::stderr(color_choice),
	)
}

/// Map the subcommand to the appropriate function.
/// This function is called when a subcommand is provided
/// to the CLI.
// we can't test this function because we can't emulate to
// reach unreachable!() branch
#[cfg_attr(coverage_nightly, coverage(off))]
fn map_subcommand<I, O, E>(
	matches: ArgMatches,
	stdin: &mut I,
	stdout: &mut O,
	stderr: &mut E,
) -> ExitCode
where
	I: BufRead,
	O: WriteColor,
	E: WriteColor,
{
	match matches.subcommand()
	{
		Some(("run", sub_matches)) =>
		{
			run_compiler_or_interpreter(
				ApplicationMode::Interpreter,
				sub_matches.clone(),
				stdin,
				stdout,
				stderr,
			)
		}
		Some(("compile", sub_matches)) =>
		{
			run_compiler_or_interpreter(
				ApplicationMode::Compiler,
				sub_matches.clone(),
				stdin,
				stdout,
				stderr,
			)
		}
		Some(..) | None => unreachable!(),
	}
}

/// Invoke the compiler or interpreter.
/// This function is called when the subcommand is either
/// "run" or "compile".
///
/// # Arguments
///
/// * `mode` - The application mode.
/// * `matches` - The command line arguments.
/// * `stdin` - The input stream.
/// * `stdout` - The output stream.
/// * `stderr` - The error stream.
///
/// # Returns
///
/// The exit code.
fn run_compiler_or_interpreter<I, O, E>(
	mode: ApplicationMode,
	matches: ArgMatches,
	stdin: &mut I,
	stdout: &mut O,
	stderr: &mut E,
) -> ExitCode
where
	I: BufRead,
	O: WriteColor,
	E: WriteColor,
{
	let config =
		Config::from_arg_matches(matches.clone(), Some(mode));

	let mut session_globals = SessionGlobals::new(
		mode, config, stdin, stdout, stderr,
	);

	let source = create_source_from_arg_matches(
		matches.clone(),
		session_globals.stdin,
	);
	if source.is_err()
	{
		let error = source.unwrap_err();
		error.print(&mut session_globals);
		return error.exit_code;
	}

	invoke_application_mode(
		source.unwrap(),
		&mut session_globals,
	)
	.map_err(|error| {
		error.print(&mut session_globals);
		return error.exit_code;
	})
	.unwrap_or_else(|exit_code| exit_code)
}

fn create_source_from_stdin<I>(stdin: &mut I,
) -> Result<Source, Error>
where
	I: BufRead,
{
	// if the input is not provided or it is "-"
	read_content_from_buf_reader(stdin)
	.map(|code| {
		Source::from(code).with_origin(SourceOrigin::Stdin)
	})
	.map_err(|error: std::io::Error| {
		Error::new(
			mabel::common::error::ErrorKind::IOError(
				error,
				"stdin".to_string(),
			),
			ExitCode::IO_ERROR,
		)
	})
}

/// Create a source from the command line arguments.
/// The INPUT argument is used to create the source.
/// If the INPUT argument is not provided or it is "-",
/// the source is created from stdin.
/// If the INPUT argument is provided, the source is
/// created from the file.
///
/// # Arguments
///
/// * `matches` - The command line arguments.
///
/// # Returns
///
/// The source or an error.
fn create_source_from_arg_matches<I>(
	matches: ArgMatches,
	stdin: &mut I,
) -> Result<Source, Error>
where
	I: BufRead,
{
	let input = matches.get_one::<String>("INPUT");

	if input.is_some_and(|s| s != "-")
	{
		let path = PathBuf::from(input.unwrap());
		Source::try_from(path)
	}
	else
	{
		create_source_from_stdin(stdin)
	}
}
