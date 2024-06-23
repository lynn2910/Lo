use std::collections::{HashMap, LinkedList};
use std::time::Instant;
use lo_core::ast::node::{AstNode, BinOp};
use lo_core::module::Module;
use lo_core::values::Value;
use lo_vm::ctx::Context;

fn main() {
    let mut unscoped_instructions = LinkedList::<AstNode>::new();

    // create value
    unscoped_instructions.push_back(
        AstNode::VariableDeclaration("v".to_string(),
        Box::new(AstNode::BinaryOp(
            Box::new(AstNode::Value(Value::Int(5))),
            BinOp::Plus,
            Box::new(AstNode::BinaryOp(
                Box::new(AstNode::Value(Value::Int(8))),
                BinOp::Minus,
                Box::new(AstNode::Value(Value::Int(3)))
            )),
        )))
    );

    unscoped_instructions.push_back(
        AstNode::FunctionCall(
            "println".to_string(),
            vec![
                Box::new(AstNode::VariableCall("v".to_string()))
            ]
        ),
    );

    let module = Module::new_empty("__undefined__", unscoped_instructions);

    let mut global_context = Context::default();
    global_context.define_as_global();

    let n = Instant::now();
    lo_vm::execute_module(module, &mut global_context);
    print!("Result performed in {}s", n.elapsed().as_secs_f64());
}
