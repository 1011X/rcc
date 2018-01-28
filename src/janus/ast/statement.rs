//use std::collections::HashMap;
//use super::interpret::{SymTab, Value};
use super::*;

type Block = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
	Skip,
	Local(Decl, Expr),
	Delocal(Decl, Expr),
	Add(LValue, Expr),
	Sub(LValue, Expr),
	Xor(LValue, Expr),
	Swap(LValue, LValue),
	If(Pred, Block, Option<Block>, Pred),
	From(Pred, Option<Block>, Option<Block>, Pred),
	Call(String, Vec<Factor>),
	Uncall(String, Vec<Factor>),
	
	// built-ins
	Print(String),
	Printf(String, Vec<Factor>),
	Error(String),
	Show(LValue),
	Pop(Factor, LValue),
	Push(Factor, LValue),
}

impl Statement {
	named!(pub parse<Self>, sp!(alt_complete!(
		value!(Statement::Skip, tag!("skip"))
		| do_parse!(
			tag!("local") >>
			decl: call!(Decl::parse) >>
			tag!("=") >>
			val: call!(Expr::parse)
			>> (Statement::Local(decl, val))
		)
		| do_parse!(
			tag!("delocal") >>
			decl: call!(Decl::parse) >>
			tag!("=") >>
			val: call!(Expr::parse)
			>> (Statement::Delocal(decl, val))
		)
		| do_parse!(
			left: call!(LValue::parse) >>
			tag!("<=>") >>
			right: call!(LValue::parse)
			>> (Statement::Swap(left, right))
		)
		| do_parse!(
			left: call!(LValue::parse) >>
			tag!("+=") >>
			expr: call!(Expr::parse)
			>> (Statement::Add(left, expr))
		)
		| do_parse!(
			left: call!(LValue::parse) >>
			tag!("-=") >>
			expr: call!(Expr::parse)
			>> (Statement::Sub(left, expr))
		)
		| do_parse!(
			left: call!(LValue::parse) >>
			tag!("^=") >>
			expr: call!(Expr::parse)
			>> (Statement::Xor(left, expr))
		)
		| do_parse!(
			tag!("from") >>
			assert: call!(Pred::parse) >>
			forward: opt!(preceded!(
				tag!("do"),
				many1!(Statement::parse)
			)) >>
			backward: opt!(preceded!(
				tag!("loop"),
				many1!(Statement::parse)
			)) >>
			tag!("until") >>
			pred: call!(Pred::parse)
			
			>> (Statement::From(assert, forward, backward, pred))
		)
		| do_parse!(
			tag!("if") >>
			pred: call!(Pred::parse) >>
			pass: preceded!(
				tag!("then"),
				many1!(Statement::parse)
			) >>
			fail: opt!(preceded!(
				tag!("else"),
				many1!(Statement::parse)
			)) >>
			tag!("fi") >>
			assert: call!(Pred::parse)
			
			>> (Statement::If(pred, pass, fail, assert))
		)
		| do_parse!(
			tag!("call") >>
			func: ident >>
			args: delimited!(
				tag!("("),
				separated_list!(tag!(","), Factor::parse),
				tag!(")")
			)
			>> (Statement::Call(func, args))
		)
		| do_parse!(
			tag!("uncall") >>
			func: ident >>
			tag!("(") >>
			args: separated_list!(tag!(","), Factor::parse) >>
			tag!(")")
			>> (Statement::Uncall(func, args))
		)
		// built-ins
		| do_parse!(
			tag!("print") >>
			tag!("(") >>
			string: st >>
			tag!(")")
			>> (Statement::Print(string))
		)
		| do_parse!(
			tag!("printf") >>
			tag!("(") >>
			string: st >>
			vargs: many0!(preceded!(
				tag!(","),
				Factor::parse
			)) >>
			tag!(")")
			>> (Statement::Printf(string, vargs))
		)
		| do_parse!(
			tag!("error") >>
			tag!("(") >>
			string: st >>
			tag!(")")
			>> (Statement::Error(string))
		)
		| do_parse!(
			tag!("show") >>
			tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Statement::Show(lval))
		)
		| do_parse!(
			tag!("pop") >>
			tag!("(") >>
			into: call!(Factor::parse) >>
			tag!(",") >>
			from: call!(LValue::parse) >>
			tag!(")")
			>> (Statement::Pop(into, from))
		)
		| do_parse!(
			tag!("push") >>
			tag!("(") >>
			from: call!(Factor::parse) >>
			tag!(",") >>
			into: call!(LValue::parse) >>
			tag!(")")
			>> (Statement::Push(from, into))
		)
	)));
	/*
	pub fn eval(&self, globs: &mut SymTab) {
		use self::Statement::*;
		match *self {
			Skip => (),
			Local(ref decl, ref expr) => {
				globs.insert(decl.name.clone(), expr.eval(globs).unwrap());
			}
			Delocal(ref decl, ref expr) => {
				let value = globs.remove(&decl.name).unwrap();
				assert_eq!(value, expr.eval(globs).unwrap());
			}
			Add(ref lval, ref expr) => {
				//let var = lval.eval();
				let entry = globs.get_mut(&lval.name).unwrap();
				match *entry {
					Value::Int(ref mut num) => {
						if let Value::Int(n) = expr.eval(globs).unwrap() {
							*num += n;
						}
					}
					_ => unimplemented!()
					//Value::IntArray(ref vec) =>
				}
			}
			_ => unimplemented!()
		}
	}
	*/
}