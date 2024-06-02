use serde_json::json;
use smol_str::SmolStr;

use super::span::{Position, Span};
use crate::t;

/// Number bases that we can parse.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NumberBase
{
	/// Binary
	Binary,
	/// Octal
	Octal,
	/// Decimal
	Decimal,
	/// Hexadecimal
	Hexadecimal,
}

/// Integer literal token.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct IntegerLiteralToken
{
	/// Base of the number.
	pub base: NumberBase,
	/// Status if the integer has integer part.
	/// For example, some integer doesn't have integer part
	/// like `0x`, `0b`, `0ou8`. But they're valid integer.
	pub has_integer_part: bool,
}

/// Float literal token.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FloatLiteralToken
{
	/// Base of the number.
	pub base: NumberBase,
	/// Status if the float has exponent part.
	pub has_exponent_part: bool,
}

/// Literal token.
/// This is a simple enum that represents the different
/// kinds of literals that our parser will recognize.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LiteralTokenKind
{
	/// Integer literal
	Integer(IntegerLiteralToken),
	/// Float literal
	Float(FloatLiteralToken),
}

/// Possible token kinds.
/// This is a simple enum that represents the different
/// kinds of tokens that our parser will recognize.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind
{
	/// Literals
	Literal
	{
		kind: LiteralTokenKind,
		suffix_start: Option<Position>,
	},
	/// Identifiers
	Identifier,
	// Arithmetic operators
	/// Addition
	Add,
	/// Subtraction
	SubtractOrNegate,
	/// Multiplication
	Multiply,
	/// Division
	Divide,
	/// Modulus
	Modulo,
	/// Exponent
	Exponent,
	// other statements' tokens
	/// Echo
	Echo,
	/// Function
	Function,
	// Other tokens
	/// Semicolon
	SemiColon,
	/// Colon
	Colon,
	/// Left Parenthesis
	LeftParen,
	/// Right Parenthesis
	RightParen,
	/// Single line comment
	SingleLineComment,
	/// Whitespace sequence
	Whitespace,
	/// Tab
	Tab,
	/// New Line
	NewLine,
	/// EOI
	EndOfInput,
}

/// Token.
/// This is a simple struct that represents a token.
#[derive(Debug, Clone)]
pub struct Token
{
	/// Kind of the token.
	pub kind: TokenKind,
	/// Span of the token.
	pub span: Span,
	/// lexeme of the token.
	pub lexeme: SmolStr,
	/// source id of the token for debugging purposes.
	/// It's in the format such as `mabel://stdin`, `mabel://REPL` or `file://path`.
	pub source_id: SmolStr,
}

/// Implementation of `Token`.
impl Token
{
	/// Creates a new `Token`.
	pub fn new(
		kind: TokenKind,
		span: Span,
		lexeme: SmolStr,
		source_id: SmolStr,
	) -> Token
	{
		Token {
			kind,
			span,
			lexeme,
			source_id,
		}
	}

	/// Returns the human readable description of the token.
	pub fn description(
		&self,
		count: usize,
		capitalization: &str,
		value: Option<String>,
		show_count: bool,
		show_value: bool,
	) -> String
	{
		let key = match self.kind
		{
			TokenKind::Literal { kind, .. } => match kind
			{
				LiteralTokenKind::Integer(_) =>
				{
					"token-description-int"
				}
				LiteralTokenKind::Float(_) =>
				{
					"token-description-float"
				}
			},
			TokenKind::Identifier =>
			{
				"token-description-identifier"
			}
			TokenKind::Add
			| TokenKind::SubtractOrNegate
			| TokenKind::Multiply
			| TokenKind::Divide
			| TokenKind::Modulo
			| TokenKind::Exponent => "token-description-operator",
			TokenKind::Echo | TokenKind::Function =>
			{
				"token-description-keyword"
			}
			TokenKind::SemiColon => "token-description-semicolon",
			TokenKind::Colon => "token-description-colon",
			TokenKind::LeftParen =>
			{
				"token-description-left-paren"
			}
			TokenKind::RightParen =>
			{
				"token-description-right-paren"
			}
			TokenKind::SingleLineComment =>
			{
				"token-description-single-line-comment"
			}
			TokenKind::Whitespace =>
			{
				"token-description-whitespace"
			}
			TokenKind::Tab => "token-description-tab",
			TokenKind::NewLine => "token-description-newline",
			TokenKind::EndOfInput => "token-description-eoi",
		};

		t!(
			key,
			value = json!(
				value.unwrap_or_else(|| self.lexeme.to_string())
			)
			.to_string(),
			capitalization = capitalization,
			count = count,
			show_count = show_count.to_string(),
			show_value = show_value.to_string()
		)
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use crate::parser::span::Position;

	#[test]
	fn test_token_new()
	{
		let token = Token::new(
			TokenKind::Literal {
				kind: LiteralTokenKind::Integer(
					IntegerLiteralToken {
						base: NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: None,
			},
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("1"),
			SmolStr::new("mabel://stdin"),
		);

		assert_eq!(
			token.kind,
			TokenKind::Literal {
				kind: LiteralTokenKind::Integer(
					IntegerLiteralToken {
						base: NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: None,
			}
		);
		assert_eq!(
			token.span,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				}
			)
		);
		assert_eq!(token.lexeme, SmolStr::new("1"));
		assert_eq!(
			token.source_id,
			SmolStr::new("mabel://stdin")
		);
	}

	#[test]
	fn test_token_description()
	{
		let token = Token::new(
			TokenKind::Literal {
				kind: LiteralTokenKind::Integer(
					IntegerLiteralToken {
						base: NumberBase::Decimal,
						has_integer_part: true,
					},
				),
				suffix_start: None,
			},
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("1"),
			SmolStr::new("mabel://stdin"),
		);

		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"an integer literal"
		);

		let token = Token::new(
			TokenKind::Literal {
				kind: LiteralTokenKind::Float(
					FloatLiteralToken {
						base: NumberBase::Decimal,
						has_exponent_part: false,
					},
				),
				suffix_start: None,
			},
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("1.0"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a float literal"
		);

		let token = Token::new(
			TokenKind::Identifier,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("a"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"an identifier"
		);

		let token = Token::new(
			TokenKind::Add,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("+"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"an operator"
		);

		let token = Token::new(
			TokenKind::Echo,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("echo"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a keyword"
		);

		let token = Token::new(
			TokenKind::SemiColon,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new(";"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a semicolon"
		);

		let token = Token::new(
			TokenKind::Colon,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new(":"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a colon"
		);

		let token = Token::new(
			TokenKind::LeftParen,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("("),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a left parenthesis"
		);

		let token = Token::new(
			TokenKind::RightParen,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new(")"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a right parenthesis"
		);

		let token = Token::new(
			TokenKind::SingleLineComment,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("//"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a single-line comment"
		);

		let token = Token::new(
			TokenKind::Whitespace,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new(" "),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a whitespace"
		);

		let token = Token::new(
			TokenKind::Tab,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 2,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("\t"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a tab"
		);

		let token = Token::new(
			TokenKind::NewLine,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 2,
					column: 1,
					char_index: 1,
					offset: 1,
				},
			),
			SmolStr::new("\n"),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"a new line"
		);

		let token = Token::new(
			TokenKind::EndOfInput,
			Span::new(
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
				Position {
					line: 1,
					column: 1,
					char_index: 0,
					offset: 0,
				},
			),
			SmolStr::new(""),
			SmolStr::new("mabel://stdin"),
		);
		assert_eq!(
			token.description(1, "lowercase", None, false, false),
			"the end of input"
		);
	}
}
