use crate::ast::{FuncValueType, LiteralType};
use std::fmt;

impl LiteralType {
    pub fn type_name(&self) -> String {
        match self {
            Self::Number(_) => "number".to_string(),
            Self::String(_) => "string".to_string(),
            Self::Char(_) => "char".to_string(),
            Self::Boolean(_) => "bool".to_string(),
            Self::Array(_) => "array".to_string(),
            Self::Func(_) => "function".to_string(),
            Self::Void => "void".to_string(),
            Self::DeclrFunc(_) => "declared function".to_string(),
            Self::Null => "null".to_string(),
            Self::Any => "any".to_string(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Number(val) => *val != 0.0,
            Self::String(val) => !val.is_empty(),
            Self::Char(val) => *val != '\0',
            Self::Boolean(val) => *val,
            Self::Null => false,
            Self::Array(val) => !val.is_empty(),
            _ => false,
        }
    }
    pub fn is_truthy_literal(&self) -> LiteralType {
        Self::Boolean(self.is_truthy())
    }
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
            Self::Char(val) => write!(f, "{}", val),
            Self::Boolean(val) => write!(f, "{}", val),
            Self::Null => write!(f, "null"),
            Self::Array(val) => {
                let mut s = String::new();
                s.push('[');
                for (i, v) in val.iter().enumerate() {
                    s.push_str(&v.to_string());
                    if i != val.len() - 1 {
                        s.push_str(", ");
                    }
                }
                s.push(']');
                write!(f, "{}", s)
            }
            Self::Any => write!(f, "any"),
            Self::Void => write!(f, "void"),
            Self::Func(func) => match func {
                FuncValueType::Func(i) => write!(f, "{}()", i.name),
                _ => write!(f, "{:?}", func),
            },
            Self::DeclrFunc(declr_func) => write!(f, "{}()", declr_func.name),
        }
    }
}

