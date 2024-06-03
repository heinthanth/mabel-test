use smol_str::SmolStr;

use super::span::Span;
use super::token::Token;
use crate::compiler::data_type::DataType;
use crate::ternary;

/// Possible values for the AST
/// This is also primitive types that Mabel could use.
/// It contains optional values because sometimes the data
/// cannot be statically analyzed and only the data
/// type is known, so the value is `None`.
#[derive(Debug, Clone)]
pub enum Value
{
	/// Unsigned 8-bit integer
	UInt8(u8),
	/// Unsigned 16-bit integer
	UInt16(u16),
	/// Unsigned 32-bit integer
	UInt32(u32),
	/// Unsigned 64-bit integer
	UInt64(u64),
	/// Signed 8-bit integer
	Int8(i8),
	/// Signed 16-bit integer
	Int16(i16),
	/// Signed 32-bit integer
	Int32(i32),
	/// Signed 64-bit integer
	Int64(i64),
	/// Platform-dependent integer
	// initially, it will be stored as 64bit but later in
	// codegen, it will be converted to the
	// platform-dependent integer
	Int(i64),
	/// Platform-dependent unsigned integer
	// initially, it will be stored as 64bit but later in
	// codegen, it will be converted to the
	// platform-dependent unsigned integer
	UInt(u64),
	/// 32-bit floating point
	Float32(f32),
	/// 64-bit floating point
	Double(f64),
}


/// Trait for getting the span of a node
pub trait GetSpan
{
	fn get_span(&self) -> Option<Span>;
}



/// Data type node
#[derive(Debug, Clone)]
pub struct DataTypeNode
{
	/// Data type token
	pub token: Option<Token>,
	/// Data type inner
	pub inner: DataType,
}

#[macro_export]
macro_rules! ast_known_data_type {
	($t:ident) => {
		DataTypeNode {
			token: None,
			inner: $crate::compiler::data_type::DataType::Known(
				crate::compiler::data_type::KnownDataType::$t,
			),
		}
	};
	($t:ident, $token:expr) => {
		DataType {
			token: Some($token),
			inner: crate::compiler::data_type::DataType::Known(
				crate::compiler::data_type::KnownDataType::$t,
			),
		}
	};
}

/// Literal expression node
#[derive(Debug, Clone)]
pub struct LiteralExpr
{
	/// Literal value
	pub value: Value,
	/// Token
	pub token: Option<Token>,
}

/// `GetSpan` implementation for `LiteralExpr`
impl GetSpan for LiteralExpr
{
	fn get_span(&self) -> Option<Span>
	{
		self.token.as_ref().map(|token| token.span)
	}
}

/// Grouping expression node
#[derive(Debug, Clone)]
pub struct GroupingExpr
{
	/// Left parenthesis
	pub left_paren_token: Option<Token>,
	/// Expression inside the grouping
	pub expression: Box<Expression>,
	/// Right parenthesis
	pub right_paren_token: Option<Token>,
}

/// `GetSpan` implementation for `GroupingExpr`
impl GetSpan for GroupingExpr
{
	fn get_span(&self) -> Option<Span>
	{
		let start = self
			.left_paren_token
			.as_ref()
			.map(|token| token.span.start);
		let end = self
			.right_paren_token
			.as_ref()
			.map(|token| token.span.end);

		ternary!(
			start.is_some() && end.is_some(),
			Some(Span {
				start: start.unwrap(),
				end: end.unwrap()
			}),
			None
		)
	}
}

/// Possible unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator
{
	/// Unary minus
	Negate,
}

/// Unary expression node
#[derive(Debug, Clone)]
pub struct UnaryExpr
{
	/// Operator
	pub operator: UnaryOperator,
	/// Operator token
	pub operator_token: Option<Token>,
	/// Right-hand side expression
	pub right: Box<Expression>,
}

/// `GetSpan` implementation for `UnaryExpr`
impl GetSpan for UnaryExpr
{
	fn get_span(&self) -> Option<Span>
	{
		let op_token = self.operator_token.clone();
		let right_span = self.right.get_span();

		if op_token.is_some() && right_span.is_some()
		{
			let start = op_token.unwrap().span.start;
			let end = right_span.unwrap().end;
			Some(Span { start, end })
		}
		else
		{
			None
		}
	}
}

/// Possible binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator
{
	/// Addition
	Add,
	/// Subtraction
	Subtract,
	/// Multiplication
	Multiply,
	/// Division
	Divide,
	/// Modulo
	Modulo,
	/// Exponentiation
	Exponent,
}

/// Binary expression node
#[derive(Debug, Clone)]
pub struct BinaryExpr
{
	/// Left-hand side expression
	pub left: Box<Expression>,
	/// Operator
	pub operator: BinaryOperator,
	/// Operator token
	pub operator_token: Option<Token>,
	/// Right-hand side expression
	pub right: Box<Expression>,
}

/// `GetSpan` implementation for `BinaryExpr`
impl GetSpan for BinaryExpr
{
	fn get_span(&self) -> Option<Span>
	{
		let left_span = self.left.get_span();
		let right_span = self.right.get_span();

		if left_span.is_some() && right_span.is_some()
		{
			let start = left_span.unwrap().start;
			let end = right_span.unwrap().end;
			Some(Span { start, end })
		}
		else
		{
			None
		}
	}
}

/// Possible expression nodes
#[derive(Debug, Clone)]
pub enum Expression
{
	/// Literal expression
	Literal(LiteralExpr),
	/// Grouping expression
	Grouping(GroupingExpr),
	/// Unary expression
	Unary(UnaryExpr),
	/// Binary expression
	Binary(BinaryExpr),
}

/// `GetSpan` implementation for `Expression`
impl GetSpan for Expression
{
	fn get_span(&self) -> Option<Span>
	{
		match self
		{
			Expression::Literal(literal) => literal.get_span(),
			Expression::Grouping(grouping) => grouping.get_span(),
			Expression::Unary(unary) => unary.get_span(),
			Expression::Binary(binary) => binary.get_span(),
		}
	}
}

/// Echo statement node
#[derive(Debug, Clone)]
pub struct EchoStmt
{
	/// Echo keyword
	pub echo_token: Option<Token>,
	/// Expression to echo
	pub expression: Expression,
}

/// `GetSpan` implementation for `EchoStmt`
impl GetSpan for EchoStmt
{
	fn get_span(&self) -> Option<Span>
	{
		let echo_token = self.echo_token.clone();
		let expression_span = self.expression.get_span();

		if echo_token.is_some() && expression_span.is_some()
		{
			let start = echo_token.unwrap().span.start;
			let end = expression_span.unwrap().end;
			Some(Span { start, end })
		}
		else
		{
			None
		}
	}
}

/// Function parameter node
#[derive(Debug, Clone)]
pub struct FunctionDeclParameter
{
	/// Parameter name
	pub identifier: Token,
	/// Parameter type
	pub data_type: DataTypeNode,
}

/// Function declaration statement node
#[derive(Debug, Clone)]
pub struct FunctionDeclStmt
{
	/// Function keyword
	pub function_token: Option<Token>,
	/// Function name
	pub name: Option<Token>,
	/// Left parenthesis
	pub left_paren_token: Option<Token>,
	/// Function parameters
	pub parameters: Vec<FunctionDeclParameter>,
	/// Right parenthesis
	pub right_paren_token: Option<Token>,
	/// Function body
	pub body: Vec<Box<Statement>>,
}

/// `GetSpan` implementation for `FunctionDeclStmt`
impl GetSpan for FunctionDeclStmt
{
	fn get_span(&self) -> Option<Span>
	{
		let fn_token = self.function_token.clone();
		let last_stmt_span = self
			.body
			.last()
			.map(|stmt| stmt.get_span())
			.flatten();

		if fn_token.is_some() && last_stmt_span.is_some()
		{
			let start = fn_token.unwrap().span.start;
			let end = last_stmt_span.unwrap().end;
			Some(Span { start, end })
		}
		else
		{
			None
		}
	}
}

/// Statement node
#[derive(Debug, Clone)]
pub enum Statement
{
	/// Expression statement
	Expression(Expression),
	/// Echo statement
	Echo(EchoStmt),
	/// Function declaration statement
	FunctionDeclaration(FunctionDeclStmt),
}

impl Statement
{
	/// Get span of the statement
	pub fn get_span(&self) -> Option<Span>
	{
		match self
		{
			Statement::Expression(expr) => expr.get_span(),
			Statement::Echo(echo) => echo.get_span(),
			Statement::FunctionDeclaration(func) =>
			{
				func.get_span()
			}
		}
	}
}

/// Module node
#[derive(Debug, Clone)]
pub struct Module
{
	pub id: SmolStr,
	/// Statements in the module
	pub statements: Vec<Box<Statement>>,
}

/// `GetSpan` implementation for `Module`
impl GetSpan for Module
{
	fn get_span(&self) -> Option<Span>
	{
		let first_stmt_span = self
			.statements
			.first()
			.map(|stmt| stmt.get_span())
			.flatten();
		let last_stmt_span = self
			.statements
			.last()
			.map(|stmt| stmt.get_span())
			.flatten();

		if first_stmt_span.is_some() && last_stmt_span.is_some()
		{
			let start = first_stmt_span.unwrap().start;
			let end = last_stmt_span.unwrap().end;
			Some(Span { start, end })
		}
		else
		{
			None
		}
	}
}
