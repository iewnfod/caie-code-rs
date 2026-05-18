use crate::{Expr, Interpreter, ObjectAccess, RuntimeValue, Stmt, TypeDefinition};

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

	pub fn call_func(&mut self, name: ObjectAccess, args: Vec<Expr>) -> Option<RuntimeValue> {
		let evaluated_args: Vec<RuntimeValue> = args.into_iter().filter_map(|arg| self.evaluate(arg)).collect();
		if let Some(func_val) = self.get_method(name) {
			let definition = func_val.0.borrow().definition.clone();
			if let TypeDefinition::Func { params, body } = definition {
				self.new_scope();
				for (param, arg_val) in params.into_iter().zip(evaluated_args.into_iter()) {
					let (param_name, param_type) = param;
					if param_type == arg_val.get_type() {
						self.environment.define(param_name.clone(), param_type);
						self.environment.assign(ObjectAccess::Direct { name: param_name.clone(), span: None }, arg_val, None);
					} else {
						panic!("Argument type mismatch for parameter '{}', expect {:?}, found {:?}", param_name, param_type, arg_val.get_type());
					}
				}
				self.print_environment();
				self.execute(*body.clone());
				let result = self.get(ObjectAccess::Direct { name: "__return__".to_string(), span: None }, None);
				self.exit_scope();
				result
			} else {
				None
			}
		} else {
			None
		}
	}
}
