use std::ops::Range;

/// Position in the source code.
/// It can be used to point a character in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position
{
	/// Line number (1-based).
	pub line: usize,
	/// Column number (1-based).
	pub column: usize,
	/// Character index in the source code (0-based).
	pub char_index: usize,
	/// The byte offset in the source code (0-based).
	pub offset: usize,
}

/// Implementation of `Span`.
impl Position
{
	/// Creates a new `Span`.
	pub fn new(
		line: usize,
		column: usize,
		char_index: usize,
		offset: usize,
	) -> Position
	{
		Position {
			line,
			column,
			char_index,
			offset,
		}
	}
}

/// `Default` implementation for `Span`.
impl Default for Position
{
	/// Returns the default `Span`.
	fn default() -> Position
	{
		Position {
			line: 1,
			column: 1,
			char_index: 0,
			offset: 0,
		}
	}
}

/// Span in the source code.
/// It can be used to point a range of characters in the
/// source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span
{
	/// Start position.
	pub start: Position,
	/// End position.
	pub end: Position,
}

/// Implementation of `Span`.
impl Span
{
	/// Creates a new `Span`.
	pub fn new(start: Position, end: Position) -> Span
	{
		Span { start, end }
	}
}

/// `Default` implementation for `Span`.
/// It returns a `Span` with default `Position`s.
impl Default for Span
{
	/// Returns the default `Span`.
	fn default() -> Span
	{
		Span {
			start: Position::default(),
			end: Position::default(),
		}
	}
}

/// `Into<Range<usize>>` implementation for `Span`.
impl Into<Range<usize>> for Span
{
	/// Converts the `Span` into a `Range<usize>`.
	fn into(self) -> Range<usize>
	{
		self.start.offset .. self.end.offset
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn test_position_new()
	{
		let position = Position::new(1, 1, 0, 0);
		assert_eq!(position.line, 1);
		assert_eq!(position.column, 1);
		assert_eq!(position.char_index, 0);
		assert_eq!(position.offset, 0);
	}

	#[test]
	fn test_position_default()
	{
		let position = Position::default();
		assert_eq!(position.line, 1);
		assert_eq!(position.column, 1);
		assert_eq!(position.char_index, 0);
		assert_eq!(position.offset, 0);
	}

	#[test]
	fn test_span_new()
	{
		let start = Position::new(1, 1, 0, 0);
		let end = Position::new(1, 2, 1, 1);
		let span = Span::new(start, end);
		assert_eq!(span.start.line, 1);
		assert_eq!(span.start.column, 1);
		assert_eq!(span.start.char_index, 0);
		assert_eq!(span.start.offset, 0);
		assert_eq!(span.end.line, 1);
		assert_eq!(span.end.column, 2);
		assert_eq!(span.end.char_index, 1);
		assert_eq!(span.end.offset, 1);
	}

	#[test]
	fn test_span_default()
	{
		let span = Span::default();
		assert_eq!(span.start.line, 1);
		assert_eq!(span.start.column, 1);
		assert_eq!(span.start.char_index, 0);
		assert_eq!(span.end.line, 1);
		assert_eq!(span.end.column, 1);
		assert_eq!(span.end.char_index, 0);
	}

	#[test]
	fn test_span_into()
	{
		let start = Position::new(1, 1, 0, 0);
		let end = Position::new(1, 2, 1, 1);
		let span = Span::new(start, end);
		let range: Range<usize> = span.into();
		assert_eq!(range.start, 0);
		assert_eq!(range.end, 1);
	}
}
