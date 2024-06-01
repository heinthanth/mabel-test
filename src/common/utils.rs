//! # Utilities (utils) module
//!
//! This module provides utility functions and macros for
//! various tasks such as date time formatting, file
//! reading, etc.

use std::io::{BufRead, IsTerminal};
use std::path::{Path, PathBuf};

use url::Url;

use super::error::{Error, ErrorKind};
use super::ExitCode;
use crate::t;

/// Get absolute path
///
/// # Arguments
///
/// * `path` - The absolute path
///
/// # Returns
///
/// The absolute path or an error.
pub fn get_absolute_path(
	path: PathBuf,
) -> Result<PathBuf, Error>
{
	std::path::absolute(path.clone()).map_err(|error| {
		Error::new(
			ErrorKind::IOError(
				error,
				path.to_string_lossy().to_string(),
			),
			ExitCode::IO_ERROR,
		)
	})
}

/// Convert a path to a file URL.
///
/// # Arguments
///
/// * `path` - the file path to convert.
///
/// # Returns
///
/// The file URL or an error.
pub fn path_to_file_url(
	path: PathBuf,
) -> Result<String, Error>
{
	Url::from_file_path(path.clone())
		.map(|url| url.to_string())
		.map_err(|_| {
			let msg = t!(
				"utils-path-to-file-url.cannot-convert",
				path = path.to_string_lossy()
			);

			Error::new(
				ErrorKind::IOError(
					std::io::Error::new(
						std::io::ErrorKind::InvalidInput,
						msg,
					),
					path.to_string_lossy().to_string(),
				),
				ExitCode::IO_ERROR,
			)
		})
}

/// Read source code from a file path.
///
/// # Arguments
///
/// * `path` - The path to the file to read.
///
/// # Errors
///
/// If an I/O error occurs while reading from the file.
///
/// # Returns
///
/// The source code read from the file.
pub fn read_content_from_path<P>(
	path: P,
) -> Result<String, std::io::Error>
where
	P: AsRef<Path>,
{
	std::fs::read_to_string(path)
}

/// Read source code from stdin.
///
/// # Errors
///
/// If an I/O error occurs while reading from stdin.
///
/// # Returns
///
/// The source code read from stdin.
pub fn read_content_from_buf_reader<R>(
	reader: &mut R,
) -> Result<String, std::io::Error>
where
	R: BufRead,
{
	reader.lines().collect()
}

/// Check if user force enables color output.
///
/// # Returns
///
/// `true` if color output is disabled, `false` otherwise.
pub fn is_color_output_force_enabled() -> bool
{
	std::env::var("COLOR").as_deref() == Ok("always")
}

/// Check if user force disables color output.
///
/// # Returns
///
/// `true` if color output is disabled, `false` otherwise.
pub fn is_color_output_disabled() -> bool
{
	if std::env::var("NO_COLOR").is_ok()
	{
		return true;
	}
	if is_color_output_force_enabled()
	{
		return false;
	}

	if std::env::var("COLOR").as_deref() == Ok("never")
	{
		return true;
	}

	return std::env::var("TERM").as_deref() == Ok("dumb")
		|| !std::io::stdin().is_terminal()
		|| !std::io::stdout().is_terminal()
		|| !std::io::stderr().is_terminal();
}

/// Helper to get certain enum variant without explicit
/// pattern matching
///
/// # Arguments
///
/// * `enum` - The enum to get the variant from.
/// * `variant` - The variant to get.
/// * `data` - The data to return if the variant matches.
#[macro_export]
macro_rules! get_enum_variant {
	($enum:expr, $variant:pat, $data:expr) => {
		match $enum
		{
			$variant => Some($data),
			_ => None,
		}
	};
}

/// Ternary operator macro
///
/// # Arguments
///
/// * `condition` - The condition to check.
/// * `true_value` - The value to return if the condition is
///   true.
/// * `false_value` - The value to return if the condition
///   is false.
///
/// # Returns
///
/// The value based on the condition.
#[macro_export]
macro_rules! ternary {
	(
		$condition:expr, $true_value:expr, $false_value:expr
	) => {
		if ($condition) == true
		{
			$true_value
		}
		else
		{
			$false_value
		}
	};
}

/// Create a local date from the current date or a date
/// string and format.
///
/// # Returns
///
/// The local date.
#[macro_export]
macro_rules! localdate {
	($year:literal) => {
		chrono::NaiveDate::parse_from_str(
			concat!($year, "-01-01"),
			"%Y-%m-%d",
		)
		.unwrap()
	};
	($year:literal, $month:literal) => {
		chrono::NaiveDate::parse_from_str(
			concat!($year, "-", $month, "-01"),
			"%Y-%m-%d",
		)
		.unwrap()
	};
	($year:literal, $month:literal, $day:literal) => {
		chrono::NaiveDate::parse_from_str(
			concat!($year, "-", $month, "-", $day),
			"%Y-%m-%d",
		)
		.unwrap()
	};
}

/// Format a date time with a format string in current
/// system locale.
///
/// # Arguments
///
/// * `datetime` - The date time to format.
/// * `format` - The format string.
///
/// # Returns
///
/// The formatted date time.
#[macro_export]
macro_rules! format_datetime {
	($datetime:expr, $format:literal) => {
		$datetime
			.format_localized(
				$format,
				$crate::common::i18n::get_os_locale().chrono_locale,
			)
			.to_string()
	};
}

#[cfg(test)]
mod tests
{
	use chrono::NaiveDate;
	use serial_test::serial;

	use crate::common::utils::path_to_file_url;
	use crate::common::ExitCode;

	#[test]
	fn test_read_content_from_path()
	{
		let content = super::read_content_from_path(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/tests/scripts/test.mbl",
		));
		assert!(content.is_ok());
		assert_eq!(content.unwrap(), "echo \"Hello, World!\"");
	}

	#[test]
	fn test_read_content_from_buf_reader()
	{
		let mut reader = std::io::BufReader::new(
			std::io::Cursor::new("echo \"Hello, World!\""),
		);
		let content =
			super::read_content_from_buf_reader(&mut reader);
		assert!(content.is_ok());
		assert_eq!(content.unwrap(), "echo \"Hello, World!\"");
	}

	#[test]
	fn test_localdate()
	{
		assert_eq!(
			localdate!(2021),
			NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
		);
		assert_eq!(
			localdate!(2021, 12),
			NaiveDate::from_ymd_opt(2021, 12, 1).unwrap()
		);
		assert_eq!(
			localdate!(2021, 12, 31),
			NaiveDate::from_ymd_opt(2021, 12, 31).unwrap()
		);
	}

	#[serial(lang)]
	#[test]
	fn test_format_datetime()
	{
		std::env::set_var("LANG", "en-US.UTF-8");

		let datetime =
			NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
		assert_eq!(
			format_datetime!(datetime, "%Y-%m-%d"),
			"2021-01-01"
		);
	}

	#[test]
	fn test_get_file_url_without_absolute_path()
	{
		use std::path::PathBuf;

		let path = PathBuf::from("test.mbl");
		let file_url = path_to_file_url(path.clone());

		assert!(file_url.is_err());
		let error = file_url.unwrap_err();
		assert_eq!(error.exit_code, ExitCode::IO_ERROR);
	}

	#[test]
	fn test_get_file_url_with_absolute_path()
	{
		use std::path::PathBuf;

		let path = PathBuf::from(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/tests/scripts/test.mbl",
		));
		let file_url = path_to_file_url(path.clone());

		assert!(file_url.is_ok());
		assert_eq!(
			file_url.unwrap(),
			format!("file://{}", path.to_string_lossy())
		);
	}

	#[serial(color)]
	#[test]
	fn test_force_color_output_enabled()
	{
		std::env::set_var("COLOR", "always");

		assert!(super::is_color_output_force_enabled());

		std::env::remove_var("COLOR");
		std::env::set_var("COLOR", "always");
		assert!(super::is_color_output_force_enabled());
	}

	#[serial(color)]
	#[test]
	fn test_force_color_output_disabled()
	{
		std::env::set_var("NO_COLOR", "1");
		assert!(super::is_color_output_disabled());

		std::env::remove_var("NO_COLOR");
		std::env::set_var("COLOR", "always");
		assert!(!super::is_color_output_disabled());

		std::env::set_var("COLOR", "never");
		assert!(super::is_color_output_disabled());
	}
}
