use std::{cell::RefCell, rc::Rc};

use crate::{CpcResult, RuntimeValue, Type, default_type_value};

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayObj {
    data: Vec<RuntimeValue>,
    start: usize,
    end: usize,
}

impl ArrayObj {
    pub fn new(ele_type: Type, start: usize, end: usize) -> CpcResult<Self> {
        let size = end.saturating_sub(start).saturating_add(1);
        let mut data = vec![];
        for _ in 0..size {
            data.push(default_type_value(&ele_type)?);
        }
        Ok(ArrayObj { data, start, end })
    }

    pub fn new_runtime(ele_type: Type, start: usize, end: usize) -> CpcResult<RuntimeValue> {
        let arr = Self::new(ele_type, start, end)?;
        Ok(RuntimeValue::Array(Rc::new(RefCell::new(arr))))
    }

    pub fn get(&self, index: usize) -> CpcResult<RuntimeValue> {
        if index < self.start || index > self.end {
            Err(crate::CpcError::Runtime {
                span: None,
                kind: crate::RuntimeErrorKind::IndexOutOfBounds {
                    index: index as i64,
                    range: (self.start, self.end),
                },
            })
        } else {
            Ok(self.data[index - self.start].clone())
        }
    }

    pub fn set(&mut self, index: usize, value: RuntimeValue) -> CpcResult<()> {
        if index < self.start || index > self.end {
            Err(crate::CpcError::Runtime {
                span: None,
                kind: crate::RuntimeErrorKind::IndexOutOfBounds {
                    index: index as i64,
                    range: (self.start, self.end),
                },
            })
        } else {
            self.data[index - self.start] = value;
            Ok(())
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
