use std::collections::LinkedList;
use std::ops::Add;
use lo_core::ast::node::{AstNode, BinOp};
use lo_core::types::Type;
use lo_core::values::Value;
use crate::ctx::Context;
use crate::system_calls::sys_println;

pub fn eval_module(list: LinkedList<AstNode>, ctx: &mut Context) {
    let list_iter = list.into_iter();
    for node in list_iter {
        match node {
            AstNode::VariableDeclaration(var_name, var_node) => {
                if ctx.variables.contains_key(&var_name) { panic!("Variable '{var_name}' is already declared") }

                let computed_value = eval_node(ctx, &var_node).into();
                println!("Variable declared: {var_name} = {computed_value:?}");
                ctx.variables.insert(var_name, computed_value);

            },
            AstNode::FunctionCall(fn_name, args) => {
                if ctx.is_function_system(&fn_name) {
                    println!("System function call: {fn_name}");
                    system_function_called(
                        fn_name.as_str(),
                        args.iter()
                            .map(|a| eval_node(ctx, a).into())
                            .collect::<Vec<Value>>()
                    );
                } else {
                    unimplemented!("No function system implemented")
                }
            },
            _ => unimplemented!("AstNode unimplemented")
        }
    }
}


fn eval_node(ctx: &mut Context, node: &AstNode) -> Option<Value> {
    match node {
        AstNode::Null => None,
        // TODO find a better way than a clone...
        AstNode::Value(v) => Some(v.clone()),
        AstNode::VariableCall(v_name) => {
            if !ctx.variables.contains_key(v_name) {
                panic!("The variable {v_name} doesn't exist")
            }

            // TODO also the clone problem...
            ctx.variables.get(v_name).cloned()
        }
        AstNode::BinaryOp(left, op, right) => {
            let left_v:  Value = eval_node(ctx, left).into();
            let left_type = left_v.get_type();

            let right_v: Value = eval_node(ctx, right).into();
            let right_type = right_v.get_type();

            match op {
                BinOp::Plus => {
                    if left_type == Type::Int && left_type == right_type {
                        Some(Value::Int(left_v.get_int().unwrap() + right_v.get_int().unwrap()))
                    } else if left_type == Type::Float && left_type == right_type {
                        Some(Value::Float(left_v.get_float().unwrap() + right_v.get_float().unwrap()))
                    } else {
                        Some(Value::String(left_v.to_string().add(right_v.to_string().as_str())))
                    }
                },
                BinOp::Minus => {
                    if left_type == Type::Int && left_type == right_type {
                        Some(Value::Int(left_v.get_int().unwrap() - right_v.get_int().unwrap()))
                    } else if left_type == Type::Float && left_type == right_type {
                        Some(Value::Float(left_v.get_float().unwrap() - right_v.get_float().unwrap()))
                    } else {
                        panic!("Logic error: Cannot subtract two values if they're not int or float")
                    }
                },
                BinOp::Divide => {
                    if left_type == Type::Int && left_type == right_type {
                        Some(Value::Int(left_v.get_int().unwrap() / right_v.get_int().unwrap()))
                    } else if left_type == Type::Float && left_type == right_type {
                        Some(Value::Float(left_v.get_float().unwrap() / right_v.get_float().unwrap()))
                    } else {
                        panic!("Logic error: Cannot divide two values if they're not int or float")
                    }
                },
                BinOp::Modulo => {
                    if left_type == Type::Int && left_type == right_type {
                        Some(Value::Int(left_v.get_int().unwrap() % right_v.get_int().unwrap()))
                    } else if left_type == Type::Float && left_type == right_type {
                        Some(Value::Float(left_v.get_float().unwrap() % right_v.get_float().unwrap()))
                    } else {
                        panic!("Logic error: Cannot performa a modulation between two values if they're not int or float")
                    }
                },
                BinOp::Multiply => {
                    if left_type == Type::Int && left_type == right_type {
                        Some(Value::Int(left_v.get_int().unwrap() * right_v.get_int().unwrap()))
                    } else if left_type == Type::Float && left_type == right_type {
                        Some(Value::Float(left_v.get_float().unwrap() * right_v.get_float().unwrap()))
                    } else {
                        panic!("Logic error: Cannot multiply two values if they're not int or float")
                    }
                }
            }
        }
        _ => unimplemented!("Unimplemented node evaluation")
    }
}

fn system_function_called(name: &str, args: Vec<Value>){
    match name {
        "println" => sys_println(args),
        _ => panic!("Unknown system function: {name}")
    }
}