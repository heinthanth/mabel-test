use std::io::BufRead;
use std::ops::Range;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use serde_json::json;
use smol_str::SmolStr;
use termcolor::WriteColor;
use unicode_segmentation::UnicodeSegmentation;
use unicode_xid::UnicodeXID;

use super::span::{Location, Position, Span};
use super::token::{
	FloatLiteralToken,
	IntegerLiteralToken,
	LiteralTokenKind,
	NumberBase,
	Token,
	TokenKind,
};
use crate::compiler::session_globals::SessionGlobals;
use crate::{t, ternary};

/// Lexer error code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexerErrorCode
{
	/// Unexpected character
	UnexpectedCharacter,
	/// Invalid number width
	InvalidNumberLiteralWidth,
	/// Unimplemented feature
	UnimplementedFeature,
}

/// Lexer error
#[derive(Debug, Clone)]
pub struct LexerError
{
	/// The error code
	pub code: LexerErrorCode,
	/// The error message
	pub message: String,
	/// Hint for the error
	pub hint: Option<String>,
	/// The position of the error
	pub location: Location,
	/// The source id of the error
	pub source_id: SmolStr,
}

/// Implementation of `LexerError`.
impl LexerError
{
	fn create_diagnostic<'i, I, O, E>(
		&self,
		session_globals: &mut SessionGlobals<'i, I, O, E>,
	) -> (SimpleFiles<String, String>, usize, Diagnostic<usize>)
	where
		I: BufRead,
		O: WriteColor,
		E: WriteColor,
	{
		let mut diagnostic_files = SimpleFiles::new();
		let current_source = session_globals
			.source_id_map
			.get(&self.source_id)
			.unwrap();

		let file_id = diagnostic_files.add(
			current_source.get_path(),
			current_source.code.clone(),
		);

		let label = Label::primary(
			file_id,
			Into::<Range<usize>>::into(self.location),
		)
		.with_message(self.message.clone().to_string());

		let diagnostic = Diagnostic::error()
			.with_code(format!("{:?}", self.code))
			.with_message(self.message.clone().to_string() + "\n")
			.with_labels(vec![label])
			.with_notes(
				self
					.hint
					.clone()
					.map(|hint| vec![hint])
					.unwrap_or_default(),
			);

		(diagnostic_files, file_id, diagnostic)
	}

	/// Print the error
	pub fn print<'i, I, O, E>(
		&self,
		session_globals: &mut SessionGlobals<'i, I, O, E>,
	) where
		I: BufRead,
		O: WriteColor,
		E: WriteColor,
	{
		let (diagnostic_files, _, diagnostic) =
			self.create_diagnostic(session_globals);

		let config =
			codespan_reporting::term::Config::default();

		codespan_reporting::term::emit(
			session_globals.stderr,
			&config,
			&diagnostic_files,
			&diagnostic,
		)
		.unwrap();
	}
}

/// Lexer result
pub type LexerResult<T> = Result<T, LexerError>;

/// Lexer
/// It's used to tokenize the source code.
pub struct Lexer
{
	/// The source code id
	source_id: SmolStr,
	//// The source code
	source_code: String,
	/// The unicode clusters of the source code.
	graphemes: Vec<(usize, String)>,
	/// the start position
	start: Position,
	/// the current position
	current: Position,
	/// Previous line column count
	previous_line_column_count: usize,
}

/// Check if a character is skipable character.
///
/// # Arguments
///
/// * `c` - The character to check.
///
/// # Returns
///
/// True if the character is skipable, false otherwise.
fn is_skipable(c: String) -> bool
{
	// whitespace characters are usually single character and
	// not complex clusters. since '\n', ' ' and '\t' are part
	// of some token kinds, we should not skip them
	c.len() == 1
		&& c.chars().next().unwrap().is_whitespace()
		&& !matches!(c.as_str(), "\n" | "\r\n" | " " | "\t")
}

/// Check if a character is a newline character.
/// It checks for '\n' and '\r\n'.
///
/// # Arguments
///
/// * `c` - The character to check.
///
/// # Returns
///
/// True if the character is a newline character, false
/// otherwise.
fn is_newline(c: String) -> bool
{
	c == "\n" || c == "\r\n"
}

/// Check if a character is a digit.
///
/// # Arguments
///
/// * `c` - The character to check.
///
/// # Returns
///
/// True if the character is a digit, false otherwise.
fn is_digit(c: String, base: NumberBase) -> bool
{
	match base
	{
		NumberBase::Decimal =>
		{
			c.chars().all(|c| c.is_digit(10))
		}
		NumberBase::Hexadecimal =>
		{
			c.chars().all(|c| c.is_digit(16))
		}
		NumberBase::Binary => c.chars().all(|c| c.is_digit(2)),
		NumberBase::Octal => c.chars().all(|c| c.is_digit(8)),
	}
}

impl Lexer
{
	/// Creates a new `Lexer`.
	///
	/// # Arguments
	///
	/// * `source` - The input source code.
	/// * `global_config` - The configuration of the lexer.
	///
	/// # Returns
	///
	/// A new `Lexer` instance.
	fn new(source_id: SmolStr, source_code: String) -> Lexer
	{
		Lexer {
			source_id,
			graphemes: UnicodeSegmentation::grapheme_indices(
				source_code.as_str(),
				true,
			)
			.map(|(i, s)| (i, s.to_owned()))
			.collect(),
			source_code,
			start: Position::default(),
			current: Position::default(),
			previous_line_column_count: 0,
		}
	}

	/// Tokenize the given source into a list of tokens.
	///
	/// # Arguments
	///
	/// * `source` - The input source code.
	/// * `global_config` - The configuration of the lexer.
	///
	/// # Returns
	///
	/// A list of tokens or a LexerError.
	pub fn tokenize(
		source_id: SmolStr,
		source_code: String,
	) -> LexerResult<Vec<Token>>
	{
		let mut lexer = Lexer::new(source_id, source_code);
		lexer.tokenize_internal()
	}

	/// Tokenize the source code into a list of tokens.
	///
	/// # Returns
	///
	/// A list of tokens or a LexerError.
	fn tokenize_internal(&mut self)
	-> LexerResult<Vec<Token>>
	{
		let mut tokens: Vec<Option<Token>> = Vec::new();

		while !self.is_eoi()
		{
			tokens.push(self.scan_token()?);
		}
		tokens.push(self.new_token(TokenKind::EndOfInput));

		let tokens = tokens
			.into_iter()
			.filter(|t| t.is_some())
			.map(|t| t.unwrap())
			.collect::<Vec<Token>>();

		Ok(tokens)
	}

	/// Check if the lexer is at the end of the input.
	///
	/// # Returns
	///
	/// True if the lexer is at the end of the input, false
	/// otherwise.
	fn is_eoi(&self) -> bool
	{
		self.current.char_index >= self.graphemes.len()
	}

	/// Get the position from nth previous character.
	///
	/// # Arguments
	///
	/// * `n` - The number of characters to go back.
	///
	/// # Returns
	///
	/// The position of the nth previous character.
	#[allow(dead_code)] // will use later
	fn get_previous_position(&self, n: usize) -> Position
	{
		let mut position = self.current.clone();
		for _ in 0 .. n
		{
			let c = self
				.graphemes
				.get(position.char_index - 1)
				.unwrap()
				.1
				.clone();

			if is_newline(c.clone())
			{
				position.line -= 1;
				position.column = self.previous_line_column_count;
			}
			else
			{
				position.column -= 1;
			}
			position.char_index -= 1;
			position.offset -= c.len();
		}
		position
	}

	/// Scan the next token.
	///
	/// # Returns
	///
	/// The next token or a LexerError.
	fn scan_token(&mut self) -> LexerResult<Option<Token>>
	{
		self.skip_skipable();
		self.start = self.current;

		let c = self.advance();

		// identifier or reserved keyword
		if c
			.chars()
			.all(|rust_char| UnicodeXID::is_xid_start(rust_char))
		{
			return self.create_reserved_or_identifier_token();
		}

		// number
		if c.chars().all(|c| c.is_digit(10))
		{
			return self.create_number_token();
		}

		match c.as_str()
		{
			"+" => Ok(self.new_token(TokenKind::Add)),
			"-" =>
			{
				Ok(self.new_token(TokenKind::SubtractOrNegate))
			}
			"*" =>
			{
				if self.match_and_consume("*")
				{
					Ok(self.new_token(TokenKind::Exponent))
				}
				else
				{
					Ok(self.new_token(TokenKind::Multiply))
				}
			}
			"/" =>
			{
				if self.match_and_consume("/")
				{
					self.create_single_line_comment_token()
				}
				else
				{
					Ok(self.new_token(TokenKind::Divide))
				}
			}
			"%" => Ok(self.new_token(TokenKind::Modulo)),
			"(" => Ok(self.new_token(TokenKind::LeftParen)),
			")" => Ok(self.new_token(TokenKind::RightParen)),
			";" => Ok(self.new_token(TokenKind::SemiColon)),
			":" => Ok(self.new_token(TokenKind::Colon)),
			" " =>
			{
				while !self.is_eoi()
					&& self.peek().map_or(false, |c| c == " ")
				{
					self.advance();
				}
				// the whitespace is collected only it's at the
				// start of the line and the line is not empty
				if self.start.column == 1
					&& self.peek().map_or(false, |c| !is_newline(c))
				{
					Ok(self.new_token(TokenKind::Whitespace))
				}
				else
				{
					Ok(None)
				}
			}
			"\t" =>
			{
				while !self.is_eoi()
					&& self.peek().map_or(false, |c| c == "\t")
				{
					self.advance();
				}
				// the tab is collected only it's at the
				// start of the line and the line is not empty
				if self.start.column == 1
					&& self.peek().map_or(false, |c| !is_newline(c))
				{
					Ok(self.new_token(TokenKind::Tab))
				}
				else
				{
					Ok(None)
				}
			}
			_ if is_newline(c.clone()) =>
			{
				Ok(self.new_token(TokenKind::NewLine))
			}
			_ => Err(LexerError {
				code: LexerErrorCode::UnexpectedCharacter,
				message: t!(
					"lexer-error-unexpected-character",
					character = json!(c).to_string()
				),
				hint: None,
				location: Location::Position(self.start),
				source_id: self.source_id.clone(),
			}),
		}
	}

	/// Create a new token.
	///
	/// # Arguments
	///
	/// * `kind` - The kind of the token.
	///
	/// # Returns
	///
	/// A new token.
	fn new_token(&self, kind: TokenKind) -> Option<Token>
	{
		Some(Token::new(
			kind,
			Span::new(self.start, self.current),
			self.source_code
				[self.start.offset .. self.current.offset]
				.to_owned()
				.into(),
			self.source_id.clone(),
		))
	}

	/// Skip the skipable characters such as whitespace except
	/// token-related characters.
	fn skip_skipable(&mut self)
	{
		while !self.is_eoi()
			&& self.peek().map_or(false, is_skipable)
		{
			self.advance();
		}
	}

	/// Peek the current character.
	///
	/// # Returns
	///
	/// The current character or None if the lexer is at the
	/// end of the input.
	fn peek(&self) -> Option<String>
	{
		self
			.graphemes
			.get(self.current.char_index)
			.map(|(_, c)| c.clone())
	}

	/// Peek the next nth character.
	/// It does not advance the lexer.
	///
	/// # Arguments
	///
	/// * `n` - The number of characters to peek.
	///
	/// # Returns
	///
	/// The nth character or None if the lexer is at the
	/// end of the input.
	#[allow(dead_code)] // will use later
	fn peek_nth(&self, n: usize) -> Option<String>
	{
		self
			.graphemes
			.get(self.current.char_index + n)
			.map(|(_, c)| c.clone())
	}

	/// Get the previous character.
	///
	/// # Returns
	///
	/// The previous character or None if the lexer is at the
	/// start of the input.
	fn previous(&self) -> Option<String>
	{
		self
			.graphemes
			.get(self.current.char_index - 1)
			.map(|(_, c)| c.clone())
	}

	/// Consume the current character if it matches the given
	/// character.
	///
	/// # Arguments
	///
	/// * `expected` - The expected character.
	///
	/// # Returns
	///
	/// True if the current character matches the expected
	/// character, false otherwise.
	fn match_and_consume(&mut self, expected: &str) -> bool
	{
		if self.peek().map_or(false, |c| c == expected)
		{
			self.advance();
			true
		}
		else
		{
			false
		}
	}

	/// Advance the lexer to the next character.
	/// It also updates the current position.
	///
	/// # Returns
	///
	/// The current character or error if the lexer is at the
	/// end of the input.
	fn advance(&mut self) -> String
	{
		let c = self.peek().unwrap();
		if is_newline(c.clone())
		{
			self.current.line += 1;
			self.previous_line_column_count = self.current.column;
			self.current.column = 1;
		}
		else
		{
			self.current.column += 1;
		}
		self.current.char_index += 1;
		self.current.offset += c.len();

		c
	}

	/// Create comment token.
	///
	/// # Returns
	///
	/// The comment token.
	fn create_single_line_comment_token(
		&mut self,
	) -> LexerResult<Option<Token>>
	{
		while !self.is_eoi()
			&& self.peek().map_or(false, |c| !is_newline(c))
		{
			self.advance();
		}
		Ok(self.new_token(TokenKind::SingleLineComment))
	}

	/// Create an identifier token.
	///
	/// # Returns
	///
	/// The identifier token.
	fn create_reserved_or_identifier_token(
		&mut self,
	) -> LexerResult<Option<Token>>
	{
		while !self.is_eoi()
			&& self.peek().map_or(false, |c| {
				c.chars().all(|rust_char| {
					UnicodeXID::is_xid_continue(rust_char)
				})
			})
		{
			self.advance();
		}

		match &self.source_code
			[self.start.offset .. self.current.offset]
		{
			"echo" => Ok(self.new_token(TokenKind::Echo)),
			_ => Err(LexerError {
				code: LexerErrorCode::UnimplementedFeature,
				message: t!(
					"lexer-error-unimplemented-feature",
					feature = "identifier"
				),
				hint: None,
				location: Location::Span(Span::new(
					self.start,
					self.current,
				)),
				source_id: self.source_id.clone().into(),
			}),
		}
	}

	/// Create a number token.
	///
	/// # Returns
	///
	/// The number token.
	fn create_number_token(
		&mut self,
	) -> LexerResult<Option<Token>>
	{
		let starting_number = self.previous().unwrap();

		match starting_number.as_str()
		{
			"0" =>
			{
				if self.match_and_consume("x")
				{
					self.consume_number(NumberBase::Hexadecimal)
				}
				else if self.match_and_consume("b")
				{
					self.consume_number(NumberBase::Binary)
				}
				else if self.match_and_consume("o")
				{
					self.consume_number(NumberBase::Octal)
				}
				else
				{
					self.consume_number(NumberBase::Decimal)
				}
			}
			_ => self.consume_number(NumberBase::Decimal),
		}
	}

	/// Check if the number width is valid.
	/// Depending on the suffix, the width of the number
	/// should be valid.
	///
	/// # Arguments
	///
	/// * `width_lexeme` - The width lexeme.
	/// * `suffix` - The suffix of the number.
	///
	/// # Returns
	///
	/// A tuple of boolean, vector of possible integer widths,
	fn validate_number_width(
		&self,
		width_lexeme: String,
		suffix: String,
	) -> (bool, Vec<&'static str>, Vec<&'static str>)
	{
		let int_possible_widths = vec!["8", "16", "32", "64"];
		let float_possible_widths = vec!["32", "64"];

		let res = match suffix.as_str()
		{
			"f" =>
			{
				width_lexeme.is_empty()
					|| float_possible_widths
						.contains(&width_lexeme.as_str())
			}
			"d" => width_lexeme.is_empty(),
			"i" | "u" =>
			{
				width_lexeme.is_empty()
					|| int_possible_widths
						.contains(&width_lexeme.as_str())
			}
			#[cfg(not(test))]
			_ => unreachable!(),
			#[cfg(test)]
			_ => false,
		};

		(res, int_possible_widths, float_possible_widths)
	}

	/// Consume the number part of the source code.
	/// This is used to tokenize integer and float literals.
	/// The lexer will consume the integer part, fractional
	/// part, exponent part,
	///
	/// # Arguments
	///
	/// * `base` - The base of the number.
	///
	/// # Returns
	///
	/// The token.
	fn consume_number(
		&mut self,
		base: NumberBase,
	) -> LexerResult<Option<Token>>
	{
		let mut has_int_part = base == NumberBase::Decimal;

		// consume the int part
		while self.peek().map_or(false, |c| is_digit(c, base))
		{
			has_int_part = true;
			self.advance();
		}

		let mut is_int = true;

		// consume the fractional part if it's base 10 and has
		// dot followed by digit
		if base == NumberBase::Decimal
			&& self.peek().map_or(false, |c| c == ".")
			&& self
				.peek_nth(1)
				.map_or(false, |c| is_digit(c, base))
		{
			is_int = false;
			// consume the dot
			self.advance();

			// consume the fractional part
			while self.peek().map_or(false, |c| is_digit(c, base))
			{
				self.advance();
			}
		}

		let mut has_expontent = false;
		// consume the exponent part if it's base 10 and has e
		// or E
		if base == NumberBase::Decimal
			&& self.peek().map_or(false, |c| c == "e" || c == "E")
		{
			is_int = false;
			has_expontent = true;

			// consume the e or E
			self.advance();

			// consume the sign
			if self.peek().map_or(false, |c| c == "+" || c == "-")
			{
				self.advance();
			}

			// consume the exponent part
			while self.peek().map_or(false, |c| is_digit(c, base))
			{
				self.advance();
			}
		}

		let mut suffix_start_pos: Option<Position> = None;
		// consume the suffix
		if self.peek().map_or(false, |c| {
			matches!(c.as_str(), "f" | "d" | "i" | "u")
		})
		{
			suffix_start_pos = Some(self.current);
			// consume the suffix
			let suffix = self.advance();

			// consume the width part like 8, 16, 32, 64
			// the maximum possible width is 64. But some
			// languages have 128 bit integers. So, let's keep it
			// 3.
			let mut width_lexeme = String::with_capacity(3);

			while self
				.peek()
				.map_or(false, |c| is_digit(c, NumberBase::Decimal))
			{
				width_lexeme.push_str(&self.advance());
			}

			let (
				valid_width,
				int_possible_widths,
				float_possible_widths,
			) = self.validate_number_width(
				width_lexeme.clone(),
				suffix.clone(),
			);

			if !valid_width
			{
				if suffix.as_str() == "d"
				{
					return Err(LexerError {
						code: LexerErrorCode::InvalidNumberLiteralWidth,
						message: t!(
							"lexer-error-invalid-number-literal-width.\
							 no-width-for-double"
						),
						hint: None,
						location: Location::Span(Span::new(
							suffix_start_pos.unwrap(),
							self.current,
						)),
						source_id: self.source_id.clone().into(),
					});
				}

				let message = t!(
					"lexer-error-invalid-number-literal-width",
					width = json!(width_lexeme).to_string(),
					literal_kind = ternary!(
						is_int,
						t!("common-data-type-desc.int"),
						t!("common-data-type-desc.float")
					),
					valid_widths = ternary!(
						is_int,
						int_possible_widths,
						float_possible_widths
					)
					.join(", ")
				);

				return Err(LexerError {
					code: LexerErrorCode::InvalidNumberLiteralWidth,
					message,
					hint: None,
					location: Location::Span(Span::new(
						suffix_start_pos.unwrap(),
						self.current,
					)),
					source_id: self.source_id.clone().into(),
				});
			}
		}

		// The suffix is not part of the decision. Only dot and
		// exponent can decide if it's float or int in lexer.
		// But later in the parser, we can decide if it's float
		// or int based on the suffix.
		if is_int
		{
			Ok(self.new_token(TokenKind::Literal {
				kind: LiteralTokenKind::Integer(
					IntegerLiteralToken {
						base,
						has_integer_part: has_int_part,
					},
				),
				suffix_start: suffix_start_pos,
			}))
		}
		else
		{
			Ok(self.new_token(TokenKind::Literal {
				kind: LiteralTokenKind::Float(FloatLiteralToken {
					base,
					has_exponent_part: has_expontent,
				}),
				suffix_start: suffix_start_pos,
			}))
		}
	}
}

#[cfg(test)]
mod tests
{
	use smol_str::ToSmolStr;
	use termcolor::Buffer;

	use super::Lexer;
	use crate::common::config::{ApplicationMode, Config};
	use crate::common::{Source, SourceOrigin};
	use crate::parser::token::TokenKind;

	#[test]
	fn test_create_and_print_diagnostic()
	{
		use super::{LexerError, LexerErrorCode, Location};
		use crate::compiler::session_globals::SessionGlobals;

		let source =
			Source::from("1+2").with_origin(SourceOrigin::String);
		let source_id = source.source_id().unwrap();

		let lexer_error = LexerError {
			code: LexerErrorCode::UnexpectedCharacter,
			message: "Unexpected character".to_owned(),
			hint: Some("Check the character".to_owned()),
			location: Location::Position(super::Position {
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

		let (diagnostic_files, file_id, diagnostic) =
			lexer_error.create_diagnostic(&mut session_globals);

		assert!(diagnostic_files.get(file_id).is_ok());
		assert_eq!(
			diagnostic.code,
			Some("UnexpectedCharacter".to_owned())
		);
		assert_eq!(
			diagnostic.message,
			"Unexpected character\n"
		);
		assert_eq!(diagnostic.labels.len(), 1);
		assert_eq!(
			diagnostic.labels[0].message,
			"Unexpected character"
		);
		assert_eq!(diagnostic.labels[0].range, 0 .. 0);
		assert_eq!(diagnostic.notes.len(), 1);

		lexer_error.print(&mut session_globals);
	}

	#[test]
	fn test_is_skipable()
	{
		assert!(!super::is_skipable(" ".to_owned()));
		assert!(!super::is_skipable("\t".to_owned()));
		assert!(!super::is_skipable("a".to_owned()));
		assert!(super::is_skipable("\r".to_owned()));
	}

	#[test]
	fn test_is_newline()
	{
		assert!(super::is_newline("\n".to_owned()));
		assert!(super::is_newline("\r\n".to_owned()));
		assert!(!super::is_newline("a".to_owned()));
	}

	#[test]
	fn test_get_previous_position()
	{
		let mut lexer =
			Lexer::new("string".into(), "+-*\n/".into());
		lexer.advance(); // +
		let position = lexer.get_previous_position(1);
		assert_eq!(position.line, 1);
		assert_eq!(position.column, 1);
		assert_eq!(position.offset, 0);

		lexer.advance(); // -
		lexer.advance(); // *
		lexer.advance(); // \n
		lexer.advance(); // /
		assert_eq!(lexer.current.line, 2);
		assert_eq!(lexer.current.column, 2);
		assert_eq!(lexer.current.offset, 5);

		let position = lexer.get_previous_position(2);
		assert_eq!(position.line, 1);
		assert_eq!(position.column, 4);
		assert_eq!(position.offset, 3);
	}

	/// Test scanning a token part.
	macro_rules! test_scan_indivitual_token {
		($source:expr, $expected_kind:expr) => {
			let tokens =
				Lexer::tokenize("string".into(), $source.into());
			assert!(tokens.is_ok());
			let tokens = tokens.unwrap();

			assert_eq!(tokens.len(), 2);
			assert_eq!(tokens[0].kind, $expected_kind);
		};
	}

	#[test]
	fn test_scan_indivitual_tokens()
	{
		test_scan_indivitual_token!("echo", TokenKind::Echo);
		test_scan_indivitual_token!("+", TokenKind::Add);
		test_scan_indivitual_token!(
			"-",
			TokenKind::SubtractOrNegate
		);
		test_scan_indivitual_token!("*", TokenKind::Multiply);
		test_scan_indivitual_token!("/", TokenKind::Divide);
		test_scan_indivitual_token!("%", TokenKind::Modulo);
		test_scan_indivitual_token!("**", TokenKind::Exponent);
		test_scan_indivitual_token!("(", TokenKind::LeftParen);
		test_scan_indivitual_token!(")", TokenKind::RightParen);
		test_scan_indivitual_token!(";", TokenKind::SemiColon);
		test_scan_indivitual_token!(":", TokenKind::Colon);
		test_scan_indivitual_token!("\n", TokenKind::NewLine);
		test_scan_indivitual_token!(
			"// hello",
			TokenKind::SingleLineComment
		);
	}

	#[test]
	fn test_whitespace_skipping()
	{
		let tokens = Lexer::tokenize(
			"string".into(),
			"    1 + 2   1 + 2".into(),
		);
		assert!(tokens.is_ok());
		let tokens = tokens.unwrap();
		// only the first whitespace should be tokenized
		assert_eq!(tokens.len(), 8);

		let tokens = Lexer::tokenize(
			"string".into(),
			"\t\t1 + 2\t\t1 + 2".into(),
		);
		assert!(tokens.is_ok());
		let tokens = tokens.unwrap();
		// only the first tab should be tokenized
		assert_eq!(tokens.len(), 8);

		let tokens =
			Lexer::tokenize("string".into(), "     ".into());
		assert!(tokens.is_ok());
		let tokens = tokens.unwrap();
		// no whitespace should be tokenized
		assert_eq!(tokens.len(), 1);

		let tokens =
			Lexer::tokenize("string".into(), "\t\t".into());
		assert!(tokens.is_ok());
		let tokens = tokens.unwrap();
		// no whitespace should be tokenized
		assert_eq!(tokens.len(), 1);
	}

	#[test]
	fn test_validate_number_width()
	{
		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("".into(), "f".into());
		assert!(valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("32".into(), "f".into());
		assert!(valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("128".into(), "f".into());
		assert!(!valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("128".into(), "d".into());
		assert!(!valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("".into(), "d".into());
		assert!(valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("32".into(), "d".into());
		assert!(!valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("".into(), "i".into());
		assert!(valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("32".into(), "i".into());
		assert!(valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("128".into(), "i".into());
		assert!(!valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("".into(), "u".into());
		assert!(valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("32".into(), "u".into());
		assert!(valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("128".into(), "u".into());
		assert!(!valid);

		let (valid, ..) =
			Lexer::new("string".into(), "".into())
				.validate_number_width("".into(), "x".into());
		assert!(!valid);
	}

	#[test]
	fn test_number_tokens()
	{
		test_scan_indivitual_token!(
			"0",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"0x",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Hexadecimal,
						has_integer_part: false,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"0b",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Binary,
						has_integer_part: false,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"0o",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Octal,
						has_integer_part: false,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"0x1234",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Hexadecimal,
						has_integer_part: true,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"0b1010",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Binary,
						has_integer_part: true,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"0o7654",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Octal,
						has_integer_part: true,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"0.0",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Float(
					super::FloatLiteralToken {
						base: super::NumberBase::Decimal,
						has_exponent_part: false,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"1e2",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Float(
					super::FloatLiteralToken {
						base: super::NumberBase::Decimal,
						has_exponent_part: true,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"1.0e-12",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Float(
					super::FloatLiteralToken {
						base: super::NumberBase::Decimal,
						has_exponent_part: true,
					},
				),
				suffix_start: None,
			}
		);

		test_scan_indivitual_token!(
			"1.0f",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Float(
					super::FloatLiteralToken {
						base: super::NumberBase::Decimal,
						has_exponent_part: false,
					},
				),
				suffix_start: Some(super::Position {
					line: 1,
					column: 4,
					offset: 3,
					char_index: 3,
				}),
			}
		);

		test_scan_indivitual_token!(
			"1f32",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: Some(super::Position {
					line: 1,
					column: 2,
					offset: 1,
					char_index: 1,
				}),
			}
		);

		test_scan_indivitual_token!(
			"1i",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: Some(super::Position {
					line: 1,
					column: 2,
					offset: 1,
					char_index: 1,
				}),
			}
		);

		test_scan_indivitual_token!(
			"1u",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: Some(super::Position {
					line: 1,
					column: 2,
					offset: 1,
					char_index: 1,
				}),
			}
		);

		test_scan_indivitual_token!(
			"1d",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: Some(super::Position {
					line: 1,
					column: 2,
					offset: 1,
					char_index: 1,
				}),
			}
		);

		test_scan_indivitual_token!(
			"1u32",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: Some(super::Position {
					line: 1,
					column: 2,
					offset: 1,
					char_index: 1,
				}),
			}
		);

		test_scan_indivitual_token!(
			"1i64",
			TokenKind::Literal {
				kind: super::LiteralTokenKind::Integer(
					super::IntegerLiteralToken {
						base: super::NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: Some(super::Position {
					line: 1,
					column: 2,
					offset: 1,
					char_index: 1,
				}),
			}
		);
	}

	#[test]
	fn test_number_with_invalid_widths()
	{
		for literal in &[
			"1f8", "1f16", "1d8", "1d16", "1d64", "1i12", "32u1",
			"12.0f60",
		]
		{
			let tokens = Lexer::tokenize(
				"string".into(),
				literal.to_owned().to_string(),
			);
			assert!(tokens.is_err());
			let error = tokens.unwrap_err();
			println!("Error: {:?}", error);
			assert_eq!(
				error.code,
				super::LexerErrorCode::InvalidNumberLiteralWidth
			);
		}
	}

	#[test]
	fn test_unimplemented_feature()
	{
		let tokens =
			Lexer::tokenize("string".into(), "id".into());
		assert!(tokens.is_err());
		let error = tokens.unwrap_err();
		assert_eq!(
			error.code,
			super::LexerErrorCode::UnimplementedFeature
		);
		assert_eq!(
			error.message,
			"Unimplemented feature: \u{2068}identifier\u{2069}"
		);
		assert_eq!(error.hint, None);
		assert_eq!(
			error.location,
			super::Location::Span(super::Span {
				start: super::Position {
					line: 1,
					column: 1,
					offset: 0,
					char_index: 0,
				},
				end: super::Position {
					line: 1,
					column: 3,
					offset: 2,
					char_index: 2,
				},
			},)
		);
		assert_eq!(error.source_id, "string".to_smolstr());
	}

	#[test]
	fn test_unrecognized_character()
	{
		let tokens =
			Lexer::tokenize("string".into(), "\\".into());
		assert!(tokens.is_err());
		let error = tokens.unwrap_err();
		assert_eq!(
			error.code,
			super::LexerErrorCode::UnexpectedCharacter
		);
		assert_eq!(
			error.message,
			"Unexpected character: \u{2068}\"\\\\\"\u{2069}"
		);
		assert_eq!(error.hint, None);
		assert_eq!(
			error.location,
			super::Location::Position(super::Position {
				line: 1,
				column: 1,
				offset: 0,
				char_index: 0,
			},)
		);
		assert_eq!(error.source_id, "string".to_smolstr());
	}

	#[test]
	fn test_skip_skipable()
	{
		let mut lexer =
			Lexer::new("string".into(), "\r\ra".into());
		lexer.skip_skipable();
		assert_eq!(lexer.current.column, 3);
	}

	#[test]
	fn test_all_supported_tokens()
	{
		let tokens = Lexer::tokenize(
			"string".into(),
			"+-***/%() \t\n// hello world\n12.0 0x1234 0b1010 \
			 0o7654 1e2 1.0e-12 1.0f 1f 1i 1u 1d 1u32 1i64"
				.into(),
		);
		assert!(tokens.is_ok());
		let tokens = tokens.unwrap();
		assert_eq!(tokens.len(), 25);
	}
}
