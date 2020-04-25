/*!
Expressions in Rever have 5 levels of precendence. From strongest to weakest:
1. Parentheses
2. Function calls
3. Unary operators: not - (maybe: ! ~)
4. Exponential operators: ^ << >> shl shr rol ror
5. Multiplicative operators: * / mod as and
6. Additive operators: + - or xor
7. Relational operators: = != ≠ < > <= ≤ >= ≥ in

Ideas:
+ Chained relations, a la Python?
+ In `if` statements, conjunctions is `,` and disjunction is `;` (from Prolog).
  + No short-circuiting; like Pascal.
  + Short-circuiting can be achieved using `and` and `or`.

TODO:
+ Add precedences 2, 
*/

use super::*;

#[derive(Debug, Clone)]
pub enum BinOp {
	// precedence 4
	Exp,
	// precedence 5
	Mul, Div, Mod, And,
	// precedence 6
	Add, Sub, Or, Xor,
	// precedence 7
	Eq, Ne, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone)]
pub enum Expr {
	// precedence 1
	Term(Term),
	Group(Box<Expr>),
	Cast(Box<Expr>, Type),
	
	// precedence 3
	Not(Box<Expr>),
	
	// binary op, precendeces 4-7
	BinOp(BinOp, Box<Expr>, Box<Expr>),
	
	// secret precendece 8
	If(Box<Expr>, Box<Expr>, Box<Expr>),
	
	// secret precedence 9
	Let(String, Option<Type>, Box<Expr>, Box<Expr>),
}

impl Parse for Expr {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		if tokens.peek() == Some(&Token::If) {
			tokens.next();
			
			let test = Box::new(Expr::parse_rel(tokens)?);
			
			// ensure there's a newline afterwards
			if tokens.next() != Some(Token::Then) {
				return Err("`then` after `if` predicate");
			}
			
			// parse the main block
			let main_expr = Box::new(Expr::parse(tokens)?);
			
			// check for `else`
			if tokens.next() != Some(Token::Else) {
				return Err("`else` in `if` expression");
			}
			
			// parse else section
			let else_block = Box::new(Expr::parse(tokens)?);
			
			Ok(Expr::If(test, main_expr, else_block))
		} else if tokens.peek() == Some(&Token::Let) {
			tokens.next();
			
			let name = match tokens.next() {
				Some(Token::Ident(name)) => name,
				_ => return Err("name for let-binding")
			};
			
			// get optional type as `: type`
			let typ = if tokens.peek() == Some(&Token::Colon) {
				tokens.next();
				Some(Type::parse(tokens)?)
			} else {
				None
			};
			
			// check for '='
			if tokens.next() != Some(Token::Eq) {
				return Err("`=` after let-binding name");
			};
			
			// val is artificially limited here on purpose. it doesn't make much
			// sence to allow `let` inside a `let` binding value, for example.
			let val = Expr::parse_rel(tokens)?;
			
			// check for newline
			if tokens.next() != Some(Token::Newline) {
				return Err("`in` after `let` binding");
			}
			
			let scope = Expr::parse(tokens)?;
			
			Ok(Expr::Let(name, typ, Box::new(val), Box::new(scope)))
		} else {
			Expr::parse_rel(tokens)
		}
	}
}

// rel  -> expr {(=|≠|<|>|≤|≥|in) expr}
// expr -> term {(+|-|or) term}
// term -> exp {(*|/|mod|and) exp}
// exp  -> atom {^ atom}
// atom -> ( expr )
//      -> expr 'as' type
//      -> factor
impl Expr {
	fn parse_rel(tokens: &mut Tokens) -> ParseResult<Self> {
		// <term>
		let first = Expr::parse_expr(tokens)?;
		let mut exprs: Vec<(BinOp, Expr)> = Vec::new();
		
		// { ('=' | '!=' | '<' | '>' | '<=' | '>=') <expr> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Eq)  => BinOp::Eq,
				Some(Token::Neq) => BinOp::Ne,
				Some(Token::Lt)  => BinOp::Lt,
				Some(Token::Gt)  => BinOp::Gt,
				Some(Token::Lte) => BinOp::Le,
				Some(Token::Gte) => BinOp::Ge,
				//Some(Token::In) => BinOp::In,
			    _ => break
			};
			tokens.next();
		    
		    let expr = Expr::parse_expr(tokens)?;
		    exprs.push((op, expr));
		}
		
		if exprs.is_empty() {
		    return Ok(first);
		}
		
		Ok(exprs.into_iter().fold(first, |acc, (op, base)|
			Expr::BinOp(op, Box::new(acc), Box::new(base))
		))
	}
	
	pub fn parse_expr(tokens: &mut Tokens) -> ParseResult<Self> {
		// <term>
		let first = Expr::parse_term(tokens)?;
		let mut terms: Vec<(BinOp, Expr)> = Vec::new();
		
		// { ('+' | '-' | 'or' | ':') <term> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Plus)  => BinOp::Add,
				Some(Token::Minus) => BinOp::Sub,
				Some(Token::Or)    => BinOp::Or,
				Some(Token::Colon) => BinOp::Xor,
			    _ => break
			};
			tokens.next();
		    
		    let term = Expr::parse_term(tokens)?;
		    terms.push((op, term));
		}
		
		if terms.is_empty() {
		    return Ok(first);
		}
		
		Ok(terms.into_iter().fold(first, |acc, (op, base)|
			Expr::BinOp(op, Box::new(acc), Box::new(base))
		))
	}
	
	fn parse_term(tokens: &mut Tokens) -> ParseResult<Self> {
		// <fact>
		let first = Expr::parse_exp(tokens)?;
		let mut facts: Vec<(BinOp, Expr)> = Vec::new();
		
		// { ('*' | '/' | 'mod' | 'and') <fact> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Star)   => BinOp::Mul,
				Some(Token::FSlash) => BinOp::Div,
				Some(Token::Mod)    => BinOp::Mod,
				Some(Token::And)    => BinOp::And,
			    _ => break
			};
			tokens.next();
		    
		    let fact = Expr::parse_exp(tokens)?;
		    facts.push((op, fact));
		}
		
		if facts.is_empty() {
		    return Ok(first);
		}
		
		Ok(facts.into_iter().fold(first, |acc, (op, base)|
			Expr::BinOp(op, Box::new(acc), Box::new(base))
		))
	}
	
	fn parse_exp(tokens: &mut Tokens) -> ParseResult<Self> {
		// <exp>
		let first = Expr::parse_atom(tokens)?;
		let mut exps = Vec::new();
		
		// { ('^') <exp> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Caret) => {}
			    _ => break
			};
			tokens.next();
		    
		    let exp = Expr::parse_atom(tokens)?;
		    exps.push(exp);
		}
		
		if exps.is_empty() {
		    return Ok(first);
		}
		
		let last = exps.pop().unwrap();
		let res = exps.into_iter().rfold(last, |acc, base|
			Expr::BinOp(BinOp::Exp, Box::new(base), Box::new(acc))
		);
		
		Ok(Expr::BinOp(BinOp::Exp, Box::new(first), Box::new(res)))
	}
	
	fn parse_atom(tokens: &mut Tokens) -> ParseResult<Self> {
		// check if there's an open parenthesis
		if tokens.peek() == Some(&Token::LParen) {
			tokens.next();
			
			let expr = Expr::parse(tokens)?;
			
			// make sure there's a closing parenthesis
			if tokens.next() != Some(Token::RParen) {
				return Err("`)` after subexpression");
			}
			
			Ok(Expr::Group(Box::new(expr)))
		} else {
			// otherwise, treat it as a Term.
			Ok(Expr::Term(Term::parse(tokens)?))
		}
	}
	
	pub fn eval(&self, t: &Scope) -> EvalResult {
		match self {
			// 1
			Expr::Term(term) => Ok(term.eval(t)),
			Expr::Group(e) => Ok(e.eval(t)?),
			Expr::Cast(e, typ) => match (typ, e.eval(t)?) {
				(Type::Unit, _) => Ok(Value::Nil),
				(Type::Int, Value::Uint(v)) => Ok(Value::Int(v as i64)),
				(Type::UInt, Value::Int(v)) => Ok(Value::Uint(v as u64)),
				_ => unimplemented!()
			}
			
			// 3
			Expr::Not(e) => match e.eval(t)? {
				Value::Bool(true) => Ok(Value::Bool(false)),
				Value::Bool(false) => Ok(Value::Bool(true)),
				_ => Err("tried NOTting non-boolean value")
			}
			
			// 4 - 7
			Expr::BinOp(op, left, right) => {
				let left = left.eval(t)?;
				let right = right.eval(t)?;
				
				match (op, left, right) {
					// 4
					(BinOp::Exp, Value::Int(b), Value::Int(e)) =>
						Ok(Value::from(b.pow(e as u32))),
					(BinOp::Exp, _, _) =>
						Err("tried to get power of non-integer values"),
					
					// 5
					(BinOp::Mul, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l * r)),
					(BinOp::Mul, _, _) =>
						Err("tried multiplying non-integer values"),
					(BinOp::Div, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l / r)),
					(BinOp::Div, _, _) =>
						Err("tried dividing non-integer values"),
					(BinOp::Mod, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from((l % r + r) % r)),
					(BinOp::Mod, _, _) =>
						Err("tried getting remainder of non-integer values"),
					(BinOp::And, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l && r)),
					(BinOp::And, _, _) =>
						Err("tried ANDing non-boolean values"),
					
					// 6
					(BinOp::Add, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l + r)),
					(BinOp::Add, _, _) =>
						Err("tried adding non-integer values"),
					(BinOp::Sub, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l - r)),
					(BinOp::Sub, _, _) =>
						Err("tried subtracting non-integer values"),
					(BinOp::Or, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l || r)),
					(BinOp::Or, _, _) =>
						Err("tried ORing non-boolean values"),
					(BinOp::Xor, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l ^ r)),
					(BinOp::Xor, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l ^ r)),
					(BinOp::Xor, _, _) =>
						Err("tried XORing non-boolean or non-integer values"),
					
					// 7
					(BinOp::Eq, l, r) =>
						Ok(Value::from(l == r)),
					(BinOp::Ne, l, r) =>
						Ok(Value::from(l != r)),
					(BinOp::Lt, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l < r)),
					(BinOp::Lt, _, _) =>
						Err("tried comparing non-integer values"),
					(BinOp::Gt, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l > r)),
					(BinOp::Gt, _, _) =>
						Err("tried comparing non-integer values"),
					(BinOp::Le, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l <= r)),
					(BinOp::Le, _, _) =>
						Err("tried comparing non-integer values"),
					(BinOp::Ge, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l >= r)),
					(BinOp::Ge, _, _) =>
						Err("tried comparing non-integer values"),
				}
			}
			
			Expr::If(test, expr, else_expr) => {
				if test.eval(t)? == Value::Bool(true) {
					expr.eval(t)
				} else {
					else_expr.eval(t)
				}
			}
			
			Expr::Let(name, _, val, scope) => {
				let val = val.eval(t)?;
				let mut t_copy = t.clone();
				t_copy.push((name.clone(), val));
				scope.eval(&t_copy)
			}
		}
	}
}

impl From<Term> for Expr {
	fn from(f: Term) -> Self { Expr::Term(f) }
}
