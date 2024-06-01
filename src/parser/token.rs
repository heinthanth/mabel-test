use smol_str::SmolStr;

use super::span::Span;

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
pub enum LiteralToken
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
	Literal(LiteralToken),
	// Arithmetic operators
	/// Addition
	Add,
	/// Subtraction
	Subtract,
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
	// Other tokens
	/// Left Parenthesis
	LeftParen,
	/// Right Parenthesis
	RightParen,
	/// Single line comment
	SingleLineComment,
	/// Whitespace sequence
	Whitespace(bool),
	/// Tab
	Tab(bool),
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
			TokenKind::Literal(LiteralToken::Integer(
				IntegerLiteralToken {
					base: NumberBase::Decimal,
					has_integer_part: true,
				},
			)),
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
			TokenKind::Literal(LiteralToken::Integer(
				IntegerLiteralToken {
					base: NumberBase::Decimal,
					has_integer_part: true,
				}
			))
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
}
