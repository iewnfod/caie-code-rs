use crate::{ScopeRef, Stmt, Type};

#[derive(Clone, Debug, PartialEq)]
pub struct FuncObj {
    params: Vec<(String, Type)>,
    body: Stmt,
    closure: ScopeRef,
}
