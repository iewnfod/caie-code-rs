use crate::{Interpreter, RuntimeValue, Stmt};

impl Interpreter {
	pub fn if_stmt(&mut self, condition: crate::Expr, true_body: Box<Stmt>, false_body: Option<Box<Stmt>>) {
		let c = self.evaluate(condition).unwrap();
		let mut flag = false;
		match c {
			RuntimeValue::Bool(b) => {
				flag = b;
			},
			RuntimeValue::Int(i) => {
				flag = i != 0;
			},
			RuntimeValue::Float(f) => {
				flag = f != 0.0;
			},
			RuntimeValue::Null => {
				flag = false;
			},
			_ => {},
		};
		if flag {
			self.execute(*true_body);
		} else if let Some(false_body) = false_body {
			self.execute(*false_body);
		}
	}
}
