use std::collections::LinkedList;
use std::time::Instant;
use lo_core::ast::node::{AstNode, BinOp};
use lo_core::values::Value;

fn main() {
    let mut linked_list = LinkedList::<AstNode>::new();

    // create value
    linked_list.push_back(
        AstNode::VariableDeclaration("v".to_string(),
        Box::new(AstNode::BinaryOp(
            Box::new(AstNode::Value(Value::Int(5))),
            BinOp::Plus,
            Box::new(AstNode::Value(Value::Int(5))),
        )))
    );

    linked_list.push_back(
        AstNode::FunctionCall(
            "println".to_string(),
            vec![
                Box::new(AstNode::VariableCall("v".to_string()))
            ]
        ),
    );
    
    let n = Instant::now();
    lo_vm::execute(linked_list);
    print!("Result performed in {}s", n.elapsed().as_secs_f64());
}
