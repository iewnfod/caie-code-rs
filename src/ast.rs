use crate::core::RuntimeValue;

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
		name: ObjectAccess,
		index: Option<Box<Expr>>, // 支持数组元素访问，如 arr[0]
		span: Option<Span>,
	},
	Call {
		callee: Box<Expr>,
		args: Vec<Expr>,
		span: Option<Span>,
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectAccess {
	Direct {
		name: String,
		span: Option<Span>,
	},
	Deep {
		name: String,
		next: Box<ObjectAccess>, // 支持 a.b.c 这种深层次赋值
		span: Vec<Option<Span>>,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
	Assign {
		name: ObjectAccess,
		value: Expr,
		index: Option<Expr>, // 支持数组元素赋值，如 arr[0] = 10
		span: Option<Span>,
	},
	Block {
		stmts: Vec<Stmt>,
		span: Option<Span>,
	},
	TypeDecl {
		name: String,
		fields: Vec<(String, TypeDefinition)>, // field name and type name
		span: Option<Span>,
	},
	FuncDecl {
		name: String,
		params: Vec<(String, TypeDefinition)>, // param name and type name
		body: Box<Stmt>,
		span: Option<Span>,
	},
	VarDecl {
		name: String,
		var_type: TypeDefinition,
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
	}
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

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDefinition {
    Primitive(String), // "INTEGER", "REAL" ...
	Array { element_type: Box<TypeDefinition>, start: usize, end: usize }, // Array of fixed size
    Record { fields: Vec<(String, TypeDefinition)> }, // TYPE ... ENDTYPE
    Class { statements: Vec<Stmt> }, // CLASS ... ENDCLASS
}
