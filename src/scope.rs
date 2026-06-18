use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{RuntimeValue, debug_print};

pub type ScopeRef = Rc<RefCell<Scope>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    vars: HashMap<String, RuntimeValue>,
    parent: Option<ScopeRef>,
}

impl Scope {
    pub fn root() -> ScopeRef {
        Rc::new(RefCell::new(Scope {
            vars: HashMap::new(),
            parent: None,
        }))
    }

    pub fn child(parent: ScopeRef) -> ScopeRef {
        Rc::new(RefCell::new(Scope {
            vars: HashMap::new(),
            parent: Some(parent),
        }))
    }
}

pub fn define(scope: &ScopeRef, name: String, value: RuntimeValue) {
    scope.borrow_mut().vars.insert(name, value);
}

pub fn get(scope: &ScopeRef, name: &str) -> Option<RuntimeValue> {
    let s = scope.borrow();
    if let Some(value) = s.vars.get(name) {
        Some(value.clone())
    } else {
        let parent = s.parent.clone();
        drop(s);
        match parent {
            Some(p) => get(&p, name),
            None => None,
        }
    }
}

pub fn set(scope: &ScopeRef, name: &str, value: RuntimeValue) -> bool {
    let mut s = scope.borrow_mut();
    if s.vars.contains_key(name) {
        s.vars.insert(name.to_string(), value);
        true
    } else {
        let parent = s.parent.clone();
        drop(s);
        match parent {
            Some(p) => set(&p, name, value),
            None => false,
        }
    }
}

pub fn print_scope(scope: &ScopeRef, debug: bool, indent: usize) {
    let s = scope.borrow();
    let indent_str = "  ".repeat(indent);
    if s.vars.is_empty() {
        debug_print(debug, format!("{}Vars: (empty)", indent_str));
    } else {
        debug_print(debug, format!("{}Vars:", indent_str));
    }
    for (k, v) in &s.vars {
        debug_print(debug, format!("{}  {}: {:?}", indent_str, k, v));
    }
    if let Some(parent) = &s.parent {
        debug_print(debug, format!("{}Parent Scope:", indent_str));
        print_scope(parent, debug, indent + 1);
    } else {
        debug_print(debug, format!("{}Parent Scope: (none)", indent_str));
    }
}
