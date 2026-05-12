use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Environment, ObjectAccess, TypeDefinition};

#[derive(Debug, Clone, PartialEq)]
pub struct ObjPtr(pub Rc<RefCell<Object>>);

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Obj(ObjPtr),    // 所有的自定义结构体、函数、数组都是 Obj
    Null,
}

impl RuntimeValue {
	pub fn get_type(&self) -> TypeDefinition {
		match self {
			RuntimeValue::Int(_) => TypeDefinition::Primitive("INT".into()),
			RuntimeValue::Float(_) => TypeDefinition::Primitive("REAL".into()),
			RuntimeValue::Str(_) => TypeDefinition::Primitive("STRING".into()),
			RuntimeValue::Bool(_) => TypeDefinition::Primitive("BOOLEAN".into()),
			RuntimeValue::Obj(obj_ptr) => {
				let obj = obj_ptr.0.borrow();
				obj.definition.clone()
			},
			RuntimeValue::Null => TypeDefinition::Primitive("NULL".into()),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
	// 对象名
	pub name: String,
	// 类定义
	pub definition: TypeDefinition,
	// 存储函数名，即对应的函数的 Object，如 __get__、__set__ 等特殊方法，以及用户定义的方法
    pub methods: HashMap<String, RuntimeValue>,
    // 存储实例数据
    pub fields: HashMap<String, RuntimeValue>,
    // 用于继承或类型回溯
    pub prototype: Option<ObjPtr>,
	// 作用域，用于类，函数等需要作用域的对象
	pub environment: Option<Rc<RefCell<Environment>>>,
}

impl Object {
	pub fn new(name: String, definition: TypeDefinition) -> Self {
		Object {
			name,
			definition,
			methods: HashMap::new(),
			fields: HashMap::new(),
			prototype: None,
			environment: None,
		}
	}

	pub fn get_var_value(&self) -> Option<RuntimeValue> {
		self.get_value("__value__")
	}

	pub fn set_var_value(&mut self, value: RuntimeValue) {
		if value.get_type() == self.definition {
			self.set_value("__value__".to_string(), value);
		}
	}

	pub fn set_value(&mut self, field_name: String, value: RuntimeValue) {
		self.fields.insert(field_name, value);
	}

	pub fn set_value_with_depths(&mut self, field_names: ObjectAccess, value: RuntimeValue) {
		let name = field_names.clone();
		match name {
			ObjectAccess::Direct { .. } => {
				self.set_var_value(value);
			},
			ObjectAccess::Deep { name, next, .. } => {
				if let Some(RuntimeValue::Obj(obj_ptr)) = self.fields.get(&name) {
					// 尝试在 fields 中找到对象
					obj_ptr.0.borrow_mut().set_value_with_depths(*next, value);
				} else if
					self.environment.is_some() &&
					let Some(obj_ptr) = self.environment.as_ref().unwrap().borrow().get(name.to_string())
				{
					// 尝试在作用域中找到对象
					obj_ptr.0.borrow_mut().set_value_with_depths(*next, value);
				} else if let Some(proto) = &self.prototype {
					// 尝试在原型链中找到对象
					proto.0.borrow_mut().set_value_with_depths(field_names, value);
				} else {
					return; // 路径无效
				}
			}
		}
	}

	pub fn get_value(&self, field_name: &str) -> Option<RuntimeValue> {
		if let Some(value) = self.fields.get(field_name) {
			Some(value.clone())
		} else if let Some(proto) = &self.prototype {
			proto.0.borrow().get_value(field_name)
		} else {
			None
		}
	}

	pub fn get_value_with_depths(&self, field_names: ObjectAccess) -> Option<RuntimeValue> {
		let name = field_names.clone();
		match name {
			ObjectAccess::Direct { name, .. } => self.get_value(&name),
			ObjectAccess::Deep { name, next, .. } => {
				if let Some(RuntimeValue::Obj(obj_ptr)) = self.fields.get(&name) {
					obj_ptr.0.borrow().get_value_with_depths(*next)
				} else if
					self.environment.is_some() &&
					let Some(obj_ptr) = self.environment.as_ref().unwrap().borrow().get(name.to_string())
				{
					obj_ptr.0.borrow().get_value_with_depths(*next)
				} else if let Some(proto) = &self.prototype {
					proto.0.borrow().get_value_with_depths(field_names)
				} else {
					None // 路径无效
				}
			}
		}
	}

	pub fn construct(&mut self, args: Vec<RuntimeValue>) -> Option<RuntimeValue> {
		self.call_method("__construct__", args)
	}

	pub fn call_func(&mut self, args: Vec<RuntimeValue>) -> Option<RuntimeValue> {
		self.call_method("__call__", args)
	}

	pub fn call_method(&mut self, method_name: &str, args: Vec<RuntimeValue>) -> Option<RuntimeValue> {
		if let Some(method) = self.methods.get(method_name) {
			match method {
				RuntimeValue::Obj(func_ptr) => {
					let mut func_obj = func_ptr.0.borrow_mut();
					func_obj.call_func(args)
				},
				_ => unimplemented!(),
			}
		} else if let Some(proto) = &self.prototype {
			proto.0.borrow_mut().call_method(method_name, args)
		} else {
			unimplemented!()
		}
	}
}
