use std::collections::HashMap;

use crate::RuntimeValue;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordObj {
    fields: HashMap<String, RuntimeValue>,
    type_name: String,
}
