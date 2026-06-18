use crate::RuntimeValue;

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayObj {
    data: Vec<RuntimeValue>,
    start: usize,
    end: usize,
}
