#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use std::ffi::OsString;
use std::io::{BufRead, Write};
use std::path::PathBuf;

use clap::ArgMatches;
use mabel::common::config::{CompilerModeConfig, InterpreterModeConfig};
use mabel::common::error::Error;
use mabel::common::utils::read_content_from_buf_reader;
use mabel::common::{ExitCode, Source, SourceOrigin};

// Obviously, we can't test the main function as it's using
// std::env::args_os(), std::process::exit() and std::io::*
#[cfg(not(test))]
fn main()
{
	let exit_code = parse_and_run_cli(
		std::env::args_os(),
		std::io::stdin().lock(),
		std::io::stdout().lock(),
		std::io::stderr().lock(),
	);
	std::process::exit(exit_code.into());
}

/// Map the subcommand to the appropriate function.
/// This function is called when a subcommand is provided
/// to the CLI.
// we can't test this function because we can't emulate to
// reach unreachable!() branch
#[cfg_attr(coverage_nightly, coverage(off))]
fn map_subcommand<I, O, E>(
	matches: ArgMatches,
	stdin: I,
	mut stdout: O,
	mut stderr: E,
) -> Result<ExitCode, Error>
where
	I: BufRead,
	O: Write,
	E: Write,
{
	match matches.subcommand()
	{
		Some(("run", sub_matches)) => run_subcmd(
			sub_matches.clone(),
			stdin,
			&mut stdout,
			&mut stderr,
		),
		Some(("compile", sub_matches)) => compile_subcmd(
			sub_matches.clone(),
			stdin,
			&mut stdout,
			&mut stderr,
		),
		Some(..) | None => unreachable!(),
	}
}

/// Run the CLI.
/// This function is the entry point for the CLI.
/// It parses the command line arguments and runs the
/// appropriate subcommand.
///
/// # Returns
///
/// The exit code.
fn parse_and_run_cli<A, T, I, O, E>(
	argv: T,
	stdin: I,
	mut stdout: O,
	mut stderr: E,
) -> ExitCode
where
	T: IntoIterator<Item = A>,
	A: Into<OsString> + Clone,
	I: BufRead,
	O: Write,
	E: Write,
{
	use mabel::clap_utils::{handle_clap_error, new_clap_app};

	let app = new_clap_app();

	app
		.clone()
		.try_get_matches_from(argv)
		.map(|matches| {
			map_subcommand(
				matches,
				stdin,
				&mut stdout,
				&mut stderr,
			)
			.map_err(|e| {
				e.print(&mut stderr);
				return e.exit_code;
			})
			.unwrap_or_else(|e| e)
		})
		.map_err(|e| {
			handle_clap_error(
				e,
				app.clone(),
				&mut stdout,
				&mut stderr,
			)
		})
		.unwrap_or_else(|e| e)
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
	stdin: I,
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
		// if the input is not provided or it is "-"
		let content =
			read_content_from_buf_reader(stdin).map(|code| {
				Source::from(code).with_origin(SourceOrigin::Stdin)
			});

		#[cfg(test)]
		return Ok(content.unwrap());

		// we can't really test that IO error because we're
		// emulating stdin
		#[cfg(not(test))]
		return content.map_err(|error: std::io::Error| {
			Error::new(
				mabel::common::error::ErrorKind::IOError(
					error,
					"stdin".to_string(),
				),
				ExitCode::IO_ERROR,
			)
		});
	}
}

/// Handle the `run` subcommand.
/// This function is called when the `run` subcommand is
/// used.
///
/// # Arguments
///
/// * `matches` - The command line arguments.
///
/// # Returns
///
/// The exit code or an error.
fn run_subcmd<I, O, E>(
	matches: ArgMatches,
	stdin: I,
	mut stdout: O,
	mut _stderr: E,
) -> Result<ExitCode, Error>
where
	I: BufRead,
	O: Write,
	E: Write,
{
	let source =
		create_source_from_arg_matches(matches.clone(), stdin)?;
	let _config =
		InterpreterModeConfig::from(matches.clone());

	writeln!(&mut stdout, "{}", source.code)
		.expect("failed to write to stdout writer");
	Ok(ExitCode::SUCCESS)
}

/// Handle the `compile` subcommand.
/// This function is called when the `compile` subcommand is
/// used.
///
/// # Arguments
///
/// * `matches` - The command line arguments.
///
/// # Returns
///
/// The exit code or an error.
fn compile_subcmd<I, O, E>(
	matches: ArgMatches,
	stdin: I,
	mut stdout: O,
	mut _stderr: E,
) -> Result<ExitCode, Error>
where
	I: BufRead,
	O: Write,
	E: Write,
{
	let source =
		create_source_from_arg_matches(matches.clone(), stdin)?;
	let _config = CompilerModeConfig::from(matches.clone());

	writeln!(&mut stdout, "{}", source.code)
		.expect("failed to write to stdout writer");
	Ok(ExitCode::SUCCESS)
}

#[cfg(test)]
mod tests
{
	use std::io::Cursor;

	use mabel::common::ExitCode;

	#[test]
	fn test_compile_cmd_no_input()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			["mabel", "compile"],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::SUCCESS);
	}

	#[test]
	fn test_compile_cmd_clap_error()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			["mabel", "compile", "--invalid"],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::INVALID_INPUT_OR_USAGE);
	}

	#[test]
	fn test_compile_cmd_file_input()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			[
				"mabel",
				"compile",
				concat!(
					env!("CARGO_MANIFEST_DIR"),
					"/tests/scripts/test.mbl"
				),
			],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::SUCCESS);
	}

	#[test]
	fn test_compile_cmd_hyphen_input()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			["mabel", "compile", "-"],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::SUCCESS);
	}

	#[test]
	fn test_compile_cmd_io_error()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			[
				"mabel",
				"compile",
				concat!(
					env!("CARGO_MANIFEST_DIR"),
					"/tests/scripts/invalid.mbl"
				),
			],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::IO_ERROR);
	}

	#[test]
	fn test_run_cmd_no_input()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			["mabel", "run"],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::SUCCESS);
	}

	#[test]
	fn test_run_cmd_clap_error()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			["mabel", "run", "--invalid"],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::INVALID_INPUT_OR_USAGE);
	}

	#[test]
	fn test_run_cmd_file_input()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			[
				"mabel",
				"run",
				concat!(
					env!("CARGO_MANIFEST_DIR"),
					"/tests/scripts/test.mbl"
				),
			],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::SUCCESS);
	}

	#[test]
	fn test_run_cmd_hyphen_input()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			["mabel", "run", "-"],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::SUCCESS);
	}

	#[test]
	fn test_run_cmd_io_error()
	{
		let stdin = Cursor::new(Vec::new());
		let mut stdout = Vec::new();
		let mut stderr = Vec::new();

		let result = super::parse_and_run_cli(
			[
				"mabel",
				"run",
				concat!(
					env!("CARGO_MANIFEST_DIR"),
					"/tests/scripts/invalid.mbl"
				),
			],
			stdin,
			&mut stdout,
			&mut stderr,
		);
		assert_eq!(result, ExitCode::IO_ERROR);
	}
}
