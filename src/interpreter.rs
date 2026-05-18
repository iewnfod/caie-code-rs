use crate::{Environment, Expr, ObjPtr, ObjectAccess, Op, RuntimeValue, Stmt, TypeDefinition};
use colored::Colorize;

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
	pub environment: Environment,  // 当前作用域
	pub debug: bool, // 是否开启调试模式
}

impl Interpreter {
	pub fn new() -> Self {
		Interpreter {
			environment: Environment::new(None),
			debug: false,
		}
	}

	pub fn debug() -> Self {
		Interpreter {
			environment: Environment::new(None),
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
			Expr::Get { name, index, .. } => {
				self.get(name, index)
			},
			Expr::Call { name, args, .. } => {
				self.call_func(name, args)
			}
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

	pub fn get(&mut self, name: ObjectAccess, index: Option<Box<Expr>>) -> Option<RuntimeValue> {
		self.debug_print(format!("Getting value for: {:?} with index {:?}", name, index));
		let mut i = None;
		let mut result = None;
		if let Some(index) = index {
			i = self.evaluate(*index);
		}
		match name {
			ObjectAccess::Direct { name, .. } => {
				if let Some(var) = self.environment.get(name) {
					if i.is_none() && !matches!(var.0.borrow().definition, TypeDefinition::Primitive(_)) {
						result = Some(RuntimeValue::Obj(var.clone()));
					} else {
						result = var.0.borrow_mut().get_var_value(i);
					}
				}
			},
			ObjectAccess::Deep { name, next, .. } => {
				if let Some(var) = self.environment.get(name) {
					result = var.0.borrow_mut().get_value_with_depths(*next, i);
				}
			}
		}
		result
	}

	pub fn get_method(&mut self, name: ObjectAccess) -> Option<ObjPtr> {
		self.debug_print(format!("Getting method for: {:?}", name));
		let mut result = None;
		match name {
			ObjectAccess::Direct { name, .. } => {
				if let Some(var) = self.environment.get(name) {
					result = var.0.borrow_mut().get_func_ptr();
				}
			},
			ObjectAccess::Deep { name, next, .. } => {
				if let Some(var) = self.environment.get(name) {
					if let Some(obj) = var.0.borrow_mut().get_value_with_depths(*next, None) {
						if let RuntimeValue::Obj(obj_ptr) = obj {
							result = obj_ptr.0.borrow_mut().get_func_ptr();
						}
					}
				}
			}
		}
		result
	}

	pub fn assign(&mut self, name: ObjectAccess, value: Expr, index: Option<Expr>) {
		if let Some(val) = self.evaluate(value) {
			let mut i = None;
			if let Some(index) = index {
				i = self.evaluate(index);
			}
			self.environment.assign(name, val, i);
		}
	}

	pub fn get_print_string(&mut self, val: RuntimeValue) -> String {
		let t = val.get_type();
		let final_s = format!("{:?}", val);
		match val {
			RuntimeValue::Str(s) => s,
			RuntimeValue::Int(n) => n.to_string(),
			RuntimeValue::Bool(b) => {
				if b {
					"TRUE".to_string()
				} else {
					"FALSE".to_string()
				}
			},
			RuntimeValue::Float(f) => f.to_string(),
			RuntimeValue::Null => "NULL".to_string(),
			RuntimeValue::Obj(obj_ptr) => {
				match t {
					TypeDefinition::Array { start, end, .. } => {
						let mut elements = vec![];
						for i in start..=end {
							if let Some(elem) = obj_ptr.0.borrow_mut().get_value("", Some(RuntimeValue::Int((i as usize).try_into().unwrap()))) {
								elements.push(format!("{}", self.get_print_string(elem)));
							} else {
								elements.push("NULL".to_string());
							}
						}
						format!("[{}]", elements.join(", "))
					},
					_ => final_s,
				}
			},
		}
	}

	pub fn print(&mut self, value: Vec<Expr>) {
		for (index, expr) in value.iter().enumerate() {
			if let Some(val) = self.evaluate(expr.clone()) {
				let s = self.get_print_string(val);
				print!("{}", s);
				if index != value.len() - 1 {
					print!(" ");
				}
			} else {
				break;
			}
		}
		println!("");
	}

	pub fn new_scope(&mut self) {
		self.debug_print("New Scope");
		self.environment = self.environment.new_child();
	}

	pub fn exit_scope(&mut self) {
		self.debug_print("Exit Scope");
		self.print_environment();
		if let Some(parent) = self.environment.ancestor(0) {
			self.environment = parent.take();
			self.debug_print("Returned to parent scope");
		} else {
			self.debug_print("No parent scope found, staying in current scope");
		}
	}

	pub fn execute(&mut self, stmt: Stmt) {
		match stmt {
			Stmt::Assign { name, value, index, .. } => {
				self.debug_print(format!("Assign {:?} to {:?} with index {:?}", value, name, index));
				self.assign(name, value, index);
			},
			Stmt::VarDecl { name, var_type, .. } => {
				self.debug_print(format!("Declaring variable: {}", name));
				self.environment.define(name.clone(), var_type);
			},
			Stmt::Block { stmts, .. } => {
				self.new_scope();
				for stmt in stmts {
					self.execute(stmt);
				}
				self.exit_scope();
			},
			Stmt::Print { value, .. } => {
				self.debug_print(format!("Printing values: {:?}", value));
				self.print(value);
			},
			Stmt::If { condition, true_body, false_body, .. } => {
				self.debug_print(format!("Executing if statement with condition: {:?}", condition));
				self.if_stmt(condition, true_body, false_body);
			},
			Stmt::For { var_name, start, end, body, .. } => {
				self.debug_print(format!("Executing for loop with variable: {}, start: {:?}, end: {:?}", var_name, start, end));
				self.for_stmt(var_name, start, end, body);
			},
			Stmt::While { condition, body, .. } => {
				self.debug_print(format!("Executing while loop with condition: {:?}", condition));
				self.while_stmt(condition, body);
			},
			Stmt::Repeat { body, condition, .. } => {
				self.debug_print(format!("Executing repeat loop with condition: {:?}", condition));
				self.repeat_stmt(body, condition);
			},
			Stmt::FuncDecl { name, params, body, .. } => {
				self.debug_print(format!("Declaring function: {}", name));
				self.environment.define_func(name, params, body);
			},
			Stmt::Expr { value, .. } => {
				self.debug_print(format!("Evaluating expression statement: {:?}", value));
				self.evaluate(value);
			}
			_ => unimplemented!(),
		}
	}

	pub fn print_environment(&self) {
		self.debug_print("===== Environment State =====");
		self.debug_print(format!("{:?}", self.environment));
		self.debug_print("=============================");
	}
}
