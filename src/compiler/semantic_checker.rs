use std::io::BufRead;
use std::ops::Range;
use std::vec;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use smol_str::SmolStr;
use termcolor::WriteColor;

use super::data_type::DataType;
use super::session_globals::SessionGlobals;
use crate::parser::ast::{self, DataTypeNode, GetSpan};
use crate::parser::span::Location;
use crate::{ast_known_data_type, t};

/// Semantic Checker error code
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticCheckerErrorCode
{
	/// Invalid operand
	InvalidOperand,
}

/// Semantic Checker error
#[derive(Debug, Clone)]
pub struct SemanticCheckerError
{
	/// The error code
	pub code: SemanticCheckerErrorCode,
	/// The error message
	pub message: String,
	/// Hint for the error
	pub hint: Option<String>,
	/// The annotation labels
	pub labels: Vec<(Location, String)>,
	/// The source id of the error
	pub source_id: SmolStr,
}

/// Implementation of `SemanticCheckerError`
impl SemanticCheckerError
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

		let labels = self
			.labels
			.iter()
			.map(|(location, message)| {
				Label::primary(
					file_id,
					Into::<Range<usize>>::into(*location),
				)
				.with_message(message.clone().to_string())
			})
			.collect::<Vec<Label<_>>>();

		let diagnostic = Diagnostic::error()
			.with_code(format!("{:?}", self.code))
			.with_message(self.message.clone().to_string() + "\n")
			.with_labels(labels)
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

/// Semantic Checker result
pub type SemanticCheckerResult<T> =
	Result<T, SemanticCheckerError>;

/// Semantic Checker
pub struct SemanticChecker
{
	/// Source ID
	source_id: SmolStr,
	/// The AST to check
	module: ast::Module,
}

impl SemanticChecker
{
	/// Create a new semantic checker
	///
	/// # Arguments
	///
	/// * `source_id` - The source ID
	/// * `module` - The module to check
	///
	/// # Returns
	///
	/// The semantic checker
	fn new(source_id: SmolStr, module: ast::Module) -> Self
	{
		Self { source_id, module }
	}

	/// Check the module for semantic errors
	/// and return the result
	///
	/// # Arguments
	///
	/// * `source_id` - The source ID
	/// * `module` - The module to check
	///
	/// # Returns
	///
	/// `void`
	///
	/// # Errors
	///
	/// If there is a semantic error, it will return an error
	pub fn check(
		source_id: SmolStr,
		module: ast::Module,
	) -> SemanticCheckerResult<()>
	{
		let checker = Self::new(source_id, module);
		checker.check_module()
	}

	/// Visit and check the module node
	/// for semantic errors
	fn check_module(&self) -> SemanticCheckerResult<()>
	{
		for statement in &self.module.statements
		{
			self.check_statement(statement)?;
		}
		Ok(())
	}

	/// Visit and check the statement node
	/// for semantic errors
	fn check_statement(
		&self,
		statement: &ast::Statement,
	) -> SemanticCheckerResult<()>
	{
		match statement
		{
			ast::Statement::Expression(expr) =>
			{
				self.check_expression(expr)?;
			}
			ast::Statement::FunctionDeclaration(func) =>
			{
				self.check_function_declaration(func)?;
			}
			_ => unimplemented!(),
		}
		Ok(())
	}

	/// Visit and check the function declaration node
	/// for semantic errors
	fn check_function_declaration(
		&self,
		function: &ast::FunctionDeclStmt,
	) -> SemanticCheckerResult<()>
	{
		for statement in &function.body
		{
			self.check_statement(statement)?;
		}
		Ok(())
	}

	/// Visit and check the expression node
	/// for semantic errors
	fn check_expression(
		&self,
		expression: &ast::Expression,
	) -> SemanticCheckerResult<DataTypeNode>
	{
		match expression
		{
			ast::Expression::Literal(literal) =>
			{
				self.check_literal_expr(literal)
			}
			ast::Expression::Grouping(grouping) =>
			{
				self.check_expression(&grouping.expression)
			}
			ast::Expression::Unary(unary) =>
			{
				self.check_unary_expr(&unary)
			}
			ast::Expression::Binary(binary) =>
			{
				self.check_binary_expr(&binary)
			}
		}
	}

	/// Check the operands for unary negate
	///
	/// # Arguments
	///
	/// * `unary` - The unary expression
	/// * `rhs` - The right-hand side data type
	///
	/// # Returns
	///
	/// The data type of the unary expression
	///
	/// # Errors
	///
	/// If the operands are invalid, it will return an error
	fn check_unary_negate_operands(
		&self,
		unary: &ast::UnaryExpr,
		rhs: &DataTypeNode,
	) -> SemanticCheckerResult<DataTypeNode>
	{
		// only signed integer and floating point are allowed
		if !rhs.inner.is_floating_point()
			|| !rhs.inner.is_signed_integer()
		{
			let mut labels: Vec<(Location, String)> = vec![];

			let operator_description = unary
				.operator_token
				.as_ref()
				.map(|token| {
					token.description(
						1,
						"lowercase",
						None,
						false,
						true,
					)
				})
				.unwrap_or_else(|| {
					t!(
						"semantic-checker-error-invalid-operand.\
						 negate-operator"
					)
				});

			if unary.operator_token.is_some()
			{
				labels.push((
					Location::Span(
						unary.operator_token.as_ref().unwrap().span,
					),
					operator_description.clone().to_string(),
				));
			}
			if let Some(right_span) = unary.right.get_span()
			{
				labels.push((
					Location::Span(right_span),
					rhs.inner.description(
						1,
						"lowercase",
						None,
						false,
					),
				));
			}

			return Err(SemanticCheckerError {
				code: SemanticCheckerErrorCode::InvalidOperand,
				message: t!(
					"semantic-checker-error-invalid-operand.message",
					operator = operator_description.clone()
				),
				hint: Some(t!(
					"semantic-checker-error-invalid-operand.\
					 negate-hint",
					data_type = rhs.inner.description(
						1,
						"uppercase",
						None,
						true
					),
					operator = operator_description
				)),
				labels,
				source_id: self.source_id.clone(),
			});
		}

		Ok(rhs.clone())
	}

	/// Visit and check the unary expression
	/// for semantic errors
	///
	/// # Arguments
	///
	/// * `unary` - The unary expression
	///
	/// # Returns
	///
	/// The data type of the unary expression
	fn check_unary_expr(
		&self,
		unary: &ast::UnaryExpr,
	) -> SemanticCheckerResult<DataTypeNode>
	{
		let right_data_type =
			self.check_expression(&unary.right)?;

		match unary.operator
		{
			ast::UnaryOperator::Negate => self
				.check_unary_negate_operands(
					unary,
					&right_data_type,
				),
		}
	}

	/// Check the binary operands for arithmetic operations
	///
	/// # Arguments
	///
	/// * `binary` - The binary expression
	/// * `lhs` - The left-hand side data type
	/// * `rhs` - The right-hand side data type
	///
	/// # Returns
	///
	/// The data type of the binary expression
	///
	/// # Errors
	///
	/// If the operands are invalid, it will return an error
	fn check_binary_arithmetic_operands(
		&self,
		binary: &ast::BinaryExpr,
		lhs: &DataTypeNode,
		rhs: &DataTypeNode,
	) -> SemanticCheckerResult<DataTypeNode>
	{
		let maybe_result_data_type =
			DataType::binary_expr_result_data_type(
				&lhs.inner, &rhs.inner,
			);

		// only numeric data types are allowed
		if !lhs.inner.is_numeric()
			|| !rhs.inner.is_numeric()
			|| maybe_result_data_type.is_none()
		{
			let mut labels: Vec<(Location, String)> = vec![];

			let operator_description = binary
				.operator_token
				.as_ref()
				.map(|token| {
					token.description(
						1,
						"lowercase",
						None,
						false,
						true,
					)
				})
				.unwrap_or_else(|| {
					t!(
						"semantic-checker-error-invalid-operand.\
						 binary-operator"
					)
				});

			if binary.operator_token.is_some()
			{
				labels.push((
					Location::Span(
						binary.operator_token.as_ref().unwrap().span,
					),
					operator_description.clone(),
				));
			}
			if let Some(left_span) = binary.left.get_span()
			{
				labels.push((
					Location::Span(left_span),
					lhs.inner.description(
						1,
						"lowercase",
						None,
						false,
					),
				));
			}
			if let Some(right_span) = binary.right.get_span()
			{
				labels.push((
					Location::Span(right_span),
					rhs.inner.description(
						1,
						"lowercase",
						None,
						false,
					),
				));
			}

			return Err(SemanticCheckerError {
				code: SemanticCheckerErrorCode::InvalidOperand,
				message: t!(
					"semantic-checker-error-invalid-operand.message",
					operator = operator_description.clone()
				),
				hint: Some(t!(
					"semantic-checker-error-invalid-operand.\
					 binary-hint",
					data_type1 = lhs.inner.description(
						1,
						"uppercase",
						None,
						true
					),
					data_type2 = rhs.inner.description(
						1,
						"lowercase",
						None,
						true
					),
					operator = operator_description
				)),
				labels,
				source_id: self.source_id.clone(),
			});
		}

		Ok(lhs.clone())
	}

	/// Visit and check the binary expression
	/// for semantic errors
	///
	/// # Arguments
	///
	/// * `binary` - The binary expression
	///
	/// # Returns
	///
	/// The data type of the binary expression
	fn check_binary_expr(
		&self,
		binary: &ast::BinaryExpr,
	) -> SemanticCheckerResult<DataTypeNode>
	{
		let left_data_type =
			self.check_expression(&binary.left)?;
		let right_data_type =
			self.check_expression(&binary.right)?;

		match binary.operator
		{
			ast::BinaryOperator::Add
			| ast::BinaryOperator::Subtract
			| ast::BinaryOperator::Multiply
			| ast::BinaryOperator::Divide
			| ast::BinaryOperator::Modulo
			| ast::BinaryOperator::Exponent => self
				.check_binary_arithmetic_operands(
					binary,
					&left_data_type,
					&right_data_type,
				),
		}
	}

	/// Check the literal expression
	/// for semantic errors
	fn check_literal_expr(
		&self,
		literal: &ast::LiteralExpr,
	) -> SemanticCheckerResult<DataTypeNode>
	{
		match literal.value
		{
			ast::Value::UInt8(_) =>
			{
				Ok(ast_known_data_type!(UInt8))
			}
			ast::Value::UInt16(_) =>
			{
				Ok(ast_known_data_type!(UInt16))
			}
			ast::Value::UInt32(_) =>
			{
				Ok(ast_known_data_type!(UInt32))
			}
			ast::Value::UInt64(_) =>
			{
				Ok(ast_known_data_type!(UInt64))
			}
			ast::Value::Int8(_) => Ok(ast_known_data_type!(Int8)),
			ast::Value::Int16(_) =>
			{
				Ok(ast_known_data_type!(Int16))
			}
			ast::Value::Int32(_) =>
			{
				Ok(ast_known_data_type!(Int32))
			}
			ast::Value::Int64(_) =>
			{
				Ok(ast_known_data_type!(Int64))
			}
			ast::Value::Int(_) => Ok(ast_known_data_type!(Int)),
			ast::Value::UInt(_) => Ok(ast_known_data_type!(UInt)),
			ast::Value::Float32(_) =>
			{
				Ok(ast_known_data_type!(Float32))
			}
			ast::Value::Double(_) =>
			{
				Ok(ast_known_data_type!(Double))
			}
		}
	}
}
