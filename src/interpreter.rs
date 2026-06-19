use crate::{CpcResult, Expr, Op, RuntimeValue, Scope, ScopeRef, Type, default_type_value, get_value_type, utils::debug_print};

#[derive(Debug, Clone, PartialEq)]
pub enum Flow {
    Return(RuntimeValue),
    Break,
    Continue,
    Normal,
}

#[derive(Debug, Clone)]
pub struct Interpreter {
	pub current_scope: ScopeRef,
	pub debug: bool, // 是否开启调试模式
	pub show_time: bool, // 是否显示执行时间
}

impl Interpreter {
	pub fn new() -> Self {
		Interpreter {
			current_scope: Scope::root(),
			debug: false,
			show_time: false,
		}
	}

	pub fn debug() -> Self {
		Interpreter {
			current_scope: Scope::root(),
			debug: true,
			show_time: false,
		}
	}

	pub fn debug_print<T: ToString>(&self, message: T) {
		if self.debug {
			debug_print(self.debug, message);
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

	pub fn evaluate(&mut self, expr: Expr) -> CpcResult<RuntimeValue> {
		self.debug_print(format!("Evaluating expression: {:?}", expr));
		match expr {
			Expr::Literal { value, .. } => Ok(value),
			Expr::Binary { left, op, right, .. } => {
				let left_val = self.evaluate(*left)?;
				let right_val = self.evaluate(*right)?;
				self.binary_op(left_val, op, right_val)
			},
			Expr::Get { name, .. } => {
				self.get(name)
			},
			Expr::Index { target, index, span } => {
				let container = self.evaluate(*target)?;
				let idx = self.evaluate(*index)?;
				match (container, idx) {
					(RuntimeValue::Array(arr), RuntimeValue::Int(i)) => {
						arr.borrow().get(i as usize)
					},
					_ => Err(crate::CpcError::Runtime { 
						span: span, kind: crate::RuntimeErrorKind::Other("Invalid index operation".into()) 
					}),
				}
			},
			Expr::Call { name, args, .. } => {
				self.call_func(name, args)
			},
			Expr::Unary { op, operand, .. } => {
				let value = self.evaluate(*operand)?;
				self.unary_op(op, value)
			},
			_ => {
				Err(crate::CpcError::Runtime { 
					span: None, kind: crate::RuntimeErrorKind::Other("Unsupported expression".into()) 
				})
			}
		}
	}

	pub fn unary_op(&self, op: Op, value: RuntimeValue) -> CpcResult<RuntimeValue> {
		self.debug_print(format!("Evaluating unary operation: {:?} {:?}", op, value));
		let value_clone = value.clone();
		match (op, value) {
			(Op::Neg, RuntimeValue::Int(i)) => Ok(RuntimeValue::Int(-i)),
			(Op::Neg, RuntimeValue::Float(f)) => Ok(RuntimeValue::Float(-f)),
			(Op::Not, RuntimeValue::Bool(b)) => Ok(RuntimeValue::Bool(!b)),
			_ => Err(crate::CpcError::Runtime { 
				span: None, kind: crate::RuntimeErrorKind::InvalidUnaryOp { op, operand: get_value_type(&value_clone)? }
			}),
		}
	}

	pub fn binary_op(&self, left: RuntimeValue, op: Op, right: RuntimeValue) -> CpcResult<RuntimeValue> {
		self.debug_print(format!("Evaluating binary operation: {:?} {:?} {:?}", left, op, right));
		let left_clone = left.clone();
		let right_clone = right.clone();
		match (left, right) {
			(RuntimeValue::Int(l), RuntimeValue::Int(r)) => {
				match op {
					Op::Add => Ok(RuntimeValue::Int(l + r)),
					Op::Sub => Ok(RuntimeValue::Int(l - r)),
					Op::Mul => Ok(RuntimeValue::Int(l * r)),
					Op::Div => Ok(RuntimeValue::Int(l / r)),
					Op::And => Ok(RuntimeValue::Bool(l != 0 && r != 0)),
					Op::Or => Ok(RuntimeValue::Bool(l != 0 || r != 0)),
					Op::Eq => Ok(RuntimeValue::Bool(l == r)),
					Op::Mod => Ok(RuntimeValue::Int(l % r)),
					Op::Gt => Ok(RuntimeValue::Bool(l > r)),
					Op::Lt => Ok(RuntimeValue::Bool(l < r)),
					Op::Gte => Ok(RuntimeValue::Bool(l >= r)),
					Op::Lte => Ok(RuntimeValue::Bool(l <= r)),
					Op::Neq => Ok(RuntimeValue::Bool(l != r)),
					_ => Err(crate::CpcError::Runtime {
						span: None,
						kind: crate::RuntimeErrorKind::InvalidOp { op, left: get_value_type(&left_clone)?, right: get_value_type(&right_clone)? },
					}),
				}
			},
			(RuntimeValue::Float(l), RuntimeValue::Float(r)) => {
				match op {
					Op::Add => Ok(RuntimeValue::Float(l + r)),
					Op::Sub => Ok(RuntimeValue::Float(l - r)),
					Op::Mul => Ok(RuntimeValue::Float(l * r)),
					Op::Div => Ok(RuntimeValue::Float(l / r)),
					Op::And => Ok(RuntimeValue::Bool(l != 0.0 && r != 0.0)),
					Op::Or => Ok(RuntimeValue::Bool(l != 0.0 || r != 0.0)),
					Op::Eq => Ok(RuntimeValue::Bool(l == r)),
					Op::Gt => Ok(RuntimeValue::Bool(l > r)),
					Op::Lt => Ok(RuntimeValue::Bool(l < r)),
					Op::Gte => Ok(RuntimeValue::Bool(l >= r)),
					Op::Lte => Ok(RuntimeValue::Bool(l <= r)),
					Op::Neq => Ok(RuntimeValue::Bool(l != r)),
					_ => Err(crate::CpcError::Runtime {
						span: None,
						kind: crate::RuntimeErrorKind::InvalidOp { op, left: get_value_type(&left_clone)?, right: get_value_type(&right_clone)? },
					}),
				}
			},
			(RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
				match op {
					Op::Add => Ok(RuntimeValue::Str(l + &r)),
					Op::Eq => Ok(RuntimeValue::Bool(l == r)),
					Op::Gt => Ok(RuntimeValue::Bool(l > r)),
					Op::Lt => Ok(RuntimeValue::Bool(l < r)),
					Op::Gte => Ok(RuntimeValue::Bool(l >= r)),
					Op::Lte => Ok(RuntimeValue::Bool(l <= r)),
					Op::Neq => Ok(RuntimeValue::Bool(l != r)),
					Op::Concat => Ok(RuntimeValue::Str(l + &r)),
					_ => Err(crate::CpcError::Runtime {
						span: None,
						kind: crate::RuntimeErrorKind::InvalidOp { op, left: get_value_type(&left_clone)?, right: get_value_type(&right_clone)? },
					}),
				}
			},
			(RuntimeValue::Bool(l), RuntimeValue::Bool(r)) => {
				match op {
					Op::And => Ok(RuntimeValue::Bool(l && r)),
					Op::Or => Ok(RuntimeValue::Bool(l || r)),
					Op::Eq => Ok(RuntimeValue::Bool(l == r)),
					Op::Neq => Ok(RuntimeValue::Bool(l != r)),
					_ => Err(crate::CpcError::Runtime {
						span: None,
						kind: crate::RuntimeErrorKind::InvalidOp { op, left: get_value_type(&left_clone)?, right: get_value_type(&right_clone)? },
					}),
				}
			},
			(RuntimeValue::Null, RuntimeValue::Null) => {
				match op {
					Op::Eq => Ok(RuntimeValue::Bool(true)),
					Op::Neq => Ok(RuntimeValue::Bool(false)),
					_ => Err(crate::CpcError::Runtime {
						span: None,
						kind: crate::RuntimeErrorKind::InvalidOp { op, left: get_value_type(&left_clone)?, right: get_value_type(&right_clone)? },
					}),
				}
			},
			_ => Err(crate::CpcError::Runtime {
				span: None,
				kind: crate::RuntimeErrorKind::InvalidOp { op, left: get_value_type(&left_clone)?, right: get_value_type(&right_clone)? },
			}),
		}
	}

	pub fn get(&mut self, name: String) -> CpcResult<RuntimeValue> {
		self.debug_print(format!("Getting value for: {:?}", name));
		crate::scope::get(&self.current_scope, &name)
	}

	pub fn set(&mut self, name: String, value: Expr) -> CpcResult<()> {
		self.debug_print(format!("Setting value for: {:?} to {:?}", name, value));
		match self.evaluate(value) {
			Ok(value) => crate::scope::set(&self.current_scope, &name, value),
			Err(e) => Err(e),
		}
	}

	pub fn define(&mut self, name: String, var_type: Type) -> CpcResult<()> {
		self.debug_print(format!("Defining variable: {:?} with type {:?}", name, var_type));
		let value = default_type_value(&var_type)?;
		crate::scope::define(&self.current_scope, name, value);
		Ok(())
	}

	pub fn print(&mut self, value: Vec<Expr>) -> CpcResult<()> {
		for expr in value {
			let val = self.evaluate(expr)?;
			print!("{} ", val.to_string());
		}
		println!();
		Ok(())
	}

	pub fn index_set(&mut self, target: Expr, index: Expr, value: Expr) -> CpcResult<()> {
		let value_val = self.evaluate(value)?;
		let index_val = self.evaluate(index)?;
		let target_val = self.evaluate(target)?;
		let i = match index_val {
			RuntimeValue::Int(i) => i as usize,
			_ => return Err(crate::CpcError::Runtime {
				span: None,
				kind: crate::RuntimeErrorKind::InvalidIndex(index_val),
			}),
		};
		match target_val {
			RuntimeValue::Array(arr) => {
				arr.borrow_mut().set(i, value_val)
			},
			_ => return Err(crate::CpcError::Runtime {
				span: None,
				kind: crate::RuntimeErrorKind::NotIndexable(get_value_type(&target_val)?),
			}),
		}
	}

	pub fn print_scope(&self) {
		if self.debug {
			self.debug_print("======== Scope State ========");
			crate::scope::print_scope(&self.current_scope, self.debug, 0);
			self.debug_print("=============================");			
		}
	}
}
