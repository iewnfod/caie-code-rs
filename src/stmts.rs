use crate::{Expr, Interpreter, RuntimeValue, Stmt, TypeDefinition};

impl Interpreter {
	pub fn if_stmt(&mut self, condition: Expr, true_body: Box<Stmt>, false_body: Option<Box<Stmt>>) {
		let c = self.evaluate(condition).unwrap();
		let flag = self.runtime2bool(c);
		if flag {
			self.execute(*true_body);
		} else if let Some(false_body) = false_body {
			self.execute(*false_body);
		}
	}

	pub fn for_stmt(&mut self, var_name: String, start: Expr, end: Expr, body: Box<Stmt>) {
		let s = self.evaluate(start).unwrap();
		let e = self.evaluate(end).unwrap();
		if let (RuntimeValue::Int(s), RuntimeValue::Int(e)) = (s, e) {
			for i in s..=e {
				self.environment.define(var_name.clone(), TypeDefinition::Primitive("INT".into()));
				if let Some(obj_ptr) = self.environment.objects.get(&var_name) {
					obj_ptr.0.borrow_mut().set_var_value(RuntimeValue::Int(i), None);
				}
				self.execute(*body.clone());
			}
		}
	}

	pub fn while_stmt(&mut self, condition: Expr, body: Box<Stmt>) {
		loop {
			let c = self.evaluate(condition.clone()).unwrap();
			let flag = self.runtime2bool(c);
			if flag {
				self.debug_print(format!("Condition passed"));
				self.execute(*body.clone());
			} else {
				self.debug_print(format!("Condition failed, exiting while loop"));
				break;
			}
		}
	}

	pub fn repeat_stmt(&mut self, body: Box<Stmt>, condition: Expr) {
		loop {
			self.execute(*body.clone());
			let c = self.evaluate(condition.clone()).unwrap();
			let flag = self.runtime2bool(c);
			if flag {
				self.debug_print(format!("Condition passed, exiting repeat loop"));
				break;
			} else {
				self.debug_print(format!("Condition failed, repeating loop"));
			}
		}
	}
}
