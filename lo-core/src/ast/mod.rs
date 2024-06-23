use std::collections::LinkedList;
use crate::ast::node::AstNode;

pub mod node;

pub type InstructionsList = LinkedList<AstNode>;