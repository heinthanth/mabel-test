//! # Clap Utilities (clap_utils) module
//!
//! This module contains the utilities for the Clap library.
//! It provides the functions to run the CLI and handle the
//! subcommands.

use clap::error::{ContextKind, ErrorKind};
use clap::{crate_authors, crate_version, Arg, ArgAction, Command};
use termcolor::WriteColor;

use crate::common::config::CompilerBackend;
use crate::common::utils::is_color_output_disabled;
use crate::common::ExitCode;
use crate::{format_datetime, localdate, t};

/// Build the version string.
///
/// # Returns
///
/// The version string.
pub fn build_version_string() -> String
{
	let version =
		t!("cli-version.version", version = crate_version!());

	let copyright = t!(
		"cli-version.copyright",
		copyrightYear =
			format_datetime!(localdate!(2024), "%Y"),
		authors = crate_authors!()
	);

	format!(
		"{} {version}\n{copyright}",
		t!("cli-version.heading")
	)
}

fn generate_help_template(cmd: &Command) -> String
{
	let subcmd_piece = if cmd.get_subcommands().count() > 0
	{
		format!(
			"\n\n{}\n{{subcommands}}",
			t!("cli-help.subcommands-heading")
		)
	}
	else
	{
		"".into()
	};

	let option_piece = if cmd.get_opts().count() > 0
	{
		format!(
			"\n\n{}\n{{options}}",
			t!("cli-help.options-heading")
		)
	}
	else
	{
		"".into()
	};

	let positional_piece = if cmd.get_positionals().count()
		> 0
	{
		format!(
			"\n\n{}\n{{positionals}}",
			t!("cli-help.positionals-heading")
		)
	}
	else
	{
		"".into()
	};

	let about_piece = if cmd.get_about().is_some()
	{
		format!(
			"{}\n{{tab}}{}\n\n",
			t!("cli-help.about-heading"),
			cmd.get_about().unwrap()
		)
	}
	else
	{
		"".into()
	};

	let version = build_version_string();
	let usage_heading = t!("cli-help.usage-heading");

	format!(
		"\
{version}\n\n{about_piece}{usage_heading}
{{tab}}{{usage}}{subcmd_piece}{option_piece}{positional_piece}"
	)
}

/// Attach help template to a command.
///
/// # Arguments
///
/// * `cmd` - The command to attach the help template to.
///
/// # Returns
///
/// The command with the help template attached.
fn attach_help_template(cmd: &Command) -> Command
{
	let tmpl = generate_help_template(cmd);
	cmd.clone().help_template(tmpl)
}

/// Create `compile` subcommand for Clap
fn compile_subcommand() -> Command
{
	let cmd = Command::new("compile")
		.about(t!("cli-subcmd-compile-help.description"))
		.arg(
			Arg::new("INPUT")
				.help(t!("cli-subcmd-compile-help.arg-input"))
				.required(false),
		)
		.arg(
			Arg::new("output")
				.help(t!("cli-subcmd-compile-help.arg-output"))
				.short('o')
				.long("out")
				.action(ArgAction::Set)
				.required(false),
		)
		.arg(
			Arg::new("backend")
				.help(t!("cli-subcmd-compile-help.arg-backend"))
				.short('b')
				.long("backend")
				.value_parser(
					CompilerBackend::get_allowed_variants(),
				)
				.default_value("LLVM")
				.action(ArgAction::Set)
				.required(false),
		);

	attach_help_template(&cmd)
}

/// Create `run` subcommand for Clap
fn run_subcommand() -> Command
{
	let cmd = Command::new("run")
		.about(t!("cli-subcmd-run-help.description"))
		.arg(
			Arg::new("INPUT")
				.help(t!("cli-subcmd-run-help.arg-input"))
				.required(false),
		)
		.arg(
			Arg::new("no-jit")
				.help(t!("cli-subcmd-run-help.arg-no-jit"))
				.long("no-jit")
				.action(ArgAction::SetTrue)
				.required(false),
		);

	attach_help_template(&cmd)
}

pub fn new_clap_app() -> Command
{
	// Global help flag
	// This flag is used to display the help message.
	let global_help_flag = Arg::new("help")
		.help(t!("cli-help.arg-help"))
		.short('h')
		.long("help")
		.action(ArgAction::Help)
		.required(false)
		.global(true);

	// Global version flag
	// This flag is used to display the version message.
	let global_version_flag = Arg::new("version")
		.help(t!("cli-help.arg-version"))
		.short('v')
		.long("version")
		.action(ArgAction::Version)
		.required(false)
		.global(true);

	// Global debug flag
	// This flag is used to enable debug mode.
	// The flag can be used multiple times to increase the
	// debug level.
	//
	// The number of output messages increases with the debug
	// level.
	let global_debug_flag = Arg::new("debug")
		.help(t!("cli-help.arg-debug"))
		.short('d')
		.long("debug")
		.action(ArgAction::Count)
		.required(false)
		.global(true);

	// Global color flag
	// This flag is used to enable or disable color output.
	// The flag can be set to `auto`, `always`, or `never`.
	//
	// The default value is `auto`.
	// The app will try to use color when available.
	// If the flag is "always" and the terminal does not
	// support color, the app will try to print color using
	// ANSI escape codes.
	let global_color_flag = Arg::new("color")
		.help(t!("cli-help.arg-color"))
		.short('c')
		.long("color")
		.action(ArgAction::Set)
		.default_missing_value("auto")
		.default_value("auto")
		.required(false)
		.global(true)
		.value_parser(["auto", "always", "never"]);

	// Root command
	// This is the root command of the app.
	let root_cmd = Command::new("mabel")
		.version(crate_version!())
		.propagate_version(true)
		.disable_help_flag(true)
		.disable_help_subcommand(true)
		.disable_version_flag(true)
		.subcommand_required(true)
		.arg(global_debug_flag)
		.arg(global_help_flag)
		.arg(global_version_flag)
		.arg(global_color_flag)
		.subcommand(compile_subcommand())
		.subcommand(run_subcommand());

	attach_help_template(&root_cmd)
}

/// Handle an argument conflict error.
/// If arguments the same, meaning the same argument was
/// provided multiple times, Otherwise, print the
/// conflicting arguments.
///
/// # Arguments
///
/// * `error` - The clap error.
///
/// # Returns
///
/// The exit code.
fn handle_arg_conflict<W>(
	error: clap::Error,
	mut writer: W,
) -> ExitCode
where
	W: WriteColor,
{
	let invalid_arg =
		error.get(ContextKind::InvalidArg).unwrap().to_string();

	let err =
		t!("cli-error.arg-no-multiple-time", arg = invalid_arg);
	// eprintln!("{}", t!("utils-error.template", error =
	// err));
	writeln!(
		&mut writer,
		"{}",
		t!("utils-error.template", error = err)
	)
	.expect("failed to write to writer");
	ExitCode::INVALID_INPUT_OR_USAGE
}

/// Handle an invalid value error.
/// Print the invalid value and the valid choices.
///
/// # Arguments
///
/// * `error` - The clap error.
///
/// # Returns
///
/// The exit code.
fn handle_invalid_value<W>(
	error: clap::Error,
	mut writer: W,
) -> ExitCode
where
	W: WriteColor,
{
	let arg =
		error.get(ContextKind::InvalidArg).unwrap().to_string();
	let value = error
		.get(ContextKind::InvalidValue)
		.unwrap()
		.to_string();
	let choices =
		error.get(ContextKind::ValidValue).unwrap().to_string();

	let err = t!(
		"cli-error.invalid-value",
		arg = arg,
		value = value,
		choices = choices
	);
	// eprintln!("{}", t!("utils-error.template", error =
	// err));
	writeln!(
		&mut writer,
		"{}",
		t!("utils-error.template", error = err)
	)
	.expect("failed to write to writer");
	ExitCode::INVALID_INPUT_OR_USAGE
}

/// Handle an unknown argument error.
/// Print the unknown argument.
///
/// # Arguments
///
/// * `error` - The clap error.
///
/// # Returns
///
/// The exit code.
fn handle_unknown_argument<W>(
	error: clap::Error,
	mut writer: W,
) -> ExitCode
where
	W: WriteColor,
{
	let arg =
		error.get(ContextKind::InvalidArg).unwrap().to_string();

	let err =
		t!("cli-error.unexpected-argument", argument = arg);
	// eprintln!("{}", t!("utils-error.template", error =
	// err));
	writeln!(
		&mut writer,
		"{}",
		t!("utils-error.template", error = err)
	)
	.expect("failed to write to writer");
	ExitCode::INVALID_INPUT_OR_USAGE
}

/// Handle an unknown subcommand error.
/// Print the unknown subcommand.
///
/// # Arguments
///
/// * `error` - The clap error.
///
/// # Returns
///
/// The exit code.
fn handle_unknown_subcmd<W>(
	error: clap::Error,
	mut writer: W,
) -> ExitCode
where
	W: WriteColor,
{
	let subcmd = error
		.get(ContextKind::InvalidSubcommand)
		.unwrap()
		.to_string();
	let err = t!(
		"cli-error.unexpected-subcommand",
		subcommand = subcmd
	);
	// eprintln!("{}", t!("utils-error.template", error =
	// err));
	writeln!(
		&mut writer,
		"{}",
		t!("utils-error.template", error = err)
	)
	.expect("failed to write to writer");
	ExitCode::INVALID_INPUT_OR_USAGE
}

/// Handle a clap error.
///
/// # Arguments
///
/// * `error` - The clap error.
/// * `root_cmd` - The root command.
///
/// # Returns
///
/// The exit code.
pub fn handle_clap_error<OW, EW>(
	error: clap::Error,
	root_cmd: Command,
	stdout: &mut OW,
	stderr: &mut EW,
) -> ExitCode
where
	OW: WriteColor,
	EW: WriteColor,
{
	match error.kind()
	{
		ErrorKind::InvalidSubcommand =>
		{
			handle_unknown_subcmd(error, stderr)
		}
		ErrorKind::UnknownArgument =>
		{
			handle_unknown_argument(error, stderr)
		}
		ErrorKind::InvalidValue =>
		{
			handle_invalid_value(error, stderr)
		}
		ErrorKind::ArgumentConflict =>
		{
			handle_arg_conflict(error, stderr)
		}
		// print help message
		ErrorKind::DisplayHelp =>
		{
			if is_color_output_disabled()
			{
				writeln!(
					stdout,
					"{}",
					root_cmd.clone().render_help()
				)
				.expect("failed to write to stdout writer");
			}
			else
			{
				writeln!(
					stdout,
					"{}",
					root_cmd.clone().render_help().ansi()
				)
				.expect("failed to write to stdout writer");
			}
			ExitCode::SUCCESS
		}
		// print version message
		ErrorKind::DisplayVersion =>
		{
			// println!("{}", build_version_string());
			writeln!(stdout, "{}", build_version_string())
				.expect("failed to write to stdout writer");
			ExitCode::SUCCESS
		}
		// print help since no subcommand was provided
		ErrorKind::MissingSubcommand =>
		{
			// eprintln!("{}", root_cmd.clone().render_help());
			if is_color_output_disabled()
			{
				writeln!(
					stderr,
					"{}",
					root_cmd.clone().render_help()
				)
				.expect("failed to write to stdout writer");
			}
			else
			{
				writeln!(
					stderr,
					"{}",
					root_cmd.clone().render_help().ansi()
				)
				.expect("failed to write to stdout writer");
			}
			ExitCode::INVALID_INPUT_OR_USAGE
		}
		#[cfg(fuzzing)]
		_ =>
		{
			panic!("unreachable error kind: {:?}", error.kind())
		}
		#[cfg(not(fuzzing))]
		_ =>
		{
			let err = t!("cli-error.generic");
			// eprintln!(
			// 	"{}",
			// 	t!("utils-error.template", error = err)
			// );
			writeln!(
				stderr,
				"{}",
				t!("utils-error.template", error = err)
			)
			.expect("failed to write to stderr writer");
			ExitCode::INVALID_INPUT_OR_USAGE
		}
	}
}

#[cfg(test)]
mod tests
{
	use clap::error::ErrorKind;
	use clap::{crate_authors, crate_version, Command};
	use serial_test::serial;
	use termcolor::Buffer;

	/// Test the `build_version_string` function.
	/// It should return the version string of the app.
	#[serial(lang)]
	#[test]
	fn test_build_version_string()
	{
		use clap::crate_authors;
		std::env::set_var("LANG", "en-US.UTF-8");

		let version = super::build_version_string();
		assert_eq!(
			version,
			format!(
				"The Mabel Compiler v\u{2068}{}\u{2069}\n(c) \
				 \u{2068}2024\u{2069} \u{2068}{}\u{2069}",
				crate_version!(),
				crate_authors!()
			),
		);
	}

	#[test]
	fn test_generate_help_template()
	{
		let cmd = Command::new("test");
		let template = super::generate_help_template(&cmd);

		assert_eq!(
			template,
			format!(
				"{}\n\nusage:\n{{tab}}{{usage}}",
				super::build_version_string()
			)
		);
	}

	#[test]
	fn test_generate_help_template_with_positionals()
	{
		let cmd = Command::new("test")
			.arg(clap::Arg::new("arg").required(true));
		let template = super::generate_help_template(&cmd);

		#[rustfmt::skip]
		assert_eq!(
			template,
			format!(
				"{}\n\nusage:\n{{tab}}{{usage}}\n\npositionals:\n\
				{{positionals}}",
				super::build_version_string()
			)
		);
	}

	#[test]
	fn test_generate_help_template_with_options()
	{
		let cmd = Command::new("test")
			.arg(clap::Arg::new("opt").short('o'));
		let template = super::generate_help_template(&cmd);

		#[rustfmt::skip]
		assert_eq!(
			template,
			format!(
				"{}\n\nusage:\n{{tab}}{{usage}}\n\noptions:\n\
				{{options}}",
				super::build_version_string()
			)
		);
	}

	#[test]
	fn test_generate_help_template_with_subcommands()
	{
		let cmd = Command::new("test").subcommand(
			Command::new("subcmd").about("Subcommand"),
		);
		let template = super::generate_help_template(&cmd);

		#[rustfmt::skip]
		assert_eq!(
			template,
			format!(
				"{}\n\nusage:\n{{tab}}{{usage}}\n\nsubcommands:\n\
				{{subcommands}}",
				super::build_version_string()
			)
		);
	}

	#[test]
	fn test_generate_help_template_with_all()
	{
		let cmd = Command::new("test")
			.about("test")
			.arg(clap::Arg::new("opt").short('o'))
			.arg(clap::Arg::new("arg").required(true))
			.subcommand(
				Command::new("subcmd").about("Subcommand"),
			);
		let template = super::generate_help_template(&cmd);

		#[rustfmt::skip]
		assert_eq!(
			template,
			format!(
				"{}\n\nabout:\n{{tab}}test\n\n\
				usage:\n{{tab}}{{usage}}\
				\n\nsubcommands:\n{{subcommands}}\
				\n\noptions:\n{{options}}\
				\n\npositionals:\n{{positionals}}",
				super::build_version_string()
			)
		);
	}

	#[test]
	fn test_compile_subcommand()
	{
		let cmd = super::compile_subcommand();
		assert_eq!(cmd.get_name(), "compile");
	}

	#[test]
	fn test_run_subcommand()
	{
		let cmd = super::run_subcommand();
		assert_eq!(cmd.get_name(), "run");
	}

	#[test]
	fn test_new_clap_app()
	{
		let cmd = super::new_clap_app();
		assert_eq!(cmd.get_name(), "mabel");
	}

	#[serial(color)]
	#[test]
	fn test_handle_clap_error_missing_subcommand_no_color()
	{
		let cli = super::new_clap_app();
		let maybe_matches =
			cli.clone().try_get_matches_from(vec!["mabel"]);
		let error = maybe_matches.unwrap_err();

		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		std::env::remove_var("COLOR");
		std::env::set_var("NO_COLOR", "1");
		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(
			exit_code,
			crate::common::ExitCode::INVALID_INPUT_OR_USAGE
		);
		assert_eq!(
			String::from_utf8(stderr.into_inner()).unwrap(),
			cli.clone().render_help().to_string() + "\n"
		);
	}

	#[serial(color)]
	#[test]
	fn test_handle_clap_error_missing_subcommand_color()
	{
		let cli = super::new_clap_app();
		let maybe_matches =
			cli.clone().try_get_matches_from(vec!["mabel"]);
		let error = maybe_matches.unwrap_err();

		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		std::env::set_var("COLOR", "always");
		std::env::remove_var("NO_COLOR");
		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(
			exit_code,
			crate::common::ExitCode::INVALID_INPUT_OR_USAGE
		);
		assert_eq!(
			String::from_utf8(stderr.into_inner()).unwrap(),
			format!("{}\n", cli.clone().render_help().ansi())
		);
	}

	#[serial(color)]
	#[test]
	fn test_handle_clap_help_flag_no_color()
	{
		let cli = super::new_clap_app();
		let maybe_matches = cli
			.clone()
			.try_get_matches_from(vec!["mabel", "--help"]);
		let error = maybe_matches.unwrap_err();

		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		std::env::remove_var("COLOR");
		std::env::set_var("NO_COLOR", "1");
		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(exit_code, crate::common::ExitCode::SUCCESS);
		assert_eq!(
			String::from_utf8(stdout.into_inner()).unwrap(),
			cli.clone().render_help().to_string() + "\n"
		);
	}

	#[serial(color)]
	#[test]
	fn test_handle_clap_help_flag_color()
	{
		let cli = super::new_clap_app();
		let maybe_matches = cli
			.clone()
			.try_get_matches_from(vec!["mabel", "--help"]);
		let error = maybe_matches.unwrap_err();

		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		std::env::set_var("COLOR", "always");
		std::env::remove_var("NO_COLOR");
		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(exit_code, crate::common::ExitCode::SUCCESS);
		assert_eq!(
			String::from_utf8(stdout.into_inner()).unwrap(),
			format!("{}\n", cli.clone().render_help().ansi())
		);
	}

	#[test]
	fn test_handle_clap_version_flag()
	{
		let cli = super::new_clap_app();
		let maybe_matches = cli
			.clone()
			.try_get_matches_from(vec!["mabel", "--version"]);
		let error = maybe_matches.unwrap_err();

		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(exit_code, crate::common::ExitCode::SUCCESS);
		assert_eq!(
			String::from_utf8(stdout.into_inner()).unwrap(),
			format!(
				"The Mabel Compiler v\u{2068}{}\u{2069}\n(c) \
				 \u{2068}2024\u{2069} \u{2068}{}\u{2069}\n",
				crate_version!(),
				crate_authors!()
			),
		);
	}

	#[test]
	fn test_handle_clap_error_invalid_value()
	{
		let cli = super::new_clap_app();
		let maybe_matches =
			cli.clone().try_get_matches_from(vec![
				"mabel",
				"compile",
				"--backend",
				"invalid",
			]);
		let error = maybe_matches.unwrap_err();

		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(
			exit_code,
			crate::common::ExitCode::INVALID_INPUT_OR_USAGE
		);
		assert_eq!(
			String::from_utf8(stderr.into_inner()).unwrap(),
			format!(
				"error: \u{2068}Invalid value \u{2068}invalid\u{2069} for argument \
				\u{2068}--backend <backend>\u{2069} and should be one of: \u{2068}{}\u{2069}\u{2069}\n",
				crate::common::config::CompilerBackend::get_allowed_variants().join(", "),
			),
		);
	}

	#[test]
	fn test_handle_clap_error_argument_conflict()
	{
		let cli = super::new_clap_app();
		let maybe_matches =
			cli.clone().try_get_matches_from(vec![
				"mabel", "compile", "-o", "out", "-o", "out",
			]);
		let error = maybe_matches.unwrap_err();

		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(
			exit_code,
			crate::common::ExitCode::INVALID_INPUT_OR_USAGE
		);
		assert_eq!(
			String::from_utf8(stderr.into_inner()).unwrap(),
			"error: \u{2068}The argument \u{2068}--out \
			 <output>\u{2069} cannot be used multiple \
			 times\u{2069}\n",
		);
	}

	#[test]
	fn test_handle_clap_error_unknown_argument()
	{
		let cli = super::new_clap_app();
		let maybe_matches =
			cli.clone().try_get_matches_from(vec![
				"mabel",
				"compile",
				"--unknown",
			]);
		let error = maybe_matches.unwrap_err();

		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(
			exit_code,
			crate::common::ExitCode::INVALID_INPUT_OR_USAGE
		);
		assert_eq!(
			String::from_utf8(stderr.into_inner()).unwrap(),
			"error: \u{2068}Unexpected argument \
			 \u{2068}--unknown\u{2069}\u{2069}\n",
		);
	}

	#[test]
	fn test_handle_clap_error_unknown_subcommand()
	{
		let cli = super::new_clap_app();
		let maybe_matches = cli
			.clone()
			.try_get_matches_from(vec!["mabel", "unknown"]);
		let error = maybe_matches.unwrap_err();

		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(
			exit_code,
			crate::common::ExitCode::INVALID_INPUT_OR_USAGE
		);
		assert_eq!(
			String::from_utf8(stderr.into_inner()).unwrap(),
			"error: \u{2068}Unexpected subcommand \
			 \u{2068}unknown\u{2069}\u{2069}\n",
		);
	}

	#[test]
	fn test_handle_clap_error_unknown()
	{
		let cli = super::new_clap_app();
		let error = clap::Error::new(ErrorKind::Io);
		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();

		let exit_code = super::handle_clap_error(
			error,
			cli.clone(),
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(
			exit_code,
			crate::common::ExitCode::INVALID_INPUT_OR_USAGE
		);
		assert_eq!(
			String::from_utf8(stderr.into_inner()).unwrap(),
			"error: \u{2068}Something went wrong while parsing \
			 CLI arguments\u{2069}\n",
		);
	}
}
