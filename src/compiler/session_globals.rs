use std::collections::HashMap;
use std::io::BufRead;

use smol_str::SmolStr;
use termcolor::WriteColor;

use crate::common::config::{ApplicationMode, Config};
use crate::common::Source;

/// Session globals
/// It contains global variables that are used in the
/// session such as I/O, source maps, etc.
pub struct SessionGlobals<'i, I, O, E>
where
	I: BufRead,
	O: WriteColor,
	E: WriteColor,
{
	/// The input stream
	pub stdin: &'i mut I,
	/// The output stream
	pub stdout: &'i mut O,
	/// The error stream
	pub stderr: &'i mut E,
	/// The Source Id Map
	pub source_id_map: HashMap<SmolStr, Source>,
	/// Current application mode
	pub mode: ApplicationMode,
	/// The configuration
	pub config: Config,
}

/// Implementation of `SessionGlobals`.
impl<'i, I, O, E> SessionGlobals<'i, I, O, E>
where
	I: BufRead,
	O: WriteColor,
	E: WriteColor,
{
	/// Creates a new `SessionGlobals`.
	pub fn new(
		mode: ApplicationMode,
		config: Config,
		stdin: &'i mut I,
		stdout: &'i mut O,
		stderr: &'i mut E,
	) -> SessionGlobals<'i, I, O, E>
	{
		SessionGlobals {
			stdin,
			stdout,
			stderr,
			mode,
			source_id_map: HashMap::new(),
			config,
		}
	}
}

#[cfg(test)]
mod tests
{
	use termcolor::Buffer;

	use super::*;

	#[test]
	fn test_session_globals_new()
	{
		let mut stdin = std::io::empty();
		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();
		let config = Config::default();

		let session_globals = SessionGlobals::new(
			ApplicationMode::Interpreter,
			config,
			&mut stdin,
			&mut stdout,
			&mut stderr,
		);

		assert_eq!(
			session_globals.mode,
			ApplicationMode::Interpreter
		);
	}
}
