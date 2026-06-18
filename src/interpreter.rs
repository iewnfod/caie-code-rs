use crate::{Expr, Op, RuntimeValue, Scope, ScopeRef, Stmt, Type};
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Interpreter {
	pub current_scope: ScopeRef,
	pub debug: bool, // 是否开启调试模式
}

impl Interpreter {
	pub fn new() -> Self {
		Interpreter {
			current_scope: Scope::root(),
			debug: false,
		}
	}

	pub fn debug() -> Self {
		Interpreter {
			current_scope: Scope::root(),
			debug: true,
		}
	}

	pub fn debug_print<T: ToString>(&self, message: T) {
		if self.debug {
			println!("{}", message.to_string().purple());
		}
	}

	pub fn runtime2bool(&self, val: RuntimeValue) -> bool {
		match val {
			RuntimeValue::Bool(b) => b,
			RuntimeValue::Int(i) => i != 0,
			RuntimeValue::Float(f) => f != 0.0,
			RuntimeValue::Null => false,
			_ => false, // 对于对象和字符串等非空值，默认视为 false
		}
	}

	pub fn evaluate(&mut self, expr: Expr) -> Option<RuntimeValue> {
		self.debug_print(format!("Evaluating expression: {:?}", expr));
		match expr {
			Expr::Literal { value, .. } => Some(value),
			Expr::Binary { left, op, right, .. } => {
				let left_val = self.evaluate(*left);
				let right_val = self.evaluate(*right);
				if let Some(left) = left_val && let Some(right) = right_val {
					self.binary_op(left, op, right)
				} else {
					None
				}
			},
			Expr::Get { name, .. } => {
				self.get(name)
			},
			_ => unimplemented!(),
		}
	}

	pub fn binary_op(&self, left: RuntimeValue, op: Op, right: RuntimeValue) -> Option<RuntimeValue> {
		self.debug_print(format!("Evaluating binary operation: {:?} {:?} {:?}", left, op, right));
		let right_clone = right.clone();
		match (left, right) {
			(RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
				match op {
					Op::Add => Some(RuntimeValue::Int(l + r)),
					Op::Sub => Some(RuntimeValue::Int(l - r)),
					Op::Mul => Some(RuntimeValue::Int(l * r)),
					Op::Div => Some(RuntimeValue::Int(l / r)),
					Op::And => Some(RuntimeValue::Bool(l != 0 && r != 0)),
					Op::Or => Some(RuntimeValue::Bool(l != 0 || r != 0)),
					Op::Eq => Some(RuntimeValue::Bool(l == r)),
					Op::Mod => Some(RuntimeValue::Int(l % r)),
					Op::Gt => Some(RuntimeValue::Bool(l > r)),
					Op::Lt => Some(RuntimeValue::Bool(l < r)),
					Op::Gte => Some(RuntimeValue::Bool(l >= r)),
					Op::Lte => Some(RuntimeValue::Bool(l <= r)),
					Op::Neq => Some(RuntimeValue::Bool(l != r)),
					_ => unimplemented!(),
				}
			},
			(RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
				match op {
					Op::Add => Some(RuntimeValue::Float(l + r)),
					Op::Sub => Some(RuntimeValue::Float(l - r)),
					Op::Mul => Some(RuntimeValue::Float(l * r)),
					Op::Div => Some(RuntimeValue::Float(l / r)),
					Op::And => Some(RuntimeValue::Bool(l != 0.0 && r != 0.0)),
					Op::Or => Some(RuntimeValue::Bool(l != 0.0 || r != 0.0)),
					Op::Eq => Some(RuntimeValue::Bool(l == r)),
					Op::Gt => Some(RuntimeValue::Bool(l > r)),
					Op::Lt => Some(RuntimeValue::Bool(l < r)),
					Op::Gte => Some(RuntimeValue::Bool(l >= r)),
					Op::Lte => Some(RuntimeValue::Bool(l <= r)),
					Op::Neq => Some(RuntimeValue::Bool(l != r)),
					_ => unimplemented!(),
				}
			},
			(RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
				match op {
					Op::Add => Some(RuntimeValue::Str(l + &r)),
					Op::Eq => Some(RuntimeValue::Bool(l == r)),
					Op::Gt => Some(RuntimeValue::Bool(l > r)),
					Op::Lt => Some(RuntimeValue::Bool(l < r)),
					Op::Gte => Some(RuntimeValue::Bool(l >= r)),
					Op::Lte => Some(RuntimeValue::Bool(l <= r)),
					Op::Neq => Some(RuntimeValue::Bool(l != r)),
					_ => unimplemented!(),
				}
			},
			(RuntimeValue::Bool(l), RuntimeValue::Bool(r)) => {
				match op {
					Op::And => Some(RuntimeValue::Bool(l && r)),
					Op::Or => Some(RuntimeValue::Bool(l || r)),
					Op::Eq => Some(RuntimeValue::Bool(l == r)),
					Op::Neq => Some(RuntimeValue::Bool(l != r)),
					_ => unimplemented!(),
				}
			},
			(RuntimeValue::Null, RuntimeValue::Null) => {
				match op {
					Op::Eq => Some(RuntimeValue::Bool(true)),
					Op::Neq => Some(RuntimeValue::Bool(false)),
					_ => unimplemented!(),
				}
			},
			// (RuntimeValue::Obj(l), RuntimeValue::Obj(_r)) => {
			// 	let l = l.0.borrow();
			// 	match op {
			// 		Op::Eq => l.clone().call_method("__eq__", vec![right_clone]),
			// 		Op::Neq => l.clone().call_method("__neq__", vec![right_clone]),
			// 		Op::Gt => l.clone().call_method("__gt__", vec![right_clone]),
			// 		Op::Lt => l.clone().call_method("__lt__", vec![right_clone]),
			// 		Op::Gte => l.clone().call_method("__gte__", vec![right_clone]),
			// 		Op::Lte => l.clone().call_method("__lte__", vec![right_clone]),
			// 		Op::And => l.clone().call_method("__and__", vec![right_clone]),
			// 		Op::Or => l.clone().call_method("__or__", vec![right_clone]),
			// 		Op::Add => l.clone().call_method("__add__", vec![right_clone]),
			// 		Op::Sub => l.clone().call_method("__sub__", vec![right_clone]),
			// 		Op::Mul => l.clone().call_method("__mul__", vec![right_clone]),
			// 		Op::Div => l.clone().call_method("__div__", vec![right_clone]),
			// 		Op::Mod => l.clone().call_method("__mod__", vec![right_clone]),
			// 		_ => unimplemented!(),
			// 	}
			// },
			_ => unimplemented!(),
		}
	}

	pub fn get(&mut self, name: String) -> Option<RuntimeValue> {
		self.debug_print(format!("Getting value for: {:?}", name));
		crate::scope::get(&self.current_scope, &name)
	}

	pub fn set(&mut self, name: String, value: Expr) -> bool {
		self.debug_print(format!("Setting value for: {:?} to {:?}", name, value));
		if let Some(value) = self.evaluate(value) {
			crate::scope::set(&self.current_scope, &name, value)
		} else {
			false
		}
	}

	pub fn define(&mut self, name: String, var_type: Type) {
		self.debug_print(format!("Defining variable: {:?} with type {:?}", name, var_type));
		let value = match var_type {
			Type::Int => RuntimeValue::Int(0),
			Type::Float => RuntimeValue::Float(0.0),
			Type::Str => RuntimeValue::Str(String::new()),
			Type::Bool => RuntimeValue::Bool(false),
			Type::Null => RuntimeValue::Null,
			_ => unimplemented!(),
		};
		crate::scope::define(&self.current_scope, name, value);
	}

	pub fn print(&mut self, value: Vec<Expr>) {
		for expr in value {
			if let Some(val) = self.evaluate(expr) {
				print!("{:?} ", val);
			} else {
				print!("None ");
			}
		}
		println!();
	}

	pub fn execute(&mut self, stmt: Stmt) {
		match stmt {
			Stmt::Assign { name, value, .. } => {
				self.debug_print(format!("Assign {:?} to {:?}", value, name));
				self.set(name, value);
			},
			Stmt::VarDecl { name, var_type, .. } => {
				self.debug_print(format!("Declaring variable: {}", name));
				self.define(name, var_type);
			},
			Stmt::Block { stmts, .. } => {
				let child = Scope::child(self.current_scope.clone());
				let saved = std::mem::replace(&mut self.current_scope, child);
				for stmt in stmts {
					self.execute(stmt);
				}
				self.current_scope = saved;
			},
			Stmt::If { condition, true_body, false_body, .. } => {
				self.debug_print(format!("Executing if statement with condition: {:?}", condition));
				self.if_stmt(condition, true_body, false_body);
			},
			Stmt::For { var_name, start, end, body, span, .. } => {
				self.debug_print(format!("Executing for loop with variable: {}, start: {:?}, end: {:?}", var_name, start, end));
				self.for_stmt(var_name, start, end, body, span);
			},
			Stmt::While { condition, body, .. } => {
				self.debug_print(format!("Executing while loop with condition: {:?}", condition));
				self.while_stmt(condition, body);
			},
			Stmt::Repeat { body, condition, .. } => {
				self.debug_print(format!("Executing repeat loop with condition: {:?}", condition));
				self.repeat_stmt(body, condition);
			},
			Stmt::Expr { value, .. } => {
				self.debug_print(format!("Evaluating expression statement: {:?}", value));
				self.evaluate(value);
			},
			Stmt::Print { value, .. } => {
				self.debug_print(format!("Executing print statement with value: {:?}", value));
				self.print(value);
			},
			_ => unimplemented!(),
		}
	}

	pub fn print_environment(&self) {
		self.debug_print("===== Environment State =====");
		self.debug_print(format!("{:?}", self.current_scope));
		self.debug_print("=============================");
	}
}
