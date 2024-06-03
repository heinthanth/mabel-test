use smol_str::SmolStr;

use super::span::Span;
use super::token::Token;
use crate::compiler::data_type::DataType;
use crate::ternary;

/// Ast Visitor Trait
pub trait AstVisitor<
	SourceExprType,
	SourceStmtType,
	ModRetType,
	ExprRetType,
	StmtRetType,
> where
	SourceExprType: GetSpan,
	SourceStmtType: GetSpan,
{
	/// Visit a module node
	fn visit_module(
		&mut self,
		module: &Module<SourceExprType>,
	) -> ModRetType;
	/// Visit a statement node
	fn visit_statement(
		&mut self,
		statement: &Statement<SourceExprType>,
	) -> StmtRetType;
	/// Visit an expression statement node
	fn visit_expression_stmt(
		&mut self,
		expression_stmt: &ExpressionStmt<SourceExprType>,
	) -> StmtRetType;
	/// Visit an echo statement node
	fn visit_echo_stmt(
		&mut self,
		echo_stmt: &EchoStmt<SourceExprType>,
	) -> StmtRetType;
	/// Visit a function declaration statement node
	fn visit_function_decl_stmt(
		&mut self,
		function_decl_stmt: &FunctionDeclStmt<SourceStmtType>,
	) -> StmtRetType;
	/// Visit an expression node
	fn visit_expression(
		&mut self,
		expression: &Expression,
	) -> ExprRetType;
	/// Visit a literal expression node
	fn visit_literal_expr(
		&mut self,
		literal_expr: &LiteralExpr,
	) -> ExprRetType;
	/// Visit a grouping expression node
	fn visit_grouping_expr(
		&mut self,
		grouping_expr: &GroupingExpr<SourceExprType>,
	) -> ExprRetType;
	/// Visit a unary expression node
	fn visit_unary_expr(
		&mut self,
		unary_expr: &UnaryExpr<SourceExprType>,
	) -> ExprRetType;
	/// Visit a binary expression node
	fn visit_binary_expr(
		&mut self,
		binary_expr: &BinaryExpr<SourceExprType>,
	) -> ExprRetType;
}

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

// #[macro_export]
// macro_rules! ast_known_data_type {
// 	($t:ident) => {
// 		DataTypeNode {
// 			token: None,
// 			inner: $crate::compiler::data_type::DataType::Known(
// 				crate::compiler::data_type::KnownDataType::$t,
// 			),
// 		}
// 	};
// 	($t:ident, $token:expr) => {
// 		DataType {
// 			token: Some($token),
// 			inner: crate::compiler::data_type::DataType::Known(
// 				crate::compiler::data_type::KnownDataType::$t,
// 			),
// 		}
// 	};
// }

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
pub struct GroupingExpr<E>
where
	E: GetSpan,
{
	/// Left parenthesis
	pub left_paren_token: Option<Token>,
	/// Expression inside the grouping
	pub expression: Box<E>,
	/// Right parenthesis
	pub right_paren_token: Option<Token>,
}

/// `GetSpan` implementation for `GroupingExpr`
impl<E> GetSpan for GroupingExpr<E>
where
	E: GetSpan,
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
pub struct UnaryExpr<E>
where
	E: GetSpan,
{
	/// Operator
	pub operator: UnaryOperator,
	/// Operator token
	pub operator_token: Option<Token>,
	/// Right-hand side expression
	pub right: Box<E>,
}

/// `GetSpan` implementation for `UnaryExpr`
impl<E> GetSpan for UnaryExpr<E>
where
	E: GetSpan,
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
pub struct BinaryExpr<E>
where
	E: GetSpan,
{
	/// Left-hand side expression
	pub left: Box<E>,
	/// Operator
	pub operator: BinaryOperator,
	/// Operator token
	pub operator_token: Option<Token>,
	/// Right-hand side expression
	pub right: Box<E>,
}

/// `GetSpan` implementation for `BinaryExpr`
impl<E> GetSpan for BinaryExpr<E>
where
	E: GetSpan,
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
	Grouping(GroupingExpr<Expression>),
	/// Unary expression
	Unary(UnaryExpr<Expression>),
	/// Binary expression
	Binary(BinaryExpr<Expression>),
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
pub struct ExpressionStmt<E>
where
	E: GetSpan,
{
	/// Expression to echo
	pub expression: E,
}

/// `GetSpan` implementation for `ExpressionStmt`
impl<E> GetSpan for ExpressionStmt<E>
where
	E: GetSpan,
{
	fn get_span(&self) -> Option<Span>
	{
		self.expression.get_span()
	}
}

/// Echo statement node
#[derive(Debug, Clone)]
pub struct EchoStmt<E>
where
	E: GetSpan,
{
	/// Echo keyword
	pub echo_token: Option<Token>,
	/// Expression to echo
	pub expression: E,
}

/// `GetSpan` implementation for `EchoStmt`
impl<E> GetSpan for EchoStmt<E>
where
	E: GetSpan,
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
pub struct FunctionDeclStmt<S>
where
	S: GetSpan,
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
	pub body: Vec<Box<S>>,
}

/// `GetSpan` implementation for `FunctionDeclStmt`
impl<S> GetSpan for FunctionDeclStmt<S>
where
	S: GetSpan,
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
pub enum Statement<E>
where
	E: GetSpan,
{
	/// Expression statement
	Expression(ExpressionStmt<E>),
	/// Echo statement
	Echo(EchoStmt<E>),
	/// Function declaration statement
	FunctionDeclaration(FunctionDeclStmt<Statement<E>>),
}

impl<E> GetSpan for Statement<E>
where
	E: GetSpan,
{
	/// Get span of the statement
	fn get_span(&self) -> Option<Span>
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
pub struct Module<E>
where
	E: GetSpan,
{
	pub id: SmolStr,
	/// Statements in the module
	pub statements: Vec<Box<Statement<E>>>,
}

/// `GetSpan` implementation for `Module`
impl<E> GetSpan for Module<E>
where
	E: GetSpan,
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
