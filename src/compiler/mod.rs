use std::io::BufRead;

use session_globals::SessionGlobals;
use termcolor::WriteColor;

use crate::common::error::{Error, ErrorKind};
use crate::common::{ExitCode, Source};
use crate::parser::lexer::Lexer;

pub mod session_globals;

/// Executes the application mode.
/// It invoke the lexer, parser and other components based
/// on the application mode.
///
/// # Arguments
///
/// * `mode` - The application mode.
/// * `source` - The source code.
/// * `config` - The configuration.
/// * `stdin` - The input stream.
/// * `stdout` - The output stream.
/// * `stderr` - The error stream.
///
/// # Returns
///
/// The result or an error.
pub fn invoke_application_mode<'i, I, O, E>(
	source: Source,
	session_globals: &mut SessionGlobals<'i, I, O, E>,
) -> Result<ExitCode, Error>
where
	I: BufRead,
	O: WriteColor,
	E: WriteColor,
{
	// we can't test to fail here because we don't have a way
	// to pass an invalid source to this function
	#[cfg(coverage)]
	let source_id = source.source_id().unwrap();

	#[cfg(not(coverage))]
	let source_id = source.source_id()?;

	// register the main source
	session_globals
		.source_id_map
		.insert(source_id.clone().into(), source.clone());

	parse_source(&source_id, &source.code, session_globals)?;
	Ok(ExitCode::SUCCESS)
}

/// Parses the source code into AST.
/// It invokes the lexer and parser to parse the source
/// code.
///
/// # Arguments
///
/// * `source` - The source code.
/// * `session_globals` - The session globals.
///
/// # Returns
///
/// The result or an error.
fn parse_source<'i, I, O, E>(
	source_id: &String,
	source_code: &String,
	_session_globals: &mut SessionGlobals<'i, I, O, E>,
) -> Result<(), Error>
where
	I: BufRead,
	O: WriteColor,
	E: WriteColor,
{
	let tokens = Lexer::tokenize(
		source_id.to_string().clone().into(),
		source_code.clone(),
	)
	.map_err(|lexer_error| {
		Error::new(
			ErrorKind::LexerError(lexer_error),
			ExitCode::SYNTAX_ERROR,
		)
	})?;
	println!("{:#?}", tokens);

	Ok(())
}
