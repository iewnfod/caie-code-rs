use std::{cell::RefCell, rc::Rc};

use crate::{RuntimeValue, ScopeRef, Stmt, Type};

#[derive(Clone, Debug, PartialEq)]
pub struct FuncObj {
    name: String,
    params: Vec<(String, Type)>,
    body: Stmt,
    closure: ScopeRef,
    return_type: Option<Type>,
}

impl FuncObj {
    pub fn new(
        name: String,
        params: Vec<(String, Type)>,
        body: Stmt,
        closure: ScopeRef,
        return_type: Option<Type>
    ) -> Self {
        FuncObj { name, params, body, closure, return_type }
    }

    pub fn new_runtime(
        name: String,
        params: Vec<(String, Type)>,
        body: Stmt,
        closure: ScopeRef,
        return_type: Option<Type>
    ) -> RuntimeValue {
        let f = Self::new(name, params, body, closure, return_type);
        return RuntimeValue::Func(Rc::new(RefCell::new(f)));
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn params(&self) -> &Vec<(String, Type)> {
        &self.params
    }

    pub fn body(&self) -> &Stmt {
        &self.body
    }

    pub fn closure(&self) -> &ScopeRef {
        &self.closure
    }

    pub fn return_type(&self) -> Option<&Type> {
        self.return_type.as_ref()
    }
}
