use serde_json::json;
use smol_str::SmolStr;

use crate::t;

/// Known data types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnownDataType
{
	/// Unsigned 8-bit integer
	UInt8,
	/// Unsigned 16-bit integer
	UInt16,
	/// Unsigned 32-bit integer
	UInt32,
	/// Unsigned 64-bit integer
	UInt64,
	/// Signed 8-bit integer
	Int8,
	/// Signed 16-bit integer
	Int16,
	/// Signed 32-bit integer
	Int32,
	/// Signed 64-bit integer
	Int64,
	/// Platform-dependent integer
	Int,
	/// Platform-dependent unsigned integer
	UInt,
	/// 32-bit floating point
	Float32,
	/// 64-bit floating point
	Double,
}

/// Data type inner
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType
{
	/// Known data type
	Known(KnownDataType),
	/// User-defined data type
	UserDefined(SmolStr),
}

/// `ToString` implementation for `DataType
impl ToString for DataType
{
	/// Convert the data type to string
	fn to_string(&self) -> String
	{
		match &self
		{
			DataType::Known(known) => match known
			{
				KnownDataType::UInt8 => "uint8".to_string(),
				KnownDataType::UInt16 => "uint16".to_string(),
				KnownDataType::UInt32 => "uint32".to_string(),
				KnownDataType::UInt64 => "uint64".to_string(),
				KnownDataType::Int8 => "uint8".to_string(),
				KnownDataType::Int16 => "int16".to_string(),
				KnownDataType::Int32 => "int32".to_string(),
				KnownDataType::Int64 => "int64".to_string(),
				KnownDataType::Int => "int".to_string(),
				KnownDataType::UInt => "uInt".to_string(),
				KnownDataType::Float32 => "float32".to_string(),
				KnownDataType::Double => "double".to_string(),
			},
			DataType::UserDefined(user_defined) =>
			{
				user_defined.to_string()
			}
		}
	}
}

/// Implementation for `DataType`
impl DataType
{
	/// Returns the human readable description of the data
	/// type
	pub fn description(
		&self,
		count: usize,
		capitalization: &str,
		value: Option<String>,
		show_count: bool,
	) -> String
	{
		let key = match self
		{
			DataType::Known(KnownDataType::UInt8) => "uint8",
			DataType::Known(KnownDataType::UInt16) => "uint16",
			DataType::Known(KnownDataType::UInt32) => "uint32",
			DataType::Known(KnownDataType::UInt64) => "uint64",
			DataType::Known(KnownDataType::Int8) => "int8",
			DataType::Known(KnownDataType::Int16) => "int16",
			DataType::Known(KnownDataType::Int32) => "int32",
			DataType::Known(KnownDataType::Int64) => "int64",
			DataType::Known(KnownDataType::Int) => "int",
			DataType::Known(KnownDataType::UInt) => "uint",
			DataType::Known(KnownDataType::Float32) => "float32",
			DataType::Known(KnownDataType::Double) => "double",
			DataType::UserDefined(t) => t.as_str(),
		};

		t!(
			format!("data-type-description-{key}"),
			value = value.clone().map(|v| json!(v).to_string()),
			capitalization = capitalization,
			count = count,
			show_count = show_count.to_string(),
			show_value = value.is_some().to_string();
			if let DataType::UserDefined(t) = self
			{
				t.to_string()
			} else {
				format!("data-type-description-{key}")
			}
		)
	}

	/// Check if two data types are the same
	///
	/// # Arguments
	///
	/// * `lhs` - The left data type
	/// * `rhs` - The right data type
	///
	/// # Returns
	///
	/// `true` if the data types are the same, otherwise
	/// `false`
	pub fn is_same(lhs: &DataType, rhs: &DataType) -> bool
	{
		if let DataType::Known(lhs_known) = lhs
			&& let DataType::Known(rhs_known) = rhs
		{
			return std::mem::discriminant(lhs_known)
				== std::mem::discriminant(rhs_known);
		}
		false
	}

	/// Get the bit size of the data type
	/// in bits
	///
	/// # Returns
	///
	/// The bit size of the data type
	pub fn get_bit_size(&self) -> u32
	{
		match self
		{
			DataType::Known(KnownDataType::UInt8) => 8,
			DataType::Known(KnownDataType::UInt16) => 16,
			DataType::Known(KnownDataType::UInt32) => 32,
			DataType::Known(KnownDataType::UInt64) => 64,
			DataType::Known(KnownDataType::Int8) => 8,
			DataType::Known(KnownDataType::Int16) => 16,
			DataType::Known(KnownDataType::Int32) => 32,
			DataType::Known(KnownDataType::Int64) => 64,
			DataType::Known(KnownDataType::Int) => 32, /* it can be 64 but we are using 32 to be safe */
			DataType::Known(KnownDataType::UInt) => 32, /* it can be 64 but we are using 32 to be safe */
			DataType::Known(KnownDataType::Float32) => 32,
			DataType::Known(KnownDataType::Double) => 64,
			DataType::UserDefined(_) => 0,
		}
	}

	/// Check if the source data type can be implictly
	/// converted to target data type
	///
	/// # Arguments
	///
	/// * `source` - The source data type
	/// * `target` - The target data type
	///
	/// # Returns
	///
	/// `true` if the data type can be implictly converted to
	/// target data type, otherwise `false`
	pub fn can_implictly_cast_to(
		source: &DataType,
		target: &DataType,
	) -> bool
	{
		let source_bit_size = source.get_bit_size();

		if source.is_signed_integer()
		{
			(target.is_signed_integer()
				&& target.get_bit_size() >= source_bit_size)
				|| target.is_floating_point()
		}
		else if source.is_unsigned_integer()
		{
			(target.is_unsigned_integer()
				&& target.get_bit_size() >= source_bit_size)
				|| target.is_floating_point()
		}
		else if source.is_floating_point()
		{
			target.is_floating_point()
				&& target.get_bit_size() >= source_bit_size
		}
		else
		{
			false
		}
	}

	/// Check if the data type is integer
	///
	/// # Returns
	///
	/// `true` if the data type is integer, otherwise
	/// `false`
	pub fn is_signed_integer(&self) -> bool
	{
		match self
		{
			DataType::Known(KnownDataType::Int8)
			| DataType::Known(KnownDataType::Int16)
			| DataType::Known(KnownDataType::Int32)
			| DataType::Known(KnownDataType::Int64)
			| DataType::Known(KnownDataType::Int) => true,
			_ => false,
		}
	}

	/// Check if the data type is unsigned integer
	///
	/// # Returns
	///
	/// `true` if the data type is integer, otherwise
	/// `false`
	pub fn is_unsigned_integer(&self) -> bool
	{
		match self
		{
			DataType::Known(KnownDataType::UInt8)
			| DataType::Known(KnownDataType::UInt16)
			| DataType::Known(KnownDataType::UInt32)
			| DataType::Known(KnownDataType::UInt64)
			| DataType::Known(KnownDataType::UInt) => true,
			_ => false,
		}
	}

	/// Check if the data type is integer
	///
	/// # Returns
	///
	/// `true` if the data type is integer, otherwise
	/// `false`
	pub fn is_generic_integer(&self) -> bool
	{
		return self.is_signed_integer()
			|| self.is_unsigned_integer();
	}

	/// Check if the data type is floating point
	///
	/// # Returns
	///
	/// `true` if the data type is floating point, otherwise
	/// `false`
	pub fn is_floating_point(&self) -> bool
	{
		match self
		{
			DataType::Known(KnownDataType::Float32)
			| DataType::Known(KnownDataType::Double) => true,
			_ => false,
		}
	}

	/// Check if the data type is number
	///
	/// # Returns
	///
	/// `true` if the data type is number, otherwise
	/// `false`
	pub fn is_numeric(&self) -> bool
	{
		return self.is_generic_integer()
			|| self.is_floating_point();
	}

	/// Infer the data type of the binary expression
	/// based on the left and right data types.
	///
	/// # Arguments
	///
	/// * `left` - The left data type
	/// * `right` - The right data type
	///
	/// # Returns
	///
	/// The inferred data type if the data types are
	/// compatible,
	pub fn binary_expr_result_data_type(
		left: &DataType,
		right: &DataType,
	) -> Option<DataType>
	{
		if DataType::is_same(left, right)
		{
			return Some(left.clone());
		}
		else if DataType::can_implictly_cast_to(right, left)
		{
			return Some(left.clone());
		}
		else if DataType::can_implictly_cast_to(left, right)
		{
			return Some(right.clone());
		}
		None
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn test_data_type_to_string()
	{
		assert_eq!(
			DataType::Known(KnownDataType::UInt8).to_string(),
			"uint8"
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt16).to_string(),
			"uint16"
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt32).to_string(),
			"uint32"
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt64).to_string(),
			"uint64"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int8).to_string(),
			"uint8"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int16).to_string(),
			"int16"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int32).to_string(),
			"int32"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int64).to_string(),
			"int64"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int).to_string(),
			"int"
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt).to_string(),
			"uInt"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Float32).to_string(),
			"float32"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Double).to_string(),
			"double"
		);
		assert_eq!(
			DataType::UserDefined("MyType".into()).to_string(),
			"MyType"
		);
	}

	#[test]
	fn test_data_type_description()
	{
		assert_eq!(
			DataType::Known(KnownDataType::UInt8).description(
				1,
				"lowercase",
				None,
				true
			),
			"an 8-bit unsigned integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt16).description(
				1,
				"lowercase",
				None,
				true
			),
			"a 16-bit unsigned integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt32).description(
				1,
				"lowercase",
				None,
				true
			),
			"a 32-bit unsigned integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt64).description(
				1,
				"lowercase",
				None,
				true
			),
			"a 64-bit unsigned integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int8).description(
				1,
				"lowercase",
				None,
				true
			),
			"an 8-bit integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int16).description(
				1,
				"lowercase",
				None,
				true
			),
			"a 16-bit integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int32).description(
				1,
				"lowercase",
				None,
				true
			),
			"a 32-bit integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int64).description(
				1,
				"lowercase",
				None,
				true
			),
			"a 64-bit integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int).description(
				1,
				"lowercase",
				None,
				true
			),
			"an integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt).description(
				1,
				"lowercase",
				None,
				true
			),
			"an unsigned integer"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Float32).description(
				1,
				"lowercase",
				None,
				true
			),
			"a 32-bit floating point number"
		);
		assert_eq!(
			DataType::Known(KnownDataType::Double).description(
				1,
				"lowercase",
				None,
				true
			),
			"a double precision floating point number"
		);
		assert_eq!(
			DataType::UserDefined("MyType".into()).description(
				1,
				"lowercase",
				None,
				true
			),
			"MyType"
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt8).description(
				1,
				"uppercase",
				Some("123".to_string()),
				true
			),
			"An 8-bit unsigned integer \u{2068}\"123\"\u{2069}"
		);
	}

	#[test]
	fn test_data_type_is_same()
	{
		assert_eq!(
			DataType::is_same(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::UInt8)
			),
			true
		);
		assert_eq!(
			DataType::is_same(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::UInt16)
			),
			false
		);
		assert_eq!(
			DataType::is_same(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::UserDefined("MyType".into())
			),
			false
		);
	}

	#[test]
	fn test_data_type_get_bit_size()
	{
		assert_eq!(
			DataType::Known(KnownDataType::UInt8).get_bit_size(),
			8
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt16).get_bit_size(),
			16
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt32).get_bit_size(),
			32
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt64).get_bit_size(),
			64
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int8).get_bit_size(),
			8
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int16).get_bit_size(),
			16
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int32).get_bit_size(),
			32
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int64).get_bit_size(),
			64
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int).get_bit_size(),
			32
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt).get_bit_size(),
			32
		);
		assert_eq!(
			DataType::Known(KnownDataType::Float32)
				.get_bit_size(),
			32
		);
		assert_eq!(
			DataType::Known(KnownDataType::Double).get_bit_size(),
			64
		);
		assert_eq!(
			DataType::UserDefined("MyType".into()).get_bit_size(),
			0
		);
	}

	#[test]
	fn test_data_type_can_implictly_cast_to()
	{
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::UInt8)
			),
			true
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::UInt32)
			),
			true
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Int8)
			),
			false
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Int16)
			),
			false
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Int64)
			),
			false
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Float32)
			),
			true
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Double)
			),
			true
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::Int),
				&DataType::Known(KnownDataType::Int64)
			),
			true
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::Int),
				&DataType::Known(KnownDataType::Double)
			),
			true
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::Float32),
				&DataType::Known(KnownDataType::Double)
			),
			true
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::Known(KnownDataType::Double),
				&DataType::Known(KnownDataType::Float32)
			),
			false
		);
		assert_eq!(
			DataType::can_implictly_cast_to(
				&DataType::UserDefined("MyType".into()),
				&DataType::Known(KnownDataType::Float32)
			),
			false
		);
	}

	#[test]
	fn test_data_type_is_numeric()
	{
		assert_eq!(
			DataType::Known(KnownDataType::UInt8).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt16).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt32).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt64).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int8).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int16).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int32).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int64).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::Int).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::UInt).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::Float32).is_numeric(),
			true
		);
		assert_eq!(
			DataType::Known(KnownDataType::Double).is_numeric(),
			true
		);
		assert_eq!(
			DataType::UserDefined("MyType".into()).is_numeric(),
			false
		);
	}

	#[test]
	fn test_data_type_binary_expr_result_data_type()
	{
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::UInt8)
			),
			Some(DataType::Known(KnownDataType::UInt8))
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::UInt32)
			),
			Some(DataType::Known(KnownDataType::UInt32))
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Int8)
			),
			None
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Int16)
			),
			None
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Int64)
			),
			None
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Float32)
			),
			Some(DataType::Known(KnownDataType::Float32))
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::UInt8),
				&DataType::Known(KnownDataType::Double)
			),
			Some(DataType::Known(KnownDataType::Double))
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::Int),
				&DataType::Known(KnownDataType::Int64)
			),
			Some(DataType::Known(KnownDataType::Int64))
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::Int),
				&DataType::Known(KnownDataType::Double)
			),
			Some(DataType::Known(KnownDataType::Double))
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::Float32),
				&DataType::Known(KnownDataType::Double)
			),
			Some(DataType::Known(KnownDataType::Double))
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::Known(KnownDataType::Double),
				&DataType::Known(KnownDataType::Float32)
			),
			Some(DataType::Known(KnownDataType::Double))
		);
		assert_eq!(
			DataType::binary_expr_result_data_type(
				&DataType::UserDefined("MyType".into()),
				&DataType::Known(KnownDataType::Float32)
			),
			None
		);
	}
}
