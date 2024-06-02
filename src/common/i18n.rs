//! # Internationalization (i18n) module.
//!
//! This module provides localization support using
//! Mozilla's Fluent. It also provides a macro `t!` to
//! lookup keys in the current OS locale.
//!
//! # Example
//!
//! ```
//! use mabel::t;
//!
//! // Note: Those macros should return actual values from .ftl but
//! // since the keys do not exist, it will return the default value or
//! // the key itself.
//!
//! std::env::set_var("LANG", "en-US.UTF-8");
//!
//! // lookup a key in current OS locale
//! let key = "foobar1234";
//! let value = t!(key);
//! assert_eq!(value, "foobar1234");
//!
//! // lookup a key in current OS locale with arguments
//! let key = "foobar1234";
//! let value = t!(key, arg1="value1", arg2="value2");
//! assert_eq!(value, "foobar1234");
//!
//! // lookup a key in current OS locale with default value
//! let key = "foobar1234";
//! let value = t!(key, "Hello, World");
//! assert_eq!(value, "Hello, World");
//!
//! // lookup a key in current OS locale with arguments and default value
//! let key = "foobar1234";
//! let value = t!(key, arg1="value1", arg2="value2"; "Hello, World");
//! assert_eq!(value, "Hello, World");
//! ```

use chrono::prelude::Locale;
use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::{static_loader, Loader};
use sys_locale::get_locale;
use unic_langid::{langid, LanguageIdentifier};

static_loader! {
	static LOCALES = {
		locales: "locales",
		fallback_language: "en-US",
	};
}

/// The current locale of the system.
#[derive(Debug, Clone)]
pub struct SystemLocale
{
	pub langid: LanguageIdentifier,
	pub chrono_locale: Locale,
}

/// Default trait implementation of the `SystemLocale`.
impl Default for SystemLocale
{
	/// Create a new `SystemLocale` with default values.
	fn default() -> Self
	{
		SystemLocale {
			langid: langid!("en-US"),
			chrono_locale: Locale::en_US,
		}
	}
}

/// Get the current locale of the system.
/// If the locale is not supported, it will fallback to
/// "en-US".
///
/// # Returns
///
/// The current locale of the system.
pub fn get_os_locale() -> SystemLocale
{
	// prioritize LANG environment variable
	let lang = std::env::var("LANG").map(|lang| {
		lang
			// split by '.' and take the first part (e.g. en, en-US, en-US.UTF-8)
			.split('.')
			.nth(0)
			.unwrap() // safe to unwrap() because even empty string has first part
			.to_string()
			// replace '_' with '-'
			.replace('_', "-")
	});

	// if it is not set, fallback to the locale of the system
	match lang
		.unwrap_or_else(|_| {
			// this is for production, at least en-US should be
			// returned
			#[cfg(not(coverage))]
			return get_locale()
				.unwrap_or_else(|| "en-US".to_string());

			// in Linux, we can write test cases to make
			// `get_locale()` function return `None`
			#[cfg(coverage)]
			#[cfg(target_os = "linux")]
			return get_locale()
				.unwrap_or_else(|| "en-US".to_string());

			// sometimes, we can't write test cases to make
			// `get_locale()` function return `None` in platform
			// like macOS, windows and will assume it's safe to
			// unwrap
			#[cfg(coverage)]
			#[cfg(not(target_os = "linux"))]
			return get_locale().unwrap();
		})
		.to_ascii_lowercase()
		.as_str()
	{
		"en-us" => SystemLocale {
			langid: langid!("en-US"),
			chrono_locale: Locale::en_US,
		},
		"my-mm" => SystemLocale {
			langid: langid!("my-MM"),
			chrono_locale: Locale::my_MM,
		},
		locale
			if locale.starts_with("en-") || locale == "en" =>
		{
			SystemLocale {
				langid: langid!("en-US"),
				chrono_locale: Locale::en_US,
			}
		}
		locale
			if locale.starts_with("my-") || locale == "my" =>
		{
			SystemLocale {
				langid: langid!("my-MM"),
				chrono_locale: Locale::my_MM,
			}
		}
		_ => SystemLocale::default(),
	}
}

/// Lookup a key in current OS locale.
///
/// # Arguments
///
/// * `key` - The key to lookup.
/// * `args` - The arguments to pass to the key.
/// * `default_value` - The default value to return if the
///   key does not exist.
///
/// # Returns
///
/// The value of the key if it exists, otherwise default
/// value or the key itself.
///
/// # Example
///
/// ```
/// use mabel::common::i18n::lookup;
///
/// let key = "foobar1234";
/// let value = lookup(key, None, None);
///
/// assert_eq!(value, "foobar1234");
///
/// let key = "foobar1234";
/// let value =
/// 	lookup(key, None, Some("Hello, World".to_string()));
///
/// assert_eq!(value, "Hello, World");
/// ```
pub fn lookup(
	key: &str,
	args: Option<&[(String, FluentValue)]>,
	default_value: Option<String>,
) -> String
{
	LOCALES
		.try_lookup_with_args(
			&get_os_locale().langid,
			key,
			&args
				.unwrap_or_else(|| &[])
				.iter()
				.cloned()
				.collect::<_>(),
		)
		.unwrap_or_else(|| {
			default_value.unwrap_or_else(|| key.to_string())
		})
}

/// Lookup a key in current OS locale with or without
/// arguments. If the key does not exist, it will return the
/// default value or the key itself.
///
/// # Arguments
///
/// * `key` - The key to lookup.
/// * `args` - The arguments to pass to the key.
/// * `default_value` - The default value to return if the
///   key does not exist.
///
/// # Returns
///
/// The value of the key if it exists, otherwise the key
#[macro_export]
macro_rules! t {
  ($key:expr) => {
    // t!("key")
    // Lookup a key in current OS locale.
    $crate::common::i18n::lookup($key, None, None)
  };
  (
    $key:expr,
    $($arg:ident=$value:expr),*
  ) => {
    // t!("key", arg1="value1", arg2="value2")
    // Lookup a key in current OS locale with arguments.
    $crate::common::i18n::lookup(
		$key,
		Some(&[
			$((stringify!($arg).to_string(),$value.into())),*
		]),
		None
    )
  };
	(
    $key:expr,
    $default:expr
  ) => {
    // t!("key", "default")
    // Lookup a key in current OS locale with default value.
    $crate::common::i18n::lookup($key, None, Some($default.to_string()))
  };
  (
    $key:expr,
    $($arg:ident=$value:expr),*;
    $default:expr
  ) => {
    // t!("key", arg1="value1", arg2="value2", "default")
    // Lookup a key in current OS locale with arguments and default value.
    $crate::common::i18n::lookup(
		$key,
		Some(&[
			$((stringify!($arg).to_string(),$value.into())),*
		]),
		Some($default.to_string())
    )
  };
}

#[cfg(test)]
mod tests
{
	use serial_test::serial;

	/// Test the `get_os_locale` function.
	/// It should return the current locale of the system
	/// depending on the LANG environment variable.
	#[serial(lang)]
	#[test]
	fn get_os_locale()
	{
		// set envionment variable
		std::env::set_var("LANG", "en-US.UTF-8");

		let locale = super::get_os_locale();
		assert_eq!(
			locale.langid,
			unic_langid::langid!("en-US")
		);
		assert_eq!(locale.chrono_locale, chrono::Locale::en_US);

		// set envionment variable
		std::env::set_var("LANG", "my-MM.UTF-8");

		let locale = super::get_os_locale();
		assert_eq!(
			locale.langid,
			unic_langid::langid!("my-MM")
		);
		assert_eq!(locale.chrono_locale, chrono::Locale::my_MM);
	}

	#[serial(lang)]
	#[test]
	fn fallback_os_locale()
	{
		// set envionment variable
		std::env::set_var("LANG", "en-GB.UTF-8");

		let locale = super::get_os_locale();
		assert_eq!(
			locale.langid,
			unic_langid::langid!("en-US")
		);
		assert_eq!(locale.chrono_locale, chrono::Locale::en_US);

		// set envionment variable
		std::env::set_var("LANG", "my-TH.UTF-8");

		let locale = super::get_os_locale();
		assert_eq!(
			locale.langid,
			unic_langid::langid!("my-MM")
		);
		assert_eq!(locale.chrono_locale, chrono::Locale::my_MM);

		// set envionment variable
		std::env::set_var("LANG", ".UTF-8");

		let locale = super::get_os_locale();
		assert_eq!(
			locale.langid,
			unic_langid::langid!("en-US")
		);
		assert_eq!(locale.chrono_locale, chrono::Locale::en_US);

		// set envionment variable
		std::env::set_var("LANG", "");

		let locale = super::get_os_locale();
		assert_eq!(
			locale.langid,
			unic_langid::langid!("en-US")
		);
		assert_eq!(locale.chrono_locale, chrono::Locale::en_US);

		// set envionment variable
		std::env::set_var("LANG", "invalid");

		let locale = super::get_os_locale();
		assert_eq!(
			locale.langid,
			unic_langid::langid!("en-US")
		);
		assert_eq!(locale.chrono_locale, chrono::Locale::en_US);

		// remove envionment variable
		std::env::remove_var("LANG");

		let locale = super::get_os_locale();
		assert_eq!(
			locale.langid,
			unic_langid::langid!("en-US")
		);
		assert_eq!(locale.chrono_locale, chrono::Locale::en_US);
	}

	/// Test the `lookup` function.
	/// It should return the value of the key and properly
	/// interpolate the arguments.
	#[serial(lang)]
	#[test]
	fn simple_lookup()
	{
		// set envionment variable
		std::env::set_var("LANG", "en-US.UTF-8");

		// simple lookup
		let value = crate::t!("common-internal-test.test1");
		assert_eq!(value, "Hello, World!");

		// lookup with arguments
		let value = crate::t!(
			"common-internal-test.test2",
			pa_name = "Bob",
			pb_name = "Alice"
		);
		assert_eq!(
			value,
			"Hello there! I'm \u{2068}Bob\u{2069}, Nice to meet \
			 u, \u{2068}Alice\u{2069}!"
		);
	}

	/// Test the `lookup` function with default value.
	/// It should return the default value if the key does
	/// not exist.
	#[serial(lang)]
	#[test]
	fn lookup_with_default_value()
	{
		// set envionment variable
		std::env::set_var("LANG", "en-US.UTF-8");

		// lookup with default value
		let value = crate::t!(
			"common-internal-test.test6",
			"Hello, World!"
		);
		assert_eq!(value, "Hello, World!");

		// lookup with arguments and default value
		let value = crate::t!(
			"common-internal-test.test7",
			pa_name = "Bob", pb_name = "Alice";
			"Hello, World!"
		);
		assert_eq!(value, "Hello, World!");
	}

	/// Test the `lookup` function with non-existent key.
	/// It should return the key itself if the key does not
	/// exist.
	#[serial(lang)]
	#[test]
	fn lookup_non_existent_key()
	{
		// set envionment variable
		std::env::set_var("LANG", "en-US.UTF-8");

		let value = crate::t!("common-internal-test.test3",);
		assert_eq!(value, "common-internal-test.test3");

		let value = crate::t!(
			"common-internal-test.test4",
			pa_name = "Bob",
			pb_name = "Alice"
		);
		assert_eq!(value, "common-internal-test.test4");
	}
}
