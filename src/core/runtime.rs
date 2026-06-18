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
