//use std::io::prelude::*;

use crate::ast::{self, Item, Module, Procedure};

mod io;
mod value;
mod intrinsic;

pub use self::value::Value;

#[derive(Debug, Clone)]
pub struct StackFrame {
    args: Vec<Value>,
    locals: Vec<(String, Value)>,
}

pub type Scope = Vec<(String, Value)>;
pub type Stack = Vec<StackFrame>;
pub type EvalResult = Result<Value, &'static str>;

pub trait Eval {
	fn eval(&self, scope: &Scope) -> EvalResult;
}

pub enum EvalError {
	TypeMismatch,
	
}


pub fn interpret_file(items: Vec<ast::Item>) {
	// create root module
	let mut root = Module::new("root".into(), items);
		
	root.items.push(
		Item::InternProc("puts", intrinsic::puts, intrinsic::unputs)
	);
	
	let main = root.items.iter()
		.find(|item| matches!(item,
			Item::Proc(Procedure { name, .. }) if name == "main"
		));
	
	// run main procedure, if any
	if let Some(main) = main {
		if let Item::Proc(pr) = main {
			println!("running `proc main`...");
			pr.call(Vec::new(), &root);
		} else {
			eprintln!("found `main`, but it's not a procedure");
		}
	} else {
		eprintln!("No main procedure found.");
	}
}
