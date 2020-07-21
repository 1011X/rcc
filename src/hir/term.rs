use super::*;

#[derive(Debug, Clone)]
pub enum Term {
	Lit(Literal),
	LVal(LValue),
}

impl Term {
	pub fn get_type(&self) -> Type {
		match self {
			Term::Lit(lit) => lit.get_type(),
			Term::LVal(_)  => todo!(),
		}
	}
}

impl Eval for Term {
	fn eval(&self, t: &Scope) -> EvalResult {
	    match self {
	        Term::Lit(lit)   => lit.eval(t),
	        Term::LVal(lval) => lval.eval(t),
	    }
	}
}

impl From<Literal> for Term {
    fn from(lit: Literal) -> Self { Term::Lit(lit) }
}

impl From<LValue> for Term {
    fn from(lval: LValue) -> Self { Term::LVal(lval) }
}

impl From<ast::Term> for Term {
	fn from(v: ast::Term) -> Self {
		match v {
			ast::Term::Lit(lit) => Term::Lit(lit.into()),
			ast::Term::LVal(lval) => Term::LVal(lval.into()),
		}
	}
}