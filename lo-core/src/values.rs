use std::fmt::{Display, Formatter};
use crate::types::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    None,
    Int(i64),
    Float(f64),
    String(String)
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Self::None      => Type::None,
            Self::Int(_)    => Type::Int,
            Self::Float(_)  => Type::Float,
            Self::String(_) => Type::String
        }
    }
    
    pub fn get_int(&self) -> Option<i64> {
        if let Self::Int(i) = self { Some(*i) }
        else { None }
    }
    
    pub fn get_float(&self) -> Option<f64> {
        if let Self::Float(f) = self { Some(*f) }
        else { None }
    }
}

impl From<Option<Value>> for Value {
    fn from(v: Option<Value>) -> Self {
        v.unwrap_or(Value::None)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::String(s) => write!(f, "{s}")
        }
    }
}