use std::fmt;
use std::collections::HashMap;
use crate::bytecode::FuncProto;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Range(i64, i64),
    Func(FuncProto),
    Builtin(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(n)      => write!(f, "{}", n),
            Value::Float(n)    => write!(f, "{}", n),
            Value::Str(s)      => write!(f, "{}", s),
            Value::Bool(b)     => write!(f, "{}", b),
            Value::Nil         => write!(f, "nil"),
            Value::Func(fp)    => write!(f, "<fn {}>", fp.name),
            Value::Builtin(n)  => write!(f, "<builtin {}>", n),
            Value::Range(a, b) => write!(f, "{}..{}", a, b),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Map(map) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in map {
                    if !first { write!(f, ", ")?; }
                    write!(f, "{}: {}", k, v)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(false) => false,
            Value::Nil         => false,
            Value::Int(0)      => false,
            _                  => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_)     => "int",
            Value::Float(_)   => "float",
            Value::Str(_)     => "str",
            Value::Bool(_)    => "bool",
            Value::Nil        => "nil",
            Value::List(_)    => "list",
            Value::Map(_)     => "map",
            Value::Range(..)  => "range",
            Value::Func(_)    => "fn",
            Value::Builtin(_) => "builtin",
        }
    }
}