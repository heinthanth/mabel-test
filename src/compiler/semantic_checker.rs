use std::io::BufRead;
use std::ops::Range;
use std::vec;

use codespan_reporting::diagnostic::{
	Diagnostic,
	Label,
	LabelStyle,
};
use codespan_reporting::files::SimpleFiles;
use smol_str::SmolStr;
use termcolor::WriteColor;

use super::annotated_ast::{
	AnnotatedBinaryExpr,
	AnnotatedEchoStmt,
	AnnotatedExpression,
	AnnotatedExpressionStmt,
	AnnotatedFunctionDeclStmt,
	AnnotatedLiteralExpr,
	AnnotatedModule,
	AnnotatedStatement,
	AnnotatedUnaryExpr,
};
use super::data_type::{DataType, KnownDataType};
use super::session_globals::SessionGlobals;
use crate::parser::ast::{
	AstVisitor,
	BinaryExpr,
	EchoStmt,
	ExpressionStmt,
	GetSpan,
	UnaryExpr,
	{self},
};
use crate::parser::span::Location;
use crate::t;

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
	pub labels: Vec<(LabelStyle, Location, String)>,
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
			.map(|(style, location, message)| {
				Label::new(
					style.clone(),
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
	/// The module to check
	module: ast::Module<ast::Expression>,
}

impl
	ast::AstVisitor<
		ast::Expression,
		ast::Statement<ast::Expression>,
		SemanticCheckerResult<
			AnnotatedModule<AnnotatedExpression>,
		>,
		SemanticCheckerResult<AnnotatedExpression>,
		SemanticCheckerResult<
			AnnotatedStatement<AnnotatedExpression>,
		>,
	> for SemanticChecker
{
	/// Visit and check the module node
	/// for semantic errors
	fn visit_module(
		&mut self,
		module: &ast::Module<ast::Expression>,
	) -> SemanticCheckerResult<
		AnnotatedModule<AnnotatedExpression>,
	>
	{
		let mut statements = vec![];

		for statement in &module.statements
		{
			statements.push(self.visit_statement(statement)?);
		}

		Ok(AnnotatedModule {
			id: self.source_id.clone(),
			statements,
		})
	}

	/// Visit and check the statement node
	/// for semantic errors
	fn visit_statement(
		&mut self,
		statement: &ast::Statement<ast::Expression>,
	) -> SemanticCheckerResult<
		AnnotatedStatement<AnnotatedExpression>,
	>
	{
		match statement
		{
			ast::Statement::Expression(stmt) =>
			{
				self.visit_expression_stmt(stmt)
			}
			ast::Statement::Echo(stmt) =>
			{
				self.visit_echo_stmt(stmt)
			}
			ast::Statement::FunctionDeclaration(func) =>
			{
				self.visit_function_decl_stmt(func)
			}
		}
	}

	/// Visit and check the function declaration node
	/// for semantic errors
	fn visit_function_decl_stmt(
		&mut self,
		function: &ast::FunctionDeclStmt<
			ast::Statement<ast::Expression>,
		>,
	) -> SemanticCheckerResult<
		AnnotatedStatement<AnnotatedExpression>,
	>
	{
		let mut body: Vec<
			Box<AnnotatedStatement<AnnotatedExpression>>,
		> = vec![];
		for statement in &function.body
		{
			let stmt = self.visit_statement(&statement)?;
			body.push(Box::new(stmt));
		}

		Ok(AnnotatedStatement::FunctionDecl(
			AnnotatedFunctionDeclStmt {
				inner: ast::FunctionDeclStmt {
					function_token: function.function_token.clone(),
					name: function.name.clone(),
					left_paren_token: function
						.left_paren_token
						.clone(),
					parameters: function.parameters.clone(),
					right_paren_token: function
						.right_paren_token
						.clone(),
					body,
				},
				data_type: None,
			},
		))
	}

	/// Visit and check the expression statement node
	/// for semantic errors
	fn visit_expression_stmt(
		&mut self,
		expression_stmt: &ast::ExpressionStmt<ast::Expression>,
	) -> SemanticCheckerResult<
		AnnotatedStatement<AnnotatedExpression>,
	>
	{
		let annotated_expression =
			self.visit_expression(&expression_stmt.expression)?;
		Ok(AnnotatedStatement::Expression(
			AnnotatedExpressionStmt {
				inner: ExpressionStmt {
					expression: annotated_expression,
				},
				data_type: None,
			},
		))
	}

	/// Visit and check the echo statement node
	/// for semantic errors
	fn visit_echo_stmt(
		&mut self,
		echo_stmt: &ast::EchoStmt<ast::Expression>,
	) -> SemanticCheckerResult<
		AnnotatedStatement<AnnotatedExpression>,
	>
	{
		let annotated_expression =
			self.visit_expression(&echo_stmt.expression)?;
		Ok(AnnotatedStatement::Echo(AnnotatedEchoStmt {
			inner: EchoStmt {
				echo_token: echo_stmt.echo_token.clone(),
				expression: annotated_expression,
			},
			data_type: None,
		}))
	}

	/// Visit and check the expression node
	/// for semantic errors
	fn visit_expression(
		&mut self,
		expression: &ast::Expression,
	) -> SemanticCheckerResult<AnnotatedExpression>
	{
		match expression
		{
			ast::Expression::Literal(literal) =>
			{
				self.visit_literal_expr(literal)
			}
			ast::Expression::Grouping(grouping) =>
			{
				self.visit_grouping_expr(grouping)
			}
			ast::Expression::Unary(unary) =>
			{
				self.visit_unary_expr(&unary)
			}
			ast::Expression::Binary(binary) =>
			{
				self.visit_binary_expr(&binary)
			}
		}
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
	fn visit_unary_expr(
		&mut self,
		unary_expr: &ast::UnaryExpr<ast::Expression>,
	) -> SemanticCheckerResult<AnnotatedExpression>
	{
		let right_data_type =
			self.visit_expression(&unary_expr.right)?;

		match unary_expr.operator
		{
			ast::UnaryOperator::Negate =>
			{
				let data_type = self.check_unary_negate_operands(
					unary_expr,
					&right_data_type.get_data_type(),
				)?;

				Ok(AnnotatedExpression::Unary(AnnotatedUnaryExpr {
					inner: UnaryExpr {
						operator: unary_expr.operator.clone(),
						operator_token: unary_expr
							.operator_token
							.clone(),
						right: Box::new(right_data_type),
					},
					data_type,
				}))
			}
		}
	}

	/// Check the grouping expression
	/// for semantic errors
	fn visit_grouping_expr(
		&mut self,
		grouping: &ast::GroupingExpr<ast::Expression>,
	) -> SemanticCheckerResult<AnnotatedExpression>
	{
		self.visit_expression(&grouping.expression)
	}

	/// Check the literal expression
	/// for semantic errors
	fn visit_literal_expr(
		&mut self,
		literal: &ast::LiteralExpr,
	) -> SemanticCheckerResult<AnnotatedExpression>
	{
		match literal.value
		{
			ast::Value::UInt8(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(
							KnownDataType::UInt8,
						),
					},
				))
			}
			ast::Value::UInt16(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(
							KnownDataType::UInt16,
						),
					},
				))
			}
			ast::Value::UInt32(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(
							KnownDataType::UInt32,
						),
					},
				))
			}
			ast::Value::UInt64(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(
							KnownDataType::UInt64,
						),
					},
				))
			}
			ast::Value::Int8(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(KnownDataType::Int8),
					},
				))
			}
			ast::Value::Int16(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(
							KnownDataType::Int16,
						),
					},
				))
			}
			ast::Value::Int32(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(
							KnownDataType::Int32,
						),
					},
				))
			}
			ast::Value::Int64(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(
							KnownDataType::Int64,
						),
					},
				))
			}
			ast::Value::Int(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(KnownDataType::Int),
					},
				))
			}
			ast::Value::UInt(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(KnownDataType::UInt),
					},
				))
			}
			ast::Value::Float32(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(
							KnownDataType::Float32,
						),
					},
				))
			}
			ast::Value::Double(_) =>
			{
				Ok(AnnotatedExpression::Literal(
					AnnotatedLiteralExpr {
						inner: literal.clone(),
						data_type: DataType::Known(
							KnownDataType::Double,
						),
					},
				))
			}
		}
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
	fn visit_binary_expr(
		&mut self,
		binary: &ast::BinaryExpr<ast::Expression>,
	) -> SemanticCheckerResult<AnnotatedExpression>
	{
		let left_data_type =
			self.visit_expression(&binary.left)?;
		let right_data_type =
			self.visit_expression(&binary.right)?;

		match binary.operator
		{
			ast::BinaryOperator::Add
			| ast::BinaryOperator::Subtract
			| ast::BinaryOperator::Multiply
			| ast::BinaryOperator::Divide
			| ast::BinaryOperator::Modulo
			| ast::BinaryOperator::Exponent =>
			{
				let data_type = self
					.check_binary_arithmetic_operands(
						binary,
						&left_data_type.get_data_type(),
						&right_data_type.get_data_type(),
					)?;

				Ok(AnnotatedExpression::Binary(
					AnnotatedBinaryExpr {
						inner: BinaryExpr {
							left: Box::new(left_data_type),
							operator: binary.operator.clone(),
							operator_token: binary.operator_token.clone(),
							right: Box::new(right_data_type),
						},
						data_type,
					},
				))
			}
		}
	}
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
	fn new(
		source_id: SmolStr,
		module: ast::Module<ast::Expression>,
	) -> Self
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
		module: ast::Module<ast::Expression>,
	) -> SemanticCheckerResult<
		AnnotatedModule<AnnotatedExpression>,
	>
	{
		let mut checker = Self::new(source_id, module);
		checker.visit_module(&checker.module.clone())
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
		unary: &ast::UnaryExpr<ast::Expression>,
		rhs: &DataType,
	) -> SemanticCheckerResult<DataType>
	{
		// only signed integer and floating point are allowed
		if !rhs.is_floating_point() || !rhs.is_signed_integer()
		{
			let mut labels: Vec<(LabelStyle, Location, String)> =
				vec![];

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
					LabelStyle::Secondary,
					Location::Span(
						unary.operator_token.as_ref().unwrap().span,
					),
					operator_description.clone().to_string(),
				));
			}
			if let Some(right_span) = unary.right.get_span()
			{
				labels.push((
					LabelStyle::Primary,
					Location::Span(right_span),
					rhs.description(1, "lowercase", None, false),
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
					data_type =
						rhs.description(1, "uppercase", None, true),
					operator = operator_description
				)),
				labels,
				source_id: self.source_id.clone(),
			});
		}

		Ok(rhs.clone())
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
		binary: &ast::BinaryExpr<ast::Expression>,
		lhs: &DataType,
		rhs: &DataType,
	) -> SemanticCheckerResult<DataType>
	{
		let maybe_result_data_type =
			DataType::binary_expr_result_data_type(&lhs, &rhs);

		if maybe_result_data_type.is_none()
		{
			let mut labels: Vec<(LabelStyle, Location, String)> = vec![];

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
					LabelStyle::Secondary,
					Location::Span(
						binary.operator_token.as_ref().unwrap().span,
					),
					operator_description.clone(),
				));
			}
			if let Some(left_span) = binary.left.get_span()
			{
				labels.push((
					LabelStyle::Secondary,
					Location::Span(left_span),
					lhs.description(1, "lowercase", None, false),
				));
			}
			if let Some(right_span) = binary.right.get_span()
			{
				labels.push((
					LabelStyle::Primary,
					Location::Span(right_span),
					rhs.description(1, "lowercase", None, false),
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
					data_type1 =
						lhs.description(1, "uppercase", None, true),
					data_type2 =
						rhs.description(1, "lowercase", None, true),
					operator = operator_description
				)),
				labels,
				source_id: self.source_id.clone(),
			});
		}

		Ok(maybe_result_data_type.unwrap())
	}
}
