use crate::{Environment, Expr, ObjectAccess, Op, RuntimeValue, Stmt};

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
	pub environment: Environment,  // 当前作用域
}

impl Interpreter {
	pub fn new() -> Self {
		Interpreter {
			environment: Environment::new(None),
		}
	}

	pub fn evaluate(&mut self, expr: Expr) -> Option<RuntimeValue> {
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
			Expr::Get { name, index, .. } => {
				self.get(name, index)
			}
			_ => unimplemented!(),
		}
	}

	fn binary_op(&self, left: RuntimeValue, op: Op, right: RuntimeValue) -> Option<RuntimeValue> {
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
			(RuntimeValue::Obj(l), RuntimeValue::Obj(_r)) => {
				let l = l.0.borrow();
				match op {
					Op::Eq => l.clone().call_method("__eq__", vec![right_clone]),
					Op::Neq => l.clone().call_method("__neq__", vec![right_clone]),
					Op::Gt => l.clone().call_method("__gt__", vec![right_clone]),
					Op::Lt => l.clone().call_method("__lt__", vec![right_clone]),
					Op::Gte => l.clone().call_method("__gte__", vec![right_clone]),
					Op::Lte => l.clone().call_method("__lte__", vec![right_clone]),
					Op::And => l.clone().call_method("__and__", vec![right_clone]),
					Op::Or => l.clone().call_method("__or__", vec![right_clone]),
					Op::Add => l.clone().call_method("__add__", vec![right_clone]),
					Op::Sub => l.clone().call_method("__sub__", vec![right_clone]),
					Op::Mul => l.clone().call_method("__mul__", vec![right_clone]),
					Op::Div => l.clone().call_method("__div__", vec![right_clone]),
					Op::Mod => l.clone().call_method("__mod__", vec![right_clone]),
					_ => unimplemented!(),
				}
			},
			_ => unimplemented!(),
		}
	}

	fn get(&mut self, name: ObjectAccess, index: Option<Box<Expr>>) -> Option<RuntimeValue> {
		if let Some(index) = index {
			let index = self.evaluate(*index);
			// get value with index
			None
		} else {
			match name {
				ObjectAccess::Direct { name, .. } => {
					if let Some(var) = self.environment.get(name) {
						var.0.borrow().get_var_value()
					} else {
						None
					}
				},
				ObjectAccess::Deep { name, next, .. } => {
					if let Some(var) = self.environment.get(name) {
						var.0.borrow().get_value_with_depths(*next)
					} else {
						None
					}
				}
			}
		}
	}

	fn assign(&mut self, name: ObjectAccess, value: Expr, index: Option<Expr>) {
		if let Some(val) = self.evaluate(value) {
			if let Some(index) = index {
				let index = self.evaluate(index);
				// set value with index
			} else {
				match name {
					ObjectAccess::Direct { name, .. } => {
						if let Some(var) = self.environment.get(name) {
							var.0.borrow_mut().set_var_value(val);
						}
					},
					ObjectAccess::Deep { name, next, .. } => {
						if let Some(var) = self.environment.get(name) {
							var.0.borrow_mut().set_value_with_depths(*next, val);
						}
					}
				}
			}
		}
	}

	fn print(&mut self, value: Vec<Expr>) {
		for (index, expr) in value.iter().enumerate() {
			if let Some(val) = self.evaluate(expr.clone()) {
				match val {
					RuntimeValue::Str(s) => print!("{}", s),
					RuntimeValue::Int(n) => print!("{}", n),
					RuntimeValue::Bool(b) => {
						if b {
							print!("TRUE");
						} else {
							print!("FALSE");
						}
					},
					RuntimeValue::Float(f) => print!("{}", f),
					RuntimeValue::Null => print!("NULL"),
					_ => print!("{:?}", val),
				}
				if index != value.len() - 1 {
					print!(" ");
				}
			} else {
				break;
			}
		}
		println!("");
	}

	pub fn execute(&mut self, stmt: Stmt) {
		match stmt {
			Stmt::Assign { name, value, index, .. } => {
				self.assign(name, value, index);
			},
			Stmt::VarDecl { name, var_type, .. } => {
				self.environment.define(name.clone(), var_type);
			},
			Stmt::Block { stmts, .. } => {
				for stmt in stmts {
					self.execute(stmt);
				}
			},
			Stmt::Print { value, .. } => {
				self.print(value);
			}
			_ => unimplemented!(),
		}
	}

	pub fn print_environment(&self) {
		println!("===== Environment State =====");
		println!("{:?}", self.environment);
		println!("=============================");
	}
}
