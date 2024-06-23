use std::collections::LinkedList;
use lo_core::ast::node;
use crate::ctx::Context;

pub mod ctx;
mod eval;
mod system_calls;

pub fn execute_with_context(list: LinkedList<node::AstNode>, mut global_context: Context){
    eval::eval_module(list, &mut global_context)
}

pub fn execute(list: LinkedList<node::AstNode>){
    let mut ctx = Context::default();
    ctx.define_as_global();

    execute_with_context(list, ctx)
}