use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{ObjPtr, Object, RuntimeValue, TypeDefinition};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    // 当前层级的变量
    pub objects: HashMap<String, ObjPtr>,
    // 指向父级作用域
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
	pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
		Environment {
			objects: HashMap::new(),
			enclosing,
		}
	}

	fn define_var(&self, name: String, var_type: TypeDefinition) -> ObjPtr {
		let mut obj = Object::new(name.clone(), var_type.clone());
		match var_type {
			TypeDefinition::Primitive(t) => {
				match t.as_str() {
					"INT" => obj.set_var_value(RuntimeValue::Int(0)),
					"REAL" => obj.set_var_value(RuntimeValue::Float(0.0)),
					"STRING" => obj.set_var_value(RuntimeValue::Str(String::new())),
					"BOOLEAN" => obj.set_var_value(RuntimeValue::Bool(false)),
					_ => obj.set_var_value(RuntimeValue::Null),
				}
			},
			TypeDefinition::Record { fields } => {
				for (field, def) in fields {
					let f = field.clone();
					obj.set_value(field, RuntimeValue::Obj(self.define_var(f, def)));
				}
			},
			TypeDefinition::Class { .. } => {
				obj.set_var_value(RuntimeValue::Null);
			}
		}
		ObjPtr(Rc::new(RefCell::new(obj)))
	}

	pub fn define(&mut self, name: String, var_type: TypeDefinition) {
		let obj = self.define_var(name.clone(), var_type);
		self.objects.insert(name, obj);
	}

	pub fn get(&self, name: String) -> Option<ObjPtr> {
		if let Some(obj) = self.objects.get(&name) {
			Some(obj.clone())
		} else if let Some(enclosing) = &self.enclosing {
			enclosing.borrow().get(name)
		} else {
			None
		}
	}

	pub fn assign(&mut self, name: String, value: ObjPtr) -> bool {
		if self.objects.contains_key(&name) {
			self.objects.insert(name, value);
			true
		} else if let Some(enclosing) = &self.enclosing {
			enclosing.borrow_mut().assign(name, value)
		} else {
			false
		}
	}

	pub fn ancestor(&self, distance: usize) -> Option<Rc<RefCell<Environment>>> {
		let mut environment = self.enclosing.clone();
		for _ in 0..distance {
			if let Some(env) = environment {
				environment = env.borrow().enclosing.clone();
			} else {
				return None;
			}
		}
		environment
	}

	pub fn new_child(&self) -> Environment {
		Environment::new(Some(Rc::new(RefCell::new(self.clone()))))
	}
}

impl Default for Environment {
	fn default() -> Self {
		Environment::new(None)
	}
}
