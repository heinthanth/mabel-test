//! # Error (error) module
//!
//! This module contains the error handling logic for the
//! application. It provides possible error kinds and the
//! error struct.

use std::io::Write;

use crate::common::ExitCode;
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
	pub fn print<W>(&self, mut stderr: W)
	where
		W: Write,
	{
		match &self.kind
		{
			ErrorKind::BasicError(msg) =>
			{
				let msg = t!("utils-error.template", error = msg);
				writeln!(&mut stderr, "{}", msg)
					.expect("failed to write to stderr writer");
			}
			ErrorKind::IOError(error, path) =>
			{
				let io_err_msg =
					io_error_to_string(error, path.clone());
				let msg =
					t!("utils-error.template", error = io_err_msg);
				// eprintln!("{}", msg);
				writeln!(&mut stderr, "{}", msg)
					.expect("failed to write to stderr writer");
			}
		}
	}
}

#[cfg(test)]
mod tests
{
	use crate::common::ExitCode;

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

		let mut stderr = Vec::new();
		error.print(&mut stderr);

		let result = String::from_utf8(stderr).unwrap();
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

		let mut stderr = Vec::new();
		error.print(&mut stderr);

		let result = String::from_utf8(stderr).unwrap();
		assert_eq!(
			result,
			"error: \u{2068}The path \u{2068}file.txt\u{2069} \
			 does not exist.\u{2069}\n"
		);
	}
}
