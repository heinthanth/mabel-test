use std::io::BufRead;
use std::ops::Range;

use ast::Expression;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use smol_str::{SmolStr, ToSmolStr};
use span::{Location, Position};
use termcolor::WriteColor;
use token::{
	FloatLiteralToken,
	IntegerLiteralToken,
	LiteralTokenKind,
	NumberBase,
	Token,
	TokenKind,
};

use crate::compiler::session_globals::SessionGlobals;
use crate::parser::ast::GetSpan;
use crate::ternary;

pub mod ast;
pub mod lexer;
pub mod span;
pub mod token;

/// Parser error code
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserErrorCode
{
	/// Expected an expression
	ExpectedExpression,
}

/// Parser error
#[derive(Debug, Clone)]
pub struct ParserError
{
	/// The error code
	pub code: ParserErrorCode,
	/// The error message
	pub message: String,
	/// Hint for the error
	pub hint: Option<String>,
	/// The position of the error
	pub location: Location,
	/// The source id of the error
	pub source_id: SmolStr,
}

/// Implementation of `ParserError`
impl ParserError
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

/// Parser result
pub type ParserResult<T> = Result<T, ParserError>;

pub struct Parser
{
	/// The source code id
	source_id: SmolStr,
	/// Tokens to parse
	tokens: Vec<Token>,
	/// Current token index
	current: usize,
	/// Is main module
	is_main_module: bool,
}

// Grammar
//
// module = stmts EOI
// stmts = stmt? ~ (NEWLINE* ~ stmt)* ~ NEWLINE?
// stmt = simple_stmts
//
// simple_stmts = simple_stmt ~ (SEMICOLON ~ simple_stmt)* ~
// SEMICOLON? simple_stmt = echo_stmt | expression_stmt
//
// echo_stmt = ECHO ~ expr
// expression_stmt = expr
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	FromPrimitive,
)]
#[repr(u8)]
enum Precedence
{
	None,
	Term,
	Factor,
	Unary,
	Exponent,
	Primary,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Associativity
{
	None,
	Left,
	Right,
}

struct ParseRule
{
	prefix_fn: Option<
		fn(&mut Parser) -> ParserResult<ast::Expression>,
	>,
	infix_fn: Option<
		fn(
			&mut Parser,
			ast::Expression,
		) -> ParserResult<ast::Expression>,
	>,
	precedence: Precedence,
	associativity: Associativity,
}

impl Parser
{
	/// Creates a new `Parser`.
	fn new(
		tokens: Vec<Token>,
		source_id: SmolStr,
		is_main_module: bool,
	) -> Parser
	{
		Parser {
			source_id,
			tokens,
			is_main_module,
			current: 0,
		}
	}

	/// Parses the tokens into a module node.
	///
	/// # Arguments
	///
	/// * source_id - The source id of the module.
	/// * tokens - The tokens to parse.
	///
	/// # Returns
	///
	/// The parsed module node.
	pub fn parse(
		source_id: SmolStr,
		is_main_module: bool,
		tokens: Vec<Token>,
	) -> ParserResult<ast::Module<Expression>>
	{
		let mut parser =
			Parser::new(tokens, source_id, is_main_module);
		parser.parse_module()
	}

	/// Parses a module.
	fn parse_module(
		&mut self,
	) -> ParserResult<ast::Module<Expression>>
	{
		// eliminate leading new lines
		self.consume_newlines();

		let statements = self.parse_statements()?;
		if self.is_main_module
		{
			// first statement span
			let first_stmt_span = statements
				.first()
				.map(|stmt| stmt.get_span())
				.flatten();

			// main function
			let func = ast::FunctionDeclStmt {
				function_token: Some(Token {
					kind: TokenKind::Function,
					span: first_stmt_span.unwrap_or_default(),
					lexeme: "func".to_smolstr(),
					source_id: self.source_id.clone(),
				}),
				name: Some(Token {
					kind: TokenKind::Identifier,
					span: Default::default(),
					lexeme: "main".to_smolstr(),
					source_id: self.source_id.clone(),
				}),
				left_paren_token: None,
				parameters: Vec::new(),
				right_paren_token: None,
				body: statements,
			};
			Ok(ast::Module {
				id: self.source_id.clone(),
				statements: vec![Box::new(
					ast::Statement::FunctionDeclaration(func),
				)],
			})
		}
		else
		{
			Ok(ast::Module {
				id: self.source_id.clone(),
				statements,
			})
		}
	}

	/// Check if the parser is at the end of the tokens.
	fn is_eoi(&self) -> bool
	{
		self.peek().kind == TokenKind::EndOfInput
	}

	/// Parses statements.
	fn parse_statements(
		&mut self,
	) -> ParserResult<Vec<Box<ast::Statement<Expression>>>>
	{
		let mut statements = Vec::new();
		while !self.is_eoi()
		{
			let statement = self.parse_statement()?;
			statements.extend(statement);
		}
		Ok(statements)
	}

	/// Consume many NewLine tokens.
	fn consume_newlines(&mut self)
	{
		while self.peek().kind == TokenKind::NewLine
		{
			self.advance();
		}
	}

	/// Parses a statement.
	fn parse_statement(
		&mut self,
	) -> ParserResult<Vec<Box<ast::Statement<Expression>>>>
	{
		let stmts = self.parse_simple_stmts()?;
		// new line acts like a statement terminator
		self.consume_newlines();
		Ok(stmts)
	}

	/// Parses simple statements.
	fn parse_simple_stmts(
		&mut self,
	) -> ParserResult<Vec<Box<ast::Statement<Expression>>>>
	{
		let stmt = self.parse_simple_stmt()?;
		Ok(vec![stmt])
	}

	/// Parses a simple statement.
	fn parse_simple_stmt(
		&mut self,
	) -> ParserResult<Box<ast::Statement<Expression>>>
	{
		if self.match_and_consume(TokenKind::Echo)
		{
			self.parse_echo_stmt()
		}
		else
		{
			self.parse_expression_stmt()
		}
	}

	/// Check if the current token is of the given kind.
	/// If it is, consume the token and return true.
	/// Otherwise, return false.
	///
	/// # Arguments
	///
	/// * kind - The token kind to match.
	///
	/// # Returns
	///
	/// True if the current token is of the given kind, false
	/// otherwise.
	fn match_and_consume(&mut self, kind: TokenKind) -> bool
	{
		if self.tokens[self.current].kind == kind
		{
			self.current += 1;
			true
		}
		else
		{
			false
		}
	}

	/// Advances the parser by one token.
	/// Returns the current token.
	///
	/// # Returns
	///
	/// The current token.
	fn advance(&mut self) -> Token
	{
		self.current += 1;
		self.tokens[self.current - 1].clone()
	}

	/// Returns the previous token.
	fn previous(&self) -> Token
	{
		self.tokens[self.current - 1].clone()
	}

	/// Peek the current token.
	fn peek(&self) -> Token
	{
		self.tokens[self.current].clone()
	}

	/// Parses an echo statement.
	fn parse_echo_stmt(
		&mut self,
	) -> ParserResult<Box<ast::Statement<Expression>>>
	{
		let echo_token = self.previous();
		let expression = self.parse_expression()?;

		Ok(Box::new(ast::Statement::Echo(ast::EchoStmt {
			echo_token: Some(echo_token),
			expression,
		})))
	}

	/// Parses an expression statement.
	fn parse_expression_stmt(
		&mut self,
	) -> ParserResult<Box<ast::Statement<Expression>>>
	{
		let expression = self.parse_expression()?;

		Ok(Box::new(ast::Statement::Expression(
			ast::ExpressionStmt { expression },
		)))
	}

	/// Parses an expression.
	fn parse_expression(
		&mut self,
	) -> ParserResult<ast::Expression>
	{
		self.pratt_parse(Precedence::Term)
	}

	/// Pratt parsing function
	/// Parses an expression with the given precedence and
	/// associativity.
	///
	/// # Arguments
	///
	/// * precedence - The precedence of the expression.
	///
	/// # Returns
	///
	/// The parsed expression or an error.
	fn pratt_parse(
		&mut self,
		precedence: Precedence,
	) -> ParserResult<ast::Expression>
	{
		let lhs_token = self.advance();
		let prefix_rule = self.get_parse_rule(lhs_token.kind);

		if prefix_rule.prefix_fn.is_none()
		{
			return Err(ParserError {
				code: ParserErrorCode::ExpectedExpression,
				message: format!(
					"expected an expression, found {}",
					lhs_token.description(
						1,
						"lowercase",
						None,
						true,
						true
					)
				),
				hint: None,
				location: Location::Span(lhs_token.span.clone()),
				source_id: self.source_id.clone(),
			});
		}

		let mut lhs = prefix_rule.prefix_fn.unwrap()(self)?;

		while precedence
			<= self
				.get_parse_rule(self.tokens[self.current].kind)
				.precedence
		{
			let token = self.advance();
			let infix_rule = self.get_parse_rule(token.kind);
			lhs = infix_rule.infix_fn.unwrap()(self, lhs)?;
		}

		Ok(lhs)
	}

	/// Gets the parse rule for the given token kind.
	///
	/// # Arguments
	///
	/// * kind - The token kind.
	///
	/// # Returns
	///
	/// The parse rule for the token kind.
	fn get_parse_rule(&self, kind: TokenKind) -> ParseRule
	{
		match kind
		{
			TokenKind::Add => ParseRule {
				prefix_fn: None,
				infix_fn: Some(Parser::parse_binary_expr),
				precedence: Precedence::Term,
				associativity: Associativity::Left,
			},
			TokenKind::SubtractOrNegate => ParseRule {
				prefix_fn: Some(Parser::parse_unary_expr),
				infix_fn: Some(Parser::parse_binary_expr),
				precedence: Precedence::Term,
				associativity: Associativity::Left,
			},
			TokenKind::Multiply => ParseRule {
				prefix_fn: None,
				infix_fn: Some(Parser::parse_binary_expr),
				precedence: Precedence::Factor,
				associativity: Associativity::Left,
			},
			TokenKind::Divide => ParseRule {
				prefix_fn: None,
				infix_fn: Some(Parser::parse_binary_expr),
				precedence: Precedence::Factor,
				associativity: Associativity::Left,
			},
			TokenKind::Exponent => ParseRule {
				prefix_fn: None,
				infix_fn: Some(Parser::parse_binary_expr),
				precedence: Precedence::Exponent,
				associativity: Associativity::Right,
			},
			TokenKind::Modulo => ParseRule {
				prefix_fn: None,
				infix_fn: Some(Parser::parse_binary_expr),
				precedence: Precedence::Factor,
				associativity: Associativity::Left,
			},
			TokenKind::LeftParen => ParseRule {
				prefix_fn: Some(Parser::parse_grouping_expr),
				infix_fn: None,
				precedence: Precedence::Primary,
				associativity: Associativity::None,
			},
			TokenKind::Literal { .. } => ParseRule {
				prefix_fn: Some(Parser::parse_literal_expr),
				infix_fn: None,
				precedence: Precedence::Primary,
				associativity: Associativity::None,
			},
			_ => ParseRule {
				prefix_fn: None,
				infix_fn: None,
				precedence: Precedence::None,
				associativity: Associativity::None,
			},
		}
	}

	/// Parses a grouping expression.
	fn parse_grouping_expr(
		&mut self,
	) -> ParserResult<ast::Expression>
	{
		let left_paren_token = self.previous();
		let expression = Box::new(self.parse_expression()?);
		let right_paren_token = self.advance();

		Ok(ast::Expression::Grouping(ast::GroupingExpr {
			left_paren_token: Some(left_paren_token),
			expression,
			right_paren_token: Some(right_paren_token),
		}))
	}

	/// Parses a unary expression.
	fn parse_unary_expr(
		&mut self,
	) -> ParserResult<ast::Expression>
	{
		let operator_token = self.previous();
		let operator = match operator_token.kind
		{
			TokenKind::SubtractOrNegate =>
			{
				ast::UnaryOperator::Negate
			}
			_ => panic!(
				"unexpected unary operator: {:?}",
				operator_token
			),
		};
		let right =
			Box::new(self.pratt_parse(Precedence::Unary)?);

		Ok(ast::Expression::Unary(ast::UnaryExpr {
			operator,
			operator_token: Some(operator_token),
			right,
		}))
	}

	/// Parses a binary expression.
	fn parse_binary_expr(
		&mut self,
		left: ast::Expression,
	) -> ParserResult<ast::Expression>
	{
		let operator_token = self.previous();
		let operator = match operator_token.kind
		{
			TokenKind::Add => ast::BinaryOperator::Add,
			TokenKind::SubtractOrNegate =>
			{
				ast::BinaryOperator::Subtract
			}
			TokenKind::Multiply => ast::BinaryOperator::Multiply,
			TokenKind::Divide => ast::BinaryOperator::Divide,
			TokenKind::Exponent => ast::BinaryOperator::Exponent,
			TokenKind::Modulo => ast::BinaryOperator::Modulo,
			_ => panic!(
				"unexpected binary operator: {:?}",
				operator_token
			),
		};

		let rule = self.get_parse_rule(operator_token.kind);
		let precedence = rule.precedence;
		let associativity = rule.associativity;

		let right = Box::new(self.pratt_parse(
			if associativity == Associativity::Right
			{
				precedence
			}
			else
			{
				FromPrimitive::from_u8(precedence as u8 + 1)
					.unwrap()
			},
		)?);

		Ok(ast::Expression::Binary(ast::BinaryExpr {
			left: Box::new(left),
			operator,
			operator_token: Some(operator_token),
			right,
		}))
	}

	/// Parses a literal expression.
	fn parse_literal_expr(
		&mut self,
	) -> ParserResult<ast::Expression>
	{
		let literal = self.previous();
		match literal.clone().kind
		{
			TokenKind::Literal {
				kind: LiteralTokenKind::Integer(integer),
				suffix_start,
			} => self.parse_integer_literal_expr(
				literal,
				integer,
				suffix_start,
			),
			TokenKind::Literal {
				kind: LiteralTokenKind::Float(float),
				suffix_start,
			} => self.parse_float_literal_expr(
				literal,
				float,
				suffix_start,
			),
			_ => panic!(
				"unexpected literal kind: {:?}",
				literal.kind
			),
		}
	}

	/// Parse suffix string
	///
	/// # Arguments
	///
	/// * suffix - The suffix string.
	///
	/// # Returns
	///
	/// The parsed suffix and width.
	fn parse_suffix_string(&self, suffix: &str)
	-> (char, u8)
	{
		let first_char = suffix.chars().next().unwrap();

		match first_char
		{
			'u' | 'i' =>
			{
				if suffix.len() == 1
				{
					return (first_char, 0);
				}
				let width = suffix[1 ..].parse().unwrap();
				(suffix.chars().next().unwrap(), width)
			}
			'f' =>
			{
				if suffix.len() == 1
				{
					return ('f', 32);
				}
				let width = suffix[1 ..].parse().unwrap();
				('f', width)
			}
			'd' => ('d', 64),
			_ => panic!("unexpected suffix: {}", suffix),
		}
	}

	/// Parse an integer literal expression.
	///
	/// # Arguments
	///
	/// * parent_token - The parent token.
	/// * integer - The integer literal token.
	/// * suffix_start - The suffix start position.
	///
	/// # Returns
	///
	/// The parsed integer literal expression or an error.
	fn parse_integer_literal_expr(
		&self,
		parent_token: Token,
		integer: IntegerLiteralToken,
		suffix_start: Option<Position>,
	) -> ParserResult<ast::Expression>
	{
		// 100 + 0x12u32
		// 2nd lexeme length: 7
		// suffix start: 10
		// suffix start relative: 4

		let raw_number_lexeme = parent_token.clone().lexeme;
		let suffix_start_relative =
			suffix_start.map(|suffix_start| {
				suffix_start.offset - parent_token.span.start.offset
			});

		// remove the suffix from the lexeme
		let number_lexeme = raw_number_lexeme
			[.. suffix_start_relative
				.unwrap_or_else(|| raw_number_lexeme.len())]
			.to_smolstr();

		// get the suffix lexeme
		let suffix = raw_number_lexeme
			[suffix_start_relative
				.unwrap_or_else(|| raw_number_lexeme.len()) ..]
			.to_smolstr();

		let parsed_suffix = suffix_start_relative
			.map_or(('i', 0), |_| {
				self.parse_suffix_string(suffix.as_str())
			});

		// parse the number with u64 since it's enough to hold
		// all possible integer values
		let number_lexeme_without_prefix = ternary!(
			integer.base == NumberBase::Decimal,
			number_lexeme.as_str(),
			&number_lexeme.as_str()[2 ..]
		);

		match parsed_suffix
		{
			('i', 0) =>
			{
				// 0 means platform dependent integer
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Int(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('i', 8) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Int8(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('i', 16) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Int16(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('i', 32) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Int32(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('i', 64) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Int64(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('u', 0) =>
			{
				// 0 means platform dependent unsigned integer
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::UInt(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('u', 8) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::UInt8(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('u', 16) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::UInt16(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('u', 32) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::UInt32(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('u', 64) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::UInt64(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('f', 32) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Float32(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('f', 64) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Double(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('d', 64) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Double(
						number_lexeme_without_prefix.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			_ => panic!("unexpected suffix: {:?}", parsed_suffix),
		}
	}

	/// Parse a float literal expression.
	///
	/// # Arguments
	///
	/// * parent_token - The parent token.
	/// * float - The float literal token.
	/// * suffix_start - The suffix start position.
	///
	/// # Returns
	///
	/// The parsed float literal expression or an error.
	fn parse_float_literal_expr(
		&self,
		parent_token: Token,
		_float: FloatLiteralToken,
		suffix_start: Option<Position>,
	) -> ParserResult<ast::Expression>
	{
		let raw_number_lexeme = parent_token.clone().lexeme;

		// remove the suffix from the lexeme
		let number_lexeme = suffix_start.map_or(
			raw_number_lexeme.clone(),
			|suffix_start| {
				raw_number_lexeme[.. suffix_start.offset].into()
			},
		);

		// get the suffix lexeme
		let suffix = suffix_start.map(|suffix_start| {
			raw_number_lexeme[suffix_start.offset ..].to_smolstr()
		});

		let parsed_suffix = suffix
			.map_or(('f', 64), |suffix| {
				self.parse_suffix_string(suffix.as_str())
			});

		match parsed_suffix
		{
			('f', 32) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Float32(
						number_lexeme.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('f', 64) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Double(
						number_lexeme.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			('d', 64) =>
			{
				Ok(ast::Expression::Literal(ast::LiteralExpr {
					value: ast::Value::Double(
						number_lexeme.parse().unwrap(),
					),
					token: Some(parent_token),
				}))
			}
			_ => panic!("unexpected suffix: {:?}", parsed_suffix),
		}
	}
}
