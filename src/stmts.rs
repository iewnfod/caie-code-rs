use crate::{Expr, Flow, Interpreter, RuntimeValue, Span, Stmt};

impl Interpreter {
	pub fn execute(&mut self, stmt: Stmt) -> Flow {
		let flow = match stmt {
			Stmt::Assign { name, value, .. } => {
				self.debug_print(format!("Assign {:?} to {:?}", value, name));
				self.set(name, value);
				Flow::Normal
			},
			Stmt::VarDecl { name, var_type, .. } => {
				self.debug_print(format!("Declaring variable: {}", name));
				self.define(name, var_type);
				Flow::Normal
			},
			Stmt::Block { stmts, .. } => {
				let child = crate::Scope::child(self.current_scope.clone());
				let saved = std::mem::replace(&mut self.current_scope, child);
				let mut flow = Flow::Normal;
				for stmt in stmts {
					flow = self.execute(stmt);
					if !matches!(flow, Flow::Normal) {
						self.debug_print(format!("Flow changed to {:?}, exiting block", flow));
						break;
					}
				}
				self.current_scope = saved;
				flow
			},
			Stmt::If { condition, true_body, false_body, .. } => {
				self.debug_print(format!("Executing if statement with condition: {:?}", condition));
				self.if_stmt(condition, true_body, false_body)
			},
			Stmt::For { var_name, start, end, body, span, .. } => {
				self.debug_print(format!("Executing for loop with variable: {}, start: {:?}, end: {:?}", var_name, start, end));
				self.for_stmt(var_name, start, end, body, span)
			},
			Stmt::While { condition, body, .. } => {
				self.debug_print(format!("Executing while loop with condition: {:?}", condition));
				self.while_stmt(condition, body)
			},
			Stmt::Repeat { body, condition, .. } => {
				self.debug_print(format!("Executing repeat loop with condition: {:?}", condition));
				self.repeat_stmt(body, condition)
			},
			Stmt::Expr { value, .. } => {
				self.debug_print(format!("Evaluating expression statement: {:?}", value));
				self.evaluate(value);
				Flow::Normal
			},
			Stmt::Print { value, .. } => {
				self.debug_print(format!("Executing print statement with value: {:?}", value));
				self.print(value);
				Flow::Normal
			},
			Stmt::IndexAssign { target, index, value, .. } => {
				self.debug_print(format!("Index assign to {:?} with value: {:?} and index: {:?}", &target, &value, &index));
				self.index_set(target, index, value);
				Flow::Normal
			},
			Stmt::FuncDecl { name, params, return_type, body, .. } => {
				self.debug_print(format!("Declaring function: {} with params: {:?} and return type: {:?}", name, params, return_type));
				let func = crate::core::FuncObj::new_runtime(
					name.clone(), 
					params,
					*body,
					self.current_scope.clone(),
					return_type
				);
				crate::scope::define(&self.current_scope, name, func);
				Flow::Normal
			},
			Stmt::Return { value, .. } => {
				self.debug_print(format!("Executing return statement with value: {:?}", value));
				if let Some(v_expr) = value {
					if let Some(val) = self.evaluate(v_expr) {
						Flow::Return(val)
					} else {
						Flow::Return(RuntimeValue::Null)
					}
				} else {
					Flow::Return(RuntimeValue::Null)
				}
			},
			Stmt::Break { .. } => {
				self.debug_print(format!("Executing break statement"));
				Flow::Break
			},
			Stmt::Continue { .. } => {
				self.debug_print(format!("Executing continue statement"));
				Flow::Continue
			},
			_ => {
				Flow::Normal
			},
		};
		self.print_scope();
		flow
	}

	pub fn if_stmt(&mut self, condition: Expr, true_body: Box<Stmt>, false_body: Option<Box<Stmt>>) -> Flow {
		let c = self.evaluate(condition).unwrap();
		let flag = self.runtime2bool(c);
		if flag {
			self.execute(*true_body)
		} else if let Some(false_body) = false_body {
			self.execute(*false_body)
		} else {
			Flow::Normal
		}
	}

	pub fn for_stmt(&mut self, var_name: String, start: Expr, end: Expr, body: Box<Stmt>, span: Option<Span>) -> Flow {
		let s = self.evaluate(start).unwrap();
		let e = self.evaluate(end).unwrap();
		if let (RuntimeValue::Int(s), RuntimeValue::Int(e)) = (s, e) {
			self.define(var_name.clone(), crate::Type::Int);
			for i in s..=e {
				self.set(var_name.clone(), Expr::Literal { value: RuntimeValue::Int(i), span: span.clone() });
				let flow = self.execute(*body.clone());
				match flow {
					Flow::Continue => continue,
					Flow::Break => return Flow::Normal,
					Flow::Return(val) => return Flow::Return(val),
					_ => (),
				}
			}
		}
		Flow::Normal
	}

	pub fn while_stmt(&mut self, condition: Expr, body: Box<Stmt>) -> Flow {
		loop {
			let c = self.evaluate(condition.clone()).unwrap();
			let flag = self.runtime2bool(c);
			if flag {
				self.debug_print(format!("Condition passed"));
				let flow = self.execute(*body.clone());
				match flow {
					Flow::Continue => continue,
					Flow::Break => return Flow::Normal,
					Flow::Return(val) => return Flow::Return(val),
					_ => (),
				}
			} else {
				self.debug_print(format!("Condition failed, exiting while loop"));
				return Flow::Normal;
			}
		}
	}

	pub fn repeat_stmt(&mut self, body: Box<Stmt>, condition: Expr) -> Flow {
		loop {
			let flow = self.execute(*body.clone());
			match flow {
				Flow::Break => return Flow::Normal,
				Flow::Return(val) => return Flow::Return(val),
				_ => (),
			}
			let c = self.evaluate(condition.clone()).unwrap();
			let flag = self.runtime2bool(c);
			if flag {
				self.debug_print(format!("Condition passed, exiting repeat loop"));
				return Flow::Normal;
			} else {
				self.debug_print(format!("Condition failed, repeating loop"));
			}
		}
	}
}
