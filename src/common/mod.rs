//! # Common (common) module
//!
//! This module contains the common logic for the
//! application. It provides the core functionality,
//! configuration, error handling, internationalization, and
//! utilities.

pub mod config;
pub mod error;
pub mod i18n;
pub mod utils;

use std::path::PathBuf;

use utils::{get_absolute_path, path_to_file_url};

use crate::common::error::{Error, ErrorKind};
use crate::common::utils::read_content_from_path;

/// The exit codes used across the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExitCode(i32);

/// `Into<i32>` implementation for `ExitCode`.
impl Into<i32> for ExitCode
{
	/// Convert the exit code into an integer.
	fn into(self) -> i32
	{
		self.0
	}
}

/// `From<i32>` implementation for `ExitCode`.
impl From<i32> for ExitCode
{
	/// Convert an integer into an exit code.
	fn from(value: i32) -> Self
	{
		Self(value)
	}
}

/// Implementation for `ExitCode`.
impl ExitCode
{
	/// The error exit code.
	pub const ERROR: ExitCode = ExitCode(1);
	/// The invalid input or usage exit code.
	pub const INVALID_INPUT_OR_USAGE: ExitCode = ExitCode(2);
	pub const IO_ERROR: ExitCode = ExitCode(3);
	/// The semantic error exit code.
	pub const SEMANTIC_ERROR: ExitCode = ExitCode(12);
	/// The success exit code.
	pub const SUCCESS: ExitCode = ExitCode(0);
	/// The syntax error exit code.
	pub const SYNTAX_ERROR: ExitCode = ExitCode(11);
}

/// The origin of the source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceOrigin
{
	/// The source code is from a string.
	String,
	/// The source code is from standard input.
	Stdin,
	/// The source code is from a REPL.
	REPL,
	/// The source code is from a file.
	File(PathBuf),
}

impl SourceOrigin
{
	/// Get the filename of the source code.
	/// If the source code is from a file, the filename
	/// is returned. Otherwise, a string representation
	/// of the origin is returned.
	///
	/// # Returns
	///
	/// The filename of the source code.
	///
	/// # Example
	///
	/// ```
	/// use std::path::PathBuf;
	///
	/// use mabel::common::SourceOrigin;
	///
	/// let origin =
	/// 	SourceOrigin::File(PathBuf::from("test.mbl"));
	///
	/// assert_eq!(origin.get_filename(), "test.mbl");
	/// ```
	pub fn get_filename(&self) -> String
	{
		match self
		{
			SourceOrigin::File(file) => file
				.file_name()
				.expect("expected a file")
				.to_string_lossy()
				.to_string(),
			_ => self.get_path(),
		}
	}

	/// Get the path of the source code.
	/// If the source code is from a file, the path
	/// is returned. Otherwise, a string representation
	/// of the origin is returned.
	///
	/// # Returns
	///
	/// The path of the source code.
	///
	/// # Example
	///
	/// ```
	/// use std::path::PathBuf;
	///
	/// use mabel::common::{SourceOrigin};
	///
	/// let origin = SourceOrigin::File(PathBuf::from("test.mbl"));
	///
	/// assert_eq!(origin.get_path(), "test.mbl");
	pub fn get_path(&self) -> String
	{
		match self
		{
			SourceOrigin::File(file) =>
			{
				file.to_string_lossy().to_string()
			}
			SourceOrigin::Stdin => "stdin".to_string(),
			_ => format!("{:?}", self),
		}
	}
}

/// The source structure of the program.
#[derive(Debug, Clone)]
pub struct Source
{
	/// The origin of the source code
	pub origin: SourceOrigin,
	/// The source code
	pub code: String,
}

impl Source
{
	/// Create a new `Source` with the given origin and
	/// code.
	///
	/// # Arguments
	///
	/// * `origin` - The origin of the source code.
	/// * `code` - The source code.
	///
	/// # Returns
	///
	/// The new `Source`.
	pub fn new(origin: SourceOrigin, code: String) -> Self
	{
		Self { origin, code }
	}

	/// Set the origin of the source code.
	/// This is useful when the origin of the source code
	/// is determined after the source code is created.
	///
	/// # Arguments
	///
	/// * `origin` - The new origin of the source code.
	///
	/// # Returns
	///
	/// The new `Source` with given origin.
	///
	/// # Example
	///
	/// ```
	/// use std::path::PathBuf;
	///
	/// use mabel::common::{Source, SourceOrigin};
	///
	/// let source =
	/// 	Source::from("1 + 32").with_origin(SourceOrigin::REPL);
	///
	/// assert_eq!(source.origin, SourceOrigin::REPL);
	/// ```
	pub fn with_origin(&self, origin: SourceOrigin) -> Self
	{
		Self::new(origin, self.code.clone())
	}

	/// Get the filename of the source code.
	/// It calls the `get_filename` method of the origin
	/// of the source code.
	///
	/// # Returns
	///
	/// The filename of the source code.
	///
	/// # Example
	///
	/// ```
	/// use std::path::PathBuf;
	///
	/// use mabel::common::{Source, SourceOrigin};
	///
	/// let source = Source::from("1 + 32").with_origin(
	/// 	SourceOrigin::File(PathBuf::from("test.mbl")),
	/// );
	///
	/// assert_eq!(source.get_filename(), "test.mbl");
	/// ```
	pub fn get_filename(&self) -> String
	{
		self.origin.get_filename()
	}

	/// Get the path of the source code.
	/// It calls the `get_path` method of the origin
	/// of the source code.
	///
	/// # Returns
	///
	/// The path of the source code.
	///
	/// # Example
	///
	/// ```
	/// use std::path::PathBuf;
	///
	/// use mabel::common::{Source, SourceOrigin};
	///
	/// let source = Source::from("1 + 32").with_origin(
	/// 	SourceOrigin::File(PathBuf::from("test.mbl")),
	/// );
	///
	/// assert_eq!(source.get_path(), "test.mbl");
	/// ```
	pub fn get_path(&self) -> String
	{
		self.origin.get_path()
	}

	/// Create a source id from the source.
	/// The source id is in the format such as
	/// `mabel://stdin`, `mabel://REPL` or `file://path`.
	///
	/// # Returns
	///
	/// The source id.
	pub fn source_id(&self) -> Result<String, Error>
	{
		match self.origin
		{
			SourceOrigin::String | SourceOrigin::Stdin =>
			{
				Ok("mabel://stdin".to_string())
			}
			SourceOrigin::REPL => Ok("mabel://REPL".to_string()),
			SourceOrigin::File(ref path) =>
			{
				let abs_path = get_absolute_path(path.clone())?;
				path_to_file_url(abs_path)
			}
		}
	}
}

/// `Default` implementation for `Source`.
impl Default for Source
{
	/// Create a new `Source` with default values.
	fn default() -> Self
	{
		Self {
			origin: SourceOrigin::String,
			code: String::new(),
		}
	}
}

/// `From<&str>` implementation for `Source`.
impl From<&str> for Source
{
	/// Create a new `Source` from a string slice.
	fn from(code: &str) -> Self
	{
		Self::new(SourceOrigin::String, code.to_string())
	}
}

/// `From<String>` implementation for `Source`.
impl From<String> for Source
{
	/// Create a new `Source` from a string.
	fn from(code: String) -> Self
	{
		Self::new(SourceOrigin::String, code)
	}
}

/// `From<PathBuf>` implementation for `Source`.
impl TryFrom<PathBuf> for Source
{
	/// The error type for the conversion.
	type Error = Error;

	/// Create a new `Source` from a file path.
	fn try_from(path: PathBuf) -> Result<Self, Self::Error>
	{
		read_content_from_path(path.clone())
			.map_err(|error| {
				Error::new(
					ErrorKind::IOError(
						error,
						path.to_string_lossy().to_string(),
					),
					ExitCode::IO_ERROR,
				)
			})
			.map(|code| {
				Source::from(code)
					.with_origin(SourceOrigin::File(path))
			})
	}
}

#[cfg(test)]
mod tests
{
	use url::Url;

	use crate::common::ExitCode;

	#[test]
	fn test_convert_exit_code_to_integer()
	{
		use crate::common::ExitCode;

		let exit_code = ExitCode::ERROR;
		let integer: i32 = exit_code.into();

		assert_eq!(integer, 1);
	}

	#[test]
	fn test_convert_integer_to_exit_code()
	{
		use crate::common::ExitCode;

		let integer = 1;
		let exit_code: ExitCode = integer.into();

		assert_eq!(exit_code, ExitCode::ERROR);
	}

	#[test]
	fn test_get_filename_of_source_origin()
	{
		use std::path::PathBuf;

		use crate::common::SourceOrigin;

		let origin =
			SourceOrigin::File(PathBuf::from("test.mbl"));
		assert_eq!(origin.get_filename(), "test.mbl");
	}

	#[test]
	fn test_get_path_of_source_origin()
	{
		use std::path::PathBuf;

		use crate::common::SourceOrigin;

		let origin =
			SourceOrigin::File(PathBuf::from("test.mbl"));
		assert_eq!(origin.get_path(), "test.mbl");

		let origin = SourceOrigin::Stdin;
		assert_eq!(origin.get_path(), "stdin");
	}

	#[test]
	fn test_get_filename_of_source()
	{
		use std::path::PathBuf;

		use crate::common::{Source, SourceOrigin};

		let source = Source::from("1 + 32").with_origin(
			SourceOrigin::File(PathBuf::from("test.mbl")),
		);
		assert_eq!(source.get_filename(), "test.mbl");

		let source = Source::from("1 + 32")
			.with_origin(SourceOrigin::Stdin);
		assert_eq!(source.get_filename(), "stdin");
	}

	#[test]
	fn test_get_path_of_source()
	{
		use std::path::PathBuf;

		use crate::common::{Source, SourceOrigin};

		let source = Source::from("1 + 32").with_origin(
			SourceOrigin::File(PathBuf::from("test.mbl")),
		);

		assert_eq!(source.get_path(), "test.mbl");

		let source = Source::from("1 + 32")
			.with_origin(SourceOrigin::Stdin);
		assert_eq!(source.get_path(), "stdin");

		let source = Source::from("1 + 32")
			.with_origin(SourceOrigin::REPL);
		assert_eq!(source.get_path(), "REPL");
	}

	#[test]
	fn test_create_source_from_string_slice()
	{
		use crate::common::Source;

		let source = Source::from("1 + 32");
		assert_eq!(source.code, "1 + 32");
	}

	#[test]
	fn test_create_source_from_string()
	{
		use crate::common::Source;

		let source = Source::from("1 + 32".to_string());

		assert_eq!(source.code, "1 + 32");
	}

	#[test]
	fn test_create_source_from_file_path()
	{
		use std::path::PathBuf;

		use crate::common::{Source, SourceOrigin};

		let path = PathBuf::from(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/tests/scripts/test.mbl"
		));
		let source = Source::try_from(path.clone());

		assert!(source.is_ok());

		let source = source.unwrap();

		assert_eq!(source.code, "echo \"Hello, World!\"");
		assert_eq!(source.origin, SourceOrigin::File(path));
	}

	#[test]
	fn test_create_source_from_file_path_causing_error()
	{
		use std::path::PathBuf;

		use crate::common::Source;

		let path =
			PathBuf::from("locales/thismightnotexist.mbl");
		let source = Source::try_from(path.clone());

		assert!(source.is_err());

		let error = source.unwrap_err();

		assert_eq!(error.exit_code, ExitCode::IO_ERROR);
	}

	#[test]
	fn test_default_source()
	{
		use crate::common::{Source, SourceOrigin};

		let source = Source::default();

		assert_eq!(source.code, "");
		assert_eq!(source.origin, SourceOrigin::String);
	}

	#[test]
	fn test_get_source_id()
	{
		use crate::common::{Source, SourceOrigin};

		let source = Source::from("1 + 32").with_origin(
			SourceOrigin::File(std::path::PathBuf::from(
				"test.mbl",
			)),
		);

		assert_eq!(
			source.source_id().unwrap(),
			Url::from_file_path(
				std::path::absolute(concat!(
					env!("CARGO_MANIFEST_DIR"),
					"/test.mbl"
				))
				.unwrap()
			)
			.unwrap()
			.to_string()
		);

		let source = Source::from("1 + 32")
			.with_origin(SourceOrigin::Stdin);

		assert_eq!(
			source.source_id().unwrap(),
			"mabel://stdin"
		);

		let source = Source::from("1 + 32")
			.with_origin(SourceOrigin::REPL);

		assert_eq!(source.source_id().unwrap(), "mabel://REPL");
	}

	#[test]
	fn test_get_source_id_causing_error()
	{
		use crate::common::{Source, SourceOrigin};

		let source = Source::from("1 + 32").with_origin(
			SourceOrigin::File(std::path::PathBuf::from("")),
		);
		let error = source.source_id().unwrap_err();
		assert_eq!(error.exit_code, ExitCode::IO_ERROR);
	}
}
