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

	pub fn evaluate(&mut self, expr: Expr) -> RuntimeValue {
		match expr {
			Expr::Literal { value, .. } => value,
			Expr::Binary { left, op, right, .. } => {
				let left_val = self.evaluate(*left);
				let right_val = self.evaluate(*right);
				match (left_val, right_val) {
					(RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
						match op {
							Op::Add => RuntimeValue::Int(l + r),
							Op::Sub => RuntimeValue::Int(l - r),
							Op::Mul => RuntimeValue::Int(l * r),
							Op::Div => RuntimeValue::Int(l / r),
							_ => unimplemented!(),
						}
					},
					_ => unimplemented!(),
				}
			},
			_ => unimplemented!(),
		}
	}

	fn assign(&mut self, name: ObjectAccess, value: Expr, index: Option<Expr>) {
		let val = self.evaluate(value);
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
			}
			_ => unimplemented!(),
		}
	}
}
