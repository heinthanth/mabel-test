//! # Utilities (utils) module
//!
//! This module provides utility functions and macros for
//! various tasks such as date time formatting, file
//! reading, etc.

use std::io::BufRead;
use std::path::Path;

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
	reader: R,
) -> Result<String, std::io::Error>
where
	R: BufRead,
{
	reader
		.lines()
		.map(|line| line)
		.collect::<Result<Vec<_>, _>>()
		.map(|lines| lines.join("\n"))
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
		let content = super::read_content_from_buf_reader(
			std::io::BufReader::new(std::io::Cursor::new(
				"echo \"Hello, World!\"",
			)),
		);
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
}
