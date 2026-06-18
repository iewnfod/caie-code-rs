use crate::{Type, core::RuntimeValue};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
	pub line: usize,
	pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	Literal {
		value: RuntimeValue,
		span: Option<Span>,
	},
	Binary {
		left: Box<Expr>,
		op: Op,
		right: Box<Expr>,
		span: Option<Span>,
	},
	Get {
		name: String,
		span: Option<Span>,
	},
	Call {  // 调用函数
		name: String,
		args: Vec<Expr>,
		span: Option<Span>,
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
	Assign {
		name: String,
		value: Expr,
		span: Option<Span>,
	},
	Block {
		stmts: Vec<Stmt>,
		span: Option<Span>,
	},
	TypeDecl {
		name: String,
		fields: Vec<(String, Type)>, // field name and type name
		span: Option<Span>,
	},
	VarDecl {
		name: String,
		var_type: Type,
		span: Option<Span>,
	},
	Print {
		value: Vec<Expr>,
		span: Option<Span>,
	},
	If {
		condition: Expr,
		true_body: Box<Stmt>,
		false_body: Option<Box<Stmt>>,
		span: Option<Span>,
	},
	For {
		var_name: String,
		start: Expr,
		end: Expr,
		body: Box<Stmt>,
		span: Option<Span>,
	},
	Repeat {
		body: Box<Stmt>,
		condition: Expr,
		span: Option<Span>,
	},
	While {
		condition: Expr,
		body: Box<Stmt>,
		span: Option<Span>,
	},
	Expr {
		value: Expr,
		span: Option<Span>,
	},
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Eq,
	Neq,
	Lt,
	Lte,
	Gt,
	Gte,
	Not,
	And,
	Or,
}
