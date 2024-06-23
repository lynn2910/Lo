use crate::values::Value;

#[derive(Default, Debug)]
pub enum AstNode {
    #[default]
    Null,
    Value(Value),
    VariableDeclaration(String, Box<AstNode>),
    BinaryOp(Box<AstNode>, BinOp, Box<AstNode>),
    /// FunctionCall(function_name, ...args)
    FunctionCall(String, Vec<Box<AstNode>>),
    /// VariableCall(variable_name)
    VariableCall(String)
}

#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo
}