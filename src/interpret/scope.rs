use std::collections::HashMap;

use super::*;

pub struct StackFrame {
	args:  HashMap<String, Value>,
	vars:  Vec<(String, Value)>,
	//items:  HashMap<String, Value>,
}

impl StackFrame {
	pub fn new(args: HashMap<String, Value>) -> Self {
		Self {
			args,
			vars: Vec::new(),
			//items: module,
		}
	}
	
	pub fn push(&mut self, name: String, val: Value) {
		self.vars.push((name, val));
	}
	
	pub fn pop(&mut self) -> Option<(String, Value)> {
		self.vars.pop()
	}
	
	pub fn remove(&mut self, given_name: &str) -> EvalResult<Value> {
		let idx = self.vars.iter()
			.enumerate()
			.rfind(|(_, (var_name, _))| *var_name == given_name)
			.map(|(i,_)| i)
			.ok_or(EvalError::UnknownIdent(given_name.to_string()))?;
		
		self.vars.remove(idx)
	}
	
	pub fn get(&self, given_name: &str) -> EvalResult<&Value> {
		self.vars.iter()
			.rfind(|(var_name, _)| var_name == given_name)
			.map(|(_, value)| value)
			.or_else(|| self.args.get(given_name))
			.ok_or(EvalError::UnknownIdent(given_name.to_string()))
	}
	
	pub fn get_mut(&mut self, given_name: &str) -> EvalResult<&mut Value> {
		self.vars.iter_mut()
			.rfind(|(var_name, _)| var_name == given_name)
			.map(|(_, value)| value)
			// not .or_else() bcuz the closure would have to borrow more than
			// one mutable reference.
			.or(self.args.get_mut(given_name))
			.ok_or(EvalError::UnknownIdent(given_name.to_string()))
	}
}
