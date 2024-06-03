use smol_str::SmolStr;

use super::data_type::DataType;
use crate::parser::ast::{self, GetSpan};
use crate::parser::span::Span;

/// The annotated AST.
/// It is the AST with data type and other information.
#[derive(Debug, Clone)]
pub struct AnnotatedAst<A, D>
{
	pub inner: A,
	pub data_type: D,
}

/// Annotated module
#[derive(Debug, Clone)]
pub struct AnnotatedModule<E>
where
	E: ast::GetSpan,
{
	/// module id
	pub id: SmolStr,
	/// statements
	pub statements: Vec<AnnotatedStatement<E>>,
}

/// Annotated literal expression.
pub type AnnotatedLiteralExpr =
	AnnotatedAst<ast::LiteralExpr, DataType>;

/// Annotated group expression.
pub type AnnotatedGroupExpr<E> =
	AnnotatedAst<ast::GroupingExpr<E>, DataType>;

/// Annotated binary expression.
pub type AnnotatedBinaryExpr<E> =
	AnnotatedAst<ast::BinaryExpr<E>, DataType>;

/// Annotated unary expression.
pub type AnnotatedUnaryExpr<E> =
	AnnotatedAst<ast::UnaryExpr<E>, DataType>;

/// Annotated Expression
#[derive(Debug, Clone)]
pub enum AnnotatedExpression
{
	Literal(AnnotatedLiteralExpr),
	Group(AnnotatedGroupExpr<AnnotatedExpression>),
	Binary(AnnotatedBinaryExpr<AnnotatedExpression>),
	Unary(AnnotatedUnaryExpr<AnnotatedExpression>),
}

impl GetSpan for AnnotatedExpression
{
	fn get_span(&self) -> Option<Span>
	{
		match self
		{
			AnnotatedExpression::Literal(literal) =>
			{
				literal.inner.get_span()
			}
			AnnotatedExpression::Group(group) =>
			{
				group.inner.get_span()
			}
			AnnotatedExpression::Binary(binary) =>
			{
				binary.inner.get_span()
			}
			AnnotatedExpression::Unary(unary) =>
			{
				unary.inner.get_span()
			}
		}
	}
}

impl AnnotatedExpression
{
	/// Get the data type of the expression.
	pub fn get_data_type(&self) -> &DataType
	{
		match self
		{
			AnnotatedExpression::Literal(literal) =>
			{
				&literal.data_type
			}
			AnnotatedExpression::Group(group) => &group.data_type,
			AnnotatedExpression::Binary(binary) =>
			{
				&binary.data_type
			}
			AnnotatedExpression::Unary(unary) => &unary.data_type,
		}
	}
}

/// Annotated expression statement.
pub type AnnotatedExpressionStmt<E> =
	AnnotatedAst<ast::ExpressionStmt<E>, Option<DataType>>;

/// Annotated echo statement.
pub type AnnotatedEchoStmt<E> =
	AnnotatedAst<ast::EchoStmt<E>, Option<DataType>>;

/// Annotated function declaration.
pub type AnnotatedFunctionDeclStmt<S> =
	AnnotatedAst<ast::FunctionDeclStmt<S>, Option<DataType>>;

/// Annotated statement.
#[derive(Debug, Clone)]
pub enum AnnotatedStatement<E>
where
	E: ast::GetSpan,
{
	Expression(AnnotatedExpressionStmt<E>),
	Echo(AnnotatedEchoStmt<E>),
	FunctionDecl(
		AnnotatedFunctionDeclStmt<AnnotatedStatement<E>>,
	),
}

impl<E> GetSpan for AnnotatedStatement<E>
where
	E: ast::GetSpan,
{
	fn get_span(&self) -> Option<Span>
	{
		match self
		{
			AnnotatedStatement::Expression(expr) =>
			{
				expr.inner.get_span()
			}
			AnnotatedStatement::Echo(echo) =>
			{
				echo.inner.get_span()
			}
			AnnotatedStatement::FunctionDecl(func) =>
			{
				func.inner.get_span()
			}
		}
	}
}
