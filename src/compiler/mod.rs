use std::io::BufRead;

use session_globals::SessionGlobals;
use smol_str::SmolStr;
use termcolor::WriteColor;

use crate::common::error::{Error, ErrorKind};
use crate::common::{ExitCode, Source};
use crate::parser::lexer::Lexer;
use crate::parser::Parser;

pub mod session_globals;

/// Executes the compiler with given mode.
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
pub fn invoke_compiler<'i, I, O, E>(
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
	source_id: &SmolStr,
	source_code: &String,
	_session_globals: &mut SessionGlobals<'i, I, O, E>,
) -> Result<(), Error>
where
	I: BufRead,
	O: WriteColor,
	E: WriteColor,
{
	let tokens =
		Lexer::tokenize(source_id.clone(), source_code.clone())
			.map_err(|lexer_error| {
				Error::new(
					ErrorKind::LexerError(lexer_error),
					ExitCode::SYNTAX_ERROR,
				)
			})?;

	let module =
		Parser::parse(source_id.clone(), true, tokens)
			.map_err(|parser_error| {
				Error::new(
					ErrorKind::ParserError(parser_error),
					ExitCode::SYNTAX_ERROR,
				)
			})?;

	println!("{:#?}", module);

	Ok(())
}
