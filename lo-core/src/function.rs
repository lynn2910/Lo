use std::collections::LinkedList;
use crate::ast::node::AstNode;

#[derive(Debug)]
pub struct Function {
    name: String,
    is_system: bool,
    system_id: Option<String>,
    internal_ast: Option<LinkedList<AstNode>>
}

impl Function {
    pub fn new_system(name: impl ToString, id: impl ToString) -> Self {
        Function {
            name: name.to_string(),
            is_system: true,
            system_id: Some(id.to_string()),
            internal_ast: None
        }
    }

    pub fn is_system(&self) -> bool {
        self.is_system
    }
}