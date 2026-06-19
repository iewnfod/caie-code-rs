use std::{cell::RefCell, rc::Rc};

use crate::core::{array::ArrayObj, func::FuncObj, record::RecordObj};

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
	Null,
	Array(Rc<RefCell<ArrayObj>>),
    Record(Rc<RefCell<RecordObj>>),
    Func(Rc<RefCell<FuncObj>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    Null,
    Array(Box<Type>, usize, usize),
    Record(String),
    Func(Vec<Type>, Box<Type>),
}

impl RuntimeValue {
    pub fn to_string(&self) -> String {
        match self {
            RuntimeValue::Int(i) => i.to_string(),
            RuntimeValue::Float(f) => f.to_string(),
            RuntimeValue::Str(s) => s.clone(),
            RuntimeValue::Bool(b) => {
                if *b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            },
            RuntimeValue::Null => "NULL".to_string(),
            RuntimeValue::Array(arr_obj) => {
                arr_obj.borrow().to_string()
            },
            RuntimeValue::Func(func_obj) => {
                format!("<function {}>", func_obj.borrow().name())
            }
            _ => unimplemented!(),
        }
    }
}

pub fn default_type_value(ty: &Type) -> RuntimeValue {
    match ty {
        Type::Int => RuntimeValue::Int(0),
        Type::Float => RuntimeValue::Float(0.0),
        Type::Str => RuntimeValue::Str(String::new()),
        Type::Bool => RuntimeValue::Bool(false),
        Type::Null => RuntimeValue::Null,
        Type::Array(ele_ty, start, end) => ArrayObj::new_runtime(*ele_ty.clone(), *start, *end),
        _ => unimplemented!(),
    }
}
