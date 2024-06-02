//! # Error (error) module
//!
//! This module contains the error handling logic for the
//! application. It provides possible error kinds and the
//! error struct.

use std::io::BufRead;

use termcolor::WriteColor;

use crate::common::ExitCode;
use crate::compiler::session_globals::SessionGlobals;
use crate::parser::lexer::LexerError;
use crate::parser::ParserError;
use crate::t;

/// Convert an I/O error into a string.
///
/// # Arguments
///
/// * `error` - The I/O error.
/// * `path` - The path that caused the error.
///
/// # Returns
///
/// The error message.
fn io_error_to_string(
	error: &std::io::Error,
	path: String,
) -> String
{
	let key = match error.kind()
	{
		std::io::ErrorKind::NotFound =>
		{
			"utils-source-code-read-error.not-found"
		}
		std::io::ErrorKind::IsADirectory =>
		{
			"utils-source-code-read-error.is-directory"
		}
		std::io::ErrorKind::PermissionDenied =>
		{
			"utils-source-code-read-error.permission-denied"
		}
		_ => "utils-source-code-read-error.generic",
	};

	t!(key, path = path)
}

/// The kind of error that might occurred across various
/// parts of the application.
#[derive(Debug)]
pub enum ErrorKind
{
	BasicError(String),
	/// An I/O error.
	IOError(std::io::Error, String),
	/// Lexer error
	LexerError(LexerError),
	/// Parser error
	ParserError(ParserError),
}

/// An error that might occurred across various parts of the
/// application.
#[derive(Debug)]
pub struct Error
{
	pub kind: ErrorKind,
	pub exit_code: ExitCode,
}

/// Implementation for `Error`.
impl Error
{
	/// Create a new error.
	///
	/// # Arguments
	///
	/// * `kind` - The kind of error.
	/// * `exit_code` - The exit code.
	///
	/// # Returns
	///
	/// The new error.
	pub fn new(kind: ErrorKind, exit_code: ExitCode) -> Self
	{
		Self { kind, exit_code }
	}

	/// Print the error to the standard error stream.
	pub fn print<'i, I, O, E>(
		&self,
		session_globals: &'i mut SessionGlobals<'i, I, O, E>,
	) where
		I: BufRead,
		O: WriteColor,
		E: WriteColor,
	{
		match &self.kind
		{
			ErrorKind::BasicError(msg) =>
			{
				let msg = t!("utils-error.template", error = msg);
				writeln!(session_globals.stderr, "{}", msg)
					.expect("failed to write to stderr writer");
			}
			ErrorKind::IOError(error, path) =>
			{
				let io_err_msg =
					io_error_to_string(error, path.clone());
				let msg =
					t!("utils-error.template", error = io_err_msg);
				// eprintln!("{}", msg);
				writeln!(session_globals.stderr, "{}", msg)
					.expect("failed to write to stderr writer");
			}
			ErrorKind::LexerError(lexer_error) =>
			{
				lexer_error.print(session_globals);
			}
			ErrorKind::ParserError(parser_error) =>
			{
				parser_error.print(session_globals);
			}
		}
	}
}

#[cfg(test)]
mod tests
{
	use serial_test::serial;
	use termcolor::Buffer;

	use crate::common::config::{ApplicationMode, Config};
	use crate::common::{ExitCode, Source, SourceOrigin};
	use crate::compiler::session_globals::SessionGlobals;
	use crate::parser::lexer::{LexerError, LexerErrorCode};
	use crate::parser::span::{Location, Position};
use crate::parser::ParserError;

	#[serial(lang)]
	#[test]
	fn test_io_error_to_string()
	{
		use std::io::ErrorKind;

		std::env::set_var("LANG", "en-US.UTF-8");
		let error = std::io::Error::new(
			ErrorKind::NotFound,
			"File not found",
		);
		let path = "file.txt".to_string();
		let result = super::io_error_to_string(&error, path);
		assert_eq!(
			result,
			"The path \u{2068}file.txt\u{2069} does not exist."
		);

		let error = std::io::Error::new(
			ErrorKind::IsADirectory,
			"Is a directory",
		);
		let path = "dir".to_string();
		let result = super::io_error_to_string(&error, path);
		assert_eq!(
			result,
			"The path \u{2068}dir\u{2069} is a directory."
		);

		let error = std::io::Error::new(
			ErrorKind::PermissionDenied,
			"Permission denied",
		);
		let path = "file.txt".to_string();
		let result = super::io_error_to_string(&error, path);
		assert_eq!(
			result,
			"The path \u{2068}file.txt\u{2069} is not readable. \
			 Please check the permissions."
		);

		let error =
			std::io::Error::new(ErrorKind::Other, "Other error");
		let path = "file.txt".to_string();
		let result = super::io_error_to_string(&error, path);
		assert_eq!(
			result,
			"An error occurred while reading the source code \
			 from \u{2068}file.txt\u{2069}."
		);
	}

	#[test]
	fn test_new_error()
	{
		let error = super::Error::new(
			super::ErrorKind::IOError(
				std::io::Error::new(
					std::io::ErrorKind::NotFound,
					"File not found",
				),
				"file.txt".to_string(),
			),
			ExitCode::IO_ERROR,
		);

		assert_eq!(error.exit_code, ExitCode::IO_ERROR);
	}

	#[test]
	fn test_error_print()
	{
		let error = super::Error::new(
			super::ErrorKind::BasicError(
				"An error occurred".to_string(),
			),
			ExitCode::ERROR,
		);

		let mut stderr = Buffer::no_color();
		let mut stdin = std::io::empty();
		let mut stdout = Buffer::no_color();

		let mut session_globals = SessionGlobals::new(
			ApplicationMode::Compiler,
			Config::default(),
			&mut stdin,
			&mut stdout,
			&mut stderr,
		);
		error.print(&mut session_globals);

		let result =
			String::from_utf8(stderr.into_inner()).unwrap();
		assert_eq!(
			result,
			"error: \u{2068}An error occurred\u{2069}\n"
		);
	}

	#[test]
	fn test_io_error_print()
	{
		let error = super::Error::new(
			super::ErrorKind::IOError(
				std::io::Error::new(
					std::io::ErrorKind::NotFound,
					"File not found",
				),
				"file.txt".to_string(),
			),
			ExitCode::IO_ERROR,
		);

		let mut stderr = Buffer::no_color();
		let mut stdin = std::io::empty();
		let mut stdout = Buffer::no_color();

		let mut session_globals = SessionGlobals::new(
			ApplicationMode::Compiler,
			Config::default(),
			&mut stdin,
			&mut stdout,
			&mut stderr,
		);
		error.print(&mut session_globals);

		let result =
			String::from_utf8(stderr.into_inner()).unwrap();
		assert_eq!(
			result,
			"error: \u{2068}The path \u{2068}file.txt\u{2069} \
			 does not exist.\u{2069}\n"
		);
	}

	#[test]
	fn test_print_lexer_error()
	{
		let source =
			Source::from("1+2").with_origin(SourceOrigin::String);
		let source_id = source.source_id().unwrap();

		let lexer_error = LexerError {
			code: LexerErrorCode::UnexpectedCharacter,
			message: "Unexpected character".to_owned(),
			hint: Some("Check the character".to_owned()),
			location: Location::Position(Position {
				line: 1,
				column: 1,
				offset: 0,
				char_index: 0,
			}),
			source_id: source_id.clone().into(),
		};

		let mut stdin = std::io::empty();
		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();
		let mut session_globals = SessionGlobals::new(
			ApplicationMode::Compiler,
			Config::default(),
			&mut stdin,
			&mut stdout,
			&mut stderr,
		);
		session_globals
			.source_id_map
			.insert(source_id.clone().into(), source);

		let error = super::Error::new(
			super::ErrorKind::LexerError(lexer_error),
			ExitCode::SYNTAX_ERROR,
		);
		error.print(&mut session_globals);
	}

	#[test]
	fn test_print_parser_error()
	{
		let source =
			Source::from("+").with_origin(SourceOrigin::String);
		let source_id = source.source_id().unwrap();

		let parser_error = ParserError {
			code: crate::parser::ParserErrorCode::ExpectedExpression,
			message: "Unexpected token".to_owned(),
			hint: Some("Check the token".to_owned()),
			location: Location::Position(Position {
				line: 1,
				column: 1,
				offset: 0,
				char_index: 0,
			}),
			source_id: source_id.clone().into(),
		};

		let mut stdin = std::io::empty();
		let mut stdout = Buffer::no_color();
		let mut stderr = Buffer::no_color();
		let mut session_globals = SessionGlobals::new(
			ApplicationMode::Compiler,
			Config::default(),
			&mut stdin,
			&mut stdout,
			&mut stderr,
		);
		session_globals
			.source_id_map
			.insert(source_id.clone().into(), source);

		let error = super::Error::new(
			super::ErrorKind::ParserError(parser_error),
			ExitCode::SYNTAX_ERROR,
		);
		error.print(&mut session_globals);
	}
}
