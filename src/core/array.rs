use std::{cell::RefCell, rc::Rc};

use crate::{RuntimeValue, Type, default_type_value};

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayObj {
    data: Vec<RuntimeValue>,
    start: usize,
    end: usize,
}

impl ArrayObj {
    pub fn new(ele_type: Type, start: usize, end: usize) -> Self {
        let size = end - start + 1;
        let data = (0..size).map(|_| default_type_value(&ele_type)).collect();
        ArrayObj { data, start, end }
    }

    pub fn new_runtime(ele_type: Type, start: usize, end: usize) -> RuntimeValue {
        let arr = Self::new(ele_type, start, end);
        RuntimeValue::Array(Rc::new(RefCell::new(arr)))
    }

    pub fn get(&self, index: usize) -> Option<RuntimeValue> {
        if index < self.start || index > self.end {
            None
        } else {
            Some(self.data[index - self.start].clone())
        }
    }

    pub fn set(&mut self, index: usize, value: RuntimeValue) -> Option<()> {
        if index < self.start || index > self.end {
            None
        } else {
            self.data[index - self.start] = value;
            Some(())
        }
    }

    pub fn start(&self) -> usize { self.start }
    pub fn end(&self) -> usize { self.end }
    pub fn len(&self) -> usize { self.data.len() }

    pub fn to_string(&self) -> String {
        let elements: Vec<String> = self.data.iter().map(|v| v.to_string()).collect();
        format!("[{}]", elements.join(", "))
    }
}
