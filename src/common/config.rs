//! # Configuration (config) module
//!
//! The `config` module provides configuration types and
//! functions used in application's mode such as compiler
//! mode, interpreter mode

use clap::ArgMatches;
use termcolor::ColorChoice;

/// Configuration enum for the program.
#[derive(Debug, Clone)]
pub struct Config
{
	/// Global configuration of the program.
	pub global_config: GlobalConfig,
	/// Inner configuration of the program.
	pub inner: Option<InnerConfig>,
}

/// Available Application modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationMode
{
	/// Run the program in interpreter mode.
	Interpreter,
	/// Run the program in compiler mode.
	Compiler,
}

/// Configuration inner of the program.
#[derive(Debug, Clone)]
pub enum InnerConfig
{
	/// Run the program in interpreter mode.
	Interpreter(InterpreterModeConfig),
	/// Run the program in compiler mode.
	Compiler(CompilerModeConfig),
}

/// `Default` implementation for `Config`.
impl Default for Config
{
	/// Create a new `Config` with default values.
	fn default() -> Self
	{
		Config {
			global_config: Default::default(),
			inner: None,
		}
	}
}

/// Implementation of `Config`.
impl Config
{
	/// Create a new `Config` instance.
	///
	/// # Arguments
	///
	/// * `global_config` - The global configuration of the
	///   program.
	/// * `inner` - The inner configuration of the program.
	///
	/// # Returns
	///
	/// The `Config` instance.
	pub fn new(
		global_config: GlobalConfig,
		inner: InnerConfig,
	) -> Self
	{
		Config {
			global_config,
			inner: Some(inner),
		}
	}

	/// Create a new `Config` from `ArgMatches`.
	///
	/// # Arguments
	///
	/// * `matches` - The `ArgMatches` to convert.
	///
	/// # Returns
	///
	/// The `Config` instance.
	pub fn from_arg_matches(
		matches: ArgMatches,
		subcommand: Option<ApplicationMode>,
	) -> Config
	{
		Config {
			global_config: GlobalConfig::from(matches.clone()),
			inner: Config::inner_config_from_arg_matches(
				matches, subcommand,
			),
		}
	}

	/// Create inner configuration from the `Config`.
	/// If the subcommand is not recognized, it will return
	/// `None`.
	///
	/// # Arguments
	///
	/// * `matches` - The `ArgMatches` to convert.
	///
	/// # Returns
	///
	/// The inner configuration of the program.
	fn inner_config_from_arg_matches(
		matches: ArgMatches,
		mode: Option<ApplicationMode>,
	) -> Option<InnerConfig>
	{
		match mode
		{
			Some(ApplicationMode::Interpreter) =>
			{
				Some(InnerConfig::Interpreter(
					InterpreterModeConfig::from(matches),
				))
			}
			Some(ApplicationMode::Compiler) =>
			{
				Some(InnerConfig::Compiler(
					CompilerModeConfig::from(matches),
				))
			}
			_ => None,
		}
	}
}

/// Configuration of the program's global settings.
#[derive(Debug, Clone)]
pub struct GlobalConfig
{
	/// The debug level of the program.
	pub debug_level: u8,
	/// The color output control
	pub color_choice: ColorChoice,
}

/// `Default` implementation for `GlobalConfig`.
impl Default for GlobalConfig
{
	/// Create a new `GlobalConfig` with default values.
	fn default() -> Self
	{
		GlobalConfig {
			debug_level: 0,
			color_choice: ColorChoice::Auto,
		}
	}
}

/// Convert a string into a `ColorChoice`.
///
/// # Arguments
///
/// * `value` - The string to convert.
///
/// # Returns
///
/// The `ColorChoice` value.
pub fn color_choice_from_string(
	value: String,
) -> ColorChoice
{
	match value.to_lowercase().as_str()
	{
		"always" => ColorChoice::Always,
		"never" => ColorChoice::Never,
		_ => ColorChoice::Auto,
	}
}

/// `From<ArgMatches>` implementation for `GlobalConfig`.
impl From<ArgMatches> for GlobalConfig
{
	/// Convert the `ArgMatches` into a `GlobalConfig`.
	fn from(matches: ArgMatches) -> Self
	{
		GlobalConfig {
			debug_level: matches.get_count("debug"),
			color_choice: matches
				.get_one::<String>("color")
				.map(|v| v.to_owned())
				.map(|v| color_choice_from_string(v))
				.unwrap_or_else(|| ColorChoice::Auto),
		}
	}
}

/// Configuration of the program's interpreter mode.
#[derive(Debug, Clone)]
pub struct InterpreterModeConfig
{
	/// Run the program using JIT execution engine.
	pub jit_enabled: bool,
}

/// `Default` implementation for `InterpreterModeConfig`.
impl Default for InterpreterModeConfig
{
	/// Create a new `InterpreterModeConfig` with default
	/// values.
	fn default() -> Self
	{
		InterpreterModeConfig { jit_enabled: true }
	}
}

/// `From<ArgMatches>` implementation for
/// `InterpreterModeConfig`.
impl From<ArgMatches> for InterpreterModeConfig
{
	/// Convert the `ArgMatches` into a
	/// `InterpreterModeConfig`.
	fn from(matches: ArgMatches) -> Self
	{
		InterpreterModeConfig {
			jit_enabled: !matches.get_flag("no-jit"),
		}
	}
}

/// The backend of the compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilerBackend
{
	/// Compile the program using LLVM.
	LLVM,
	/// Transpile the program to C.
	C,
	/// Transpile the program to JavaScript.
	JavaScript,
}

/// Implementation of `CompilerBackend`.
impl CompilerBackend
{
	/// Get the name of the backend.
	pub fn get_allowed_variants() -> [&'static str; 3]
	{
		["LLVM", "C", "JavaScript"]
	}
}

/// `From<String>` implementation for `CompilerBackend`.
impl From<String> for CompilerBackend
{
	/// Convert a string into a `CompilerBackend`.
	fn from(value: String) -> Self
	{
		match value.to_lowercase().as_str()
		{
			"llvm" => CompilerBackend::LLVM,
			"c" => CompilerBackend::C,
			"javascript" => CompilerBackend::JavaScript,
			_ => CompilerBackend::LLVM,
		}
	}
}

/// Configuration of the program's compiler mode.
#[derive(Debug, Clone)]
pub struct CompilerModeConfig
{
	/// The output file of the compiler.
	pub output_file: Option<String>,
	/// The backend of the compiler.
	pub backend: CompilerBackend,
}

/// `Default` implementation for `CompilerModeConfig`.
impl Default for CompilerModeConfig
{
	/// Create a new `CompilerModeConfig` with default
	/// values.
	fn default() -> Self
	{
		CompilerModeConfig {
			output_file: None,
			backend: CompilerBackend::LLVM,
		}
	}
}

/// `From<ArgMatches>` implementation for
/// `CompilerModeConfig`.
impl From<ArgMatches> for CompilerModeConfig
{
	/// Convert the `ArgMatches` into a
	/// `CompilerModeConfig`.
	fn from(matches: ArgMatches) -> Self
	{
		CompilerModeConfig {
			output_file: matches
				.get_one::<String>("output")
				.map(|v| v.to_owned()),
			backend: matches
				.get_one::<String>("backend")
				.map(|v| v.to_owned())
				.map(|v| CompilerBackend::from(v))
				.unwrap_or_else(|| CompilerBackend::LLVM),
		}
	}
}

#[cfg(test)]
mod tests
{
	use clap::{Arg, ArgAction, Command};
	#[cfg(coverage)]
	use coverage_helper::test;
	use termcolor::ColorChoice;

	use crate::common::config::{
		ApplicationMode,
		Config,
		InnerConfig,
	};
	use crate::get_enum_variant;

	#[test]
	fn test_color_choice_from_string()
	{
		use termcolor::ColorChoice;

		use super::color_choice_from_string;

		assert_eq!(
			color_choice_from_string("always".to_owned()),
			ColorChoice::Always
		);
		assert_eq!(
			color_choice_from_string("never".to_owned()),
			ColorChoice::Never
		);
		assert_eq!(
			color_choice_from_string("auto".to_owned()),
			ColorChoice::Auto
		);
		assert_eq!(
			color_choice_from_string("".to_owned()),
			ColorChoice::Auto
		);
		assert_eq!(
			color_choice_from_string("invalid".to_owned()),
			ColorChoice::Auto
		);
		assert_eq!(
			color_choice_from_string("ALWAYS".to_owned()),
			ColorChoice::Always
		);
	}

	#[test]
	fn test_default_config()
	{
		use termcolor::ColorChoice;

		use super::Config;

		let config = Config::default();

		assert_eq!(config.global_config.debug_level, 0);
		assert_eq!(
			config.global_config.color_choice,
			ColorChoice::Auto
		);
		assert!(config.inner.is_none());
	}

	#[test]
	fn test_config_new()
	{
		use termcolor::ColorChoice;

		use super::{Config, GlobalConfig, InnerConfig};

		let global_config = GlobalConfig::default();
		let inner =
			InnerConfig::Interpreter(Default::default());
		let config = Config::new(global_config, inner);

		assert_eq!(config.global_config.debug_level, 0);
		assert_eq!(
			config.global_config.color_choice,
			ColorChoice::Auto
		);
	}

	#[test]
	fn test_global_config_from_arg_matches()
	{
		use termcolor::ColorChoice;

		use super::GlobalConfig;

		let command = Command::new("test")
			.arg(
				Arg::new("debug")
					.action(ArgAction::Count)
					.short('d')
					.long("debug"),
			)
			.arg(
				Arg::new("color")
					.short('c')
					.long("color")
					.action(ArgAction::Set),
			);

		let matches = command
			.get_matches_from(vec!["test", "-d", "-c", "always"]);
		let config = GlobalConfig::from(matches);

		assert_eq!(config.debug_level, 1);
		assert_eq!(config.color_choice, ColorChoice::Always);
	}

	#[test]
	fn test_config_from_arg_matches_with_invalid_subcmd()
	{
		let command = Command::new("test")
			.arg(
				Arg::new("debug")
					.action(ArgAction::Count)
					.short('d')
					.long("debug"),
			)
			.arg(
				Arg::new("color")
					.short('c')
					.long("color")
					.action(ArgAction::Set),
			);

		let matches = command
			.clone()
			.get_matches_from(vec!["test", "-d", "-c", "always"]);
		let config = Config::from_arg_matches(matches, None);

		assert_eq!(config.global_config.debug_level, 1);
		assert_eq!(
			config.global_config.color_choice,
			ColorChoice::Always
		);
		assert!(config.inner.is_none());
	}

	#[test]
	fn test_compiler_mode_config_from_arg_matches()
	{
		use super::{CompilerBackend, CompilerModeConfig};

		let command = Command::new("test")
			.arg(
				Arg::new("debug")
					.action(ArgAction::Count)
					.short('d')
					.long("debug"),
			)
			.arg(
				Arg::new("color")
					.short('c')
					.long("color")
					.action(ArgAction::Set),
			)
			.arg(
				Arg::new("output")
					.short('o')
					.long("output")
					.action(ArgAction::Set),
			)
			.arg(
				Arg::new("backend")
					.short('b')
					.long("backend")
					.action(ArgAction::Set),
			);

		let matches = command.clone().get_matches_from(vec![
			"test", "-d", "-o", "output", "-b", "llvm",
		]);
		let config = Config::from_arg_matches(
			matches,
			Some(ApplicationMode::Compiler),
		);

		assert_eq!(config.global_config.debug_level, 1);
		assert_eq!(
			config.global_config.color_choice,
			Default::default()
		);

		let inner = get_enum_variant!(
			config.inner.unwrap(),
			InnerConfig::Compiler(data),
			data
		)
		.unwrap();
		assert_eq!(
			inner.output_file,
			Some("output".to_owned())
		);
		assert_eq!(inner.backend, CompilerBackend::LLVM);

		let matches = command.clone().get_matches_from(vec![
			"test", "-d", "-c", "always", "-o", "output",
		]);
		let config = CompilerModeConfig::from(matches);
		assert_eq!(config.backend, CompilerBackend::LLVM);
	}

	#[test]
	fn test_interpreter_mode_config_from_arg_matches()
	{
		let command = Command::new("test")
			.arg(
				Arg::new("debug")
					.action(ArgAction::Count)
					.short('d')
					.long("debug"),
			)
			.arg(
				Arg::new("color")
					.short('c')
					.long("color")
					.action(ArgAction::Set),
			)
			.arg(
				Arg::new("no-jit")
					.long("no-jit")
					.action(ArgAction::SetTrue),
			);

		let matches = command
			.clone()
			.get_matches_from(vec!["test", "-d", "--no-jit"]);
		let config = Config::from_arg_matches(
			matches,
			Some(ApplicationMode::Interpreter),
		);

		assert_eq!(config.global_config.debug_level, 1);
		assert_eq!(
			config.global_config.color_choice,
			Default::default()
		);

		let inner = get_enum_variant!(
			config.inner.unwrap(),
			InnerConfig::Interpreter(data),
			data
		)
		.unwrap();
		assert_eq!(inner.jit_enabled, false);
	}

	#[test]
	fn test_compiler_backend_from_string()
	{
		use super::CompilerBackend;

		assert_eq!(
			CompilerBackend::from("llvm".to_owned()),
			CompilerBackend::LLVM
		);
		assert_eq!(
			CompilerBackend::from("c".to_owned()),
			CompilerBackend::C
		);
		assert_eq!(
			CompilerBackend::from("javascript".to_owned()),
			CompilerBackend::JavaScript
		);
		assert_eq!(
			CompilerBackend::from("invalid".to_owned()),
			CompilerBackend::LLVM
		);
	}

	#[test]
	fn test_compiler_backend_get_allowed_variants()
	{
		use super::CompilerBackend;

		assert_eq!(
			CompilerBackend::get_allowed_variants(),
			["LLVM", "C", "JavaScript"]
		);
	}

	#[test]
	fn test_global_config_default()
	{
		use termcolor::ColorChoice;

		use super::GlobalConfig;

		let config = GlobalConfig::default();

		assert_eq!(config.debug_level, 0);
		assert_eq!(config.color_choice, ColorChoice::Auto);
	}

	#[test]
	fn test_compiler_mode_config_default()
	{
		use super::{CompilerBackend, CompilerModeConfig};

		let config = CompilerModeConfig::default();

		assert_eq!(config.output_file, None);
		assert_eq!(config.backend, CompilerBackend::LLVM);
	}

	#[test]
	fn test_interpreter_mode_config_default()
	{
		use super::InterpreterModeConfig;

		let config = InterpreterModeConfig::default();

		assert_eq!(config.jit_enabled, true);
	}
}
