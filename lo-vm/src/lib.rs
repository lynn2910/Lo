use lo_core::module::Module;
use crate::ctx::Context;

pub mod ctx;
mod eval;
mod system_calls;

pub fn execute_module<'a>(module: Module, global_context: &'a mut Context<'a>){
    eval::eval_module(&module, global_context)
}