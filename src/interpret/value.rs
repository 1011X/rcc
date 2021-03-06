use std::fmt;

use crate::interpret::{EvalError, EvalResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
	Nil,
	Bool(bool),
	//Byte(u8),
	Int(i64),
	Uint(u64),
	Char(char),
	String(String),
	Array(Box<[Value]>),
	//Proc(Path),
}

use crate::ast::Type;
impl Value {
	pub fn get_type(&self) -> Type {
		match self {
			Value::Nil       => Type::Unit,
			Value::Bool(_)   => Type::Bool,
			Value::Int(_)    => Type::Int,
			Value::Uint(_)   => Type::UInt,
			Value::Char(_)   => Type::Char,
			Value::String(_) => Type::String,
			
			Value::Array(_)  => todo!()
		}
	}
	
	pub fn swap(&mut self, val: &mut Value) -> EvalResult<()> {
		// check that types are the same.
		if self.get_type() != val.get_type() {
			return Err(EvalError::TypeMismatch {
				expected: self.get_type(),
				got: val.get_type(),
			});
		}
		
		std::mem::swap(self, val);
		Ok(())
	}
	
	pub fn xor(&mut self, val: &Value) -> EvalResult<()> {
		match (self, val) {
			(Value::Nil, Value::Nil) => {}
			
			(Value::Bool(a), Value::Bool(b)) => *a ^= b,
			
			(Value::Int(a), Value::Int(b)) => *a ^= b,
			
			_ => todo!()
		}
		Ok(())
	}
}

impl fmt::Display for Value {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Value::Nil => fmt.write_str("nil"),
			
			Value::Bool(b) => b.fmt(fmt),
			Value::Int(i)  => i.fmt(fmt),
			Value::Uint(u) => u.fmt(fmt),
			
			Value::Char(c)   => write!(fmt, "{:?}", c),
			Value::String(s) => write!(fmt, "{:?}", s),
			
			Value::Array(array) => {
				fmt.write_str("[")?;
				for value in array.iter() {
					write!(fmt, "{}, ", value)?;
				}
				fmt.write_str("]")
			}
		}
	}
}

impl From<()> for Value {
	#[inline]
	fn from(_: ()) -> Self { Value::Nil }
}

impl From<bool> for Value {
	#[inline]
	fn from(b: bool) -> Self { Value::Bool(b) }
}

impl From<char> for Value {
	#[inline]
	fn from(c: char) -> Self { Value::Char(c) }
}

impl From<u64> for Value {
	#[inline]
	fn from(n: u64) -> Self { Value::Uint(n) }
}

impl From<i64> for Value {
	#[inline]
	fn from(n: i64) -> Self { Value::Int(n) }
}

impl From<String> for Value {
	#[inline]
	fn from(s: String) -> Self { Value::String(s) }
}
