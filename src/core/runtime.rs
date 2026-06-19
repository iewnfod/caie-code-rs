use std::{cell::RefCell, rc::Rc};

use crate::{CpcResult, core::{array::ArrayObj, func::FuncObj, record::RecordObj}};

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
            _ => "<object>".to_string(),
        }
    }
}

pub fn default_type_value(ty: &Type) -> CpcResult<RuntimeValue> {
    match ty {
        Type::Int => Ok(RuntimeValue::Int(0)),
        Type::Float => Ok(RuntimeValue::Float(0.0)),
        Type::Str => Ok(RuntimeValue::Str(String::new())),
        Type::Bool => Ok(RuntimeValue::Bool(false)),
        Type::Null => Ok(RuntimeValue::Null),
        Type::Array(ele_ty, start, end) => ArrayObj::new_runtime(*ele_ty.clone(), *start, *end),
        _ => Err(crate::CpcError::Runtime {
            span: None,
            kind: crate::RuntimeErrorKind::TypeWithoutDefaultValue(ty.clone()),
        }),
    }
}

pub fn get_value_type(value: &RuntimeValue) -> CpcResult<Type> {
    match value {
        RuntimeValue::Int(_) => Ok(Type::Int),
        RuntimeValue::Float(_) => Ok(Type::Float),
        RuntimeValue::Str(_) => Ok(Type::Str),
        RuntimeValue::Bool(_) => Ok(Type::Bool),
        RuntimeValue::Null => Ok(Type::Null),
        RuntimeValue::Array(arr_obj) => {
            let arr = arr_obj.borrow();
            let start = arr.get(arr.start())?;
            let ele_type = get_value_type(&start)?;
            Ok(Type::Array(Box::new(ele_type), arr.start(), arr.end()))
        },
        _ => Err(crate::CpcError::Runtime {
            span: None,
            kind: crate::RuntimeErrorKind::UnknownType(value.clone()),
        }),
    }
}
