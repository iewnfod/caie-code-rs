use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{ObjPtr, Object, ObjectAccess, RuntimeValue, Stmt, TypeDefinition};

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
					"INT" => obj.set_var_value(RuntimeValue::Int(0), None),
					"REAL" => obj.set_var_value(RuntimeValue::Float(0.0), None),
					"STRING" => obj.set_var_value(RuntimeValue::Str(String::new()), None),
					"BOOLEAN" => obj.set_var_value(RuntimeValue::Bool(false), None),
					_ => obj.set_var_value(RuntimeValue::Null, None),
				}
			},
			TypeDefinition::Record { fields } => {
				for (field, def) in fields {
					let f = field.clone();
					obj.set_value(field, RuntimeValue::Obj(self.define_var(f, def)), None);
				}
			},
			TypeDefinition::Array { element_type, start, end } => {
				for i in start..=end {
					let index = format!("__{}__", i);
					let t = *element_type.clone();
					let var = self.define_var(index.clone(), t.clone());
					match t {
						TypeDefinition::Primitive(_) => {
							obj.set_value(index, var.0.borrow_mut().get_var_value(None).unwrap(), None);
						},
						_ => {
							obj.set_value(index, RuntimeValue::Obj(var), None);
						}
					}
				}
			},
			_ => {},
		}
		ObjPtr(Rc::new(RefCell::new(obj)))
	}

	pub fn define(&mut self, name: String, var_type: TypeDefinition) {
		let obj = self.define_var(name.clone(), var_type);
		self.objects.insert(name, obj);
	}

	pub fn get(&mut self, name: String) -> Option<ObjPtr> {
		if let Some(obj) = self.objects.get(&name) {
			Some(obj.clone())
		} else if let Some(enclosing) = &self.enclosing {
			enclosing.borrow_mut().get(name)
		} else {
			None
		}
	}

	pub fn assign(&mut self, name: ObjectAccess, value: RuntimeValue, index: Option<RuntimeValue>) {
		let name_clone = name.clone();
		match name {
			ObjectAccess::Direct { name, .. } => {
				if let Some(var) = self.objects.get(&name) {
					var.0.borrow_mut().set_var_value(value, index);
				} else if let Some(enclosing) = &self.enclosing {
					enclosing.borrow_mut().assign(name_clone, value, index);
				} else {
					panic!("Undefined variable: {}", name);
				}
			},
			ObjectAccess::Deep { name, next, .. } => {
				if let Some(var) = self.objects.get(&name) {
					var.0.borrow_mut().set_value_with_depths(*next, value, index);
				} else if let Some(enclosing) = &self.enclosing {
					enclosing.borrow_mut().assign(name_clone, value, index);
				} else {
					panic!("Undefined variable: {}", name);
				}
			}
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

	pub fn define_func(&mut self, name: String, params: Vec<(String, TypeDefinition)>, body: Box<Stmt>) {
		let func_obj = Object {
			name: name.clone(),
			definition: TypeDefinition::Func { params: params.clone(), body: body.clone() },
			fields: HashMap::new(),
			prototype: None,
			environment: Some(Rc::new(RefCell::new(self.clone()))),
		};
		self.objects.insert(name, ObjPtr(Rc::new(RefCell::new(func_obj))));
	}
}

impl Default for Environment {
	fn default() -> Self {
		Environment::new(None)
	}
}
