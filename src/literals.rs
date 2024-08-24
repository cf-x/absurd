use crate::ast::{FuncValueType, LiteralKind, LiteralType, Token, TokenType};
use crate::errors::{Error, ErrorCode::*};
use crate::manifest::Project;
use std::fmt;
use std::process::exit;

impl LiteralType {
    fn err(&self) -> Error {
        Error::new("", Project::new())
    }

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

    #[allow(dead_code)]
    pub fn token_to_literal(&self, token: Token) -> LiteralType {
        match token.token {
            TokenType::NumberLit => {
                let val = match token.value {
                    Some(LiteralKind::Number { base: _, value }) => value,
                    _ => {
                        self.err()
                            .throw(E0x408, token.line, token.pos, vec!["number".to_string()]);
                        exit(0);
                    }
                };
                Self::Number(val)
            }
            TokenType::StringLit => {
                let val = match token.value {
                    Some(LiteralKind::String { value }) => value,
                    _ => {
                        self.err()
                            .throw(E0x408, token.line, token.pos, vec!["string".to_string()]);
                        exit(0);
                    }
                };
                Self::String(val)
            }
            TokenType::CharLit => {
                let val = match token.value {
                    Some(LiteralKind::Char { value }) => value,
                    _ => {
                        self.err()
                            .throw(E0x408, token.line, token.pos, vec!["char".to_string()]);
                        exit(0);
                    }
                };
                Self::Char(val)
            }
            TokenType::TrueLit | TokenType::FalseLit => {
                let val = match token.value {
                    Some(LiteralKind::Bool { value }) => value,
                    _ => {
                        self.err().throw(
                            E0x408,
                            token.line,
                            token.pos,
                            vec!["boolean".to_string()],
                        );
                        exit(0);
                    }
                };
                Self::Boolean(val)
            }
            TokenType::NullLit => Self::Null,
            // @todo array literal
            TokenType::ArrayLit => Self::Array(vec![]),
            _ => {
                self.err().throw(E0x407, token.line, token.pos, vec![]);
                exit(0);
            }
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
