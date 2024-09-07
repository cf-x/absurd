use super::{Base, LiteralKind, Token, TokenType};
use crate::ast::LiteralType;
use std::fmt;

impl LiteralType {
    pub fn type_name(&self) -> String {
        match self {
            Self::Obj(_) => "object".to_string(),
            Self::Number(_) => "number".to_string(),
            Self::String(_) => "string".to_string(),
            Self::Char(_) => "char".to_string(),
            Self::Boolean(_) => "bool".to_string(),
            Self::Vec(_) => "vector".to_string(),
            Self::Func(_) => "function".to_string(),
            Self::Void => "void".to_string(),
            Self::DeclrFunc(_) => "declared function".to_string(),
            Self::Null => "null".to_string(),
        }
    }

    pub fn to_token(&self) -> Token {
        match self {
            Self::Obj(_) => Token {
                token: TokenType::AnyIdent,
                lexeme: "any".to_string(),
                value: None,
                line: 0,
                pos: (0, 0),
            },
            Self::Number(n) => Token {
                token: TokenType::NumLit,
                lexeme: n.to_string(),
                value: Some(LiteralKind::Number {
                    base: Base::Decimal,
                    value: *n,
                }),
                line: 0,
                pos: (0, 0),
            },
            Self::String(s) => Token {
                token: TokenType::StrLit,
                lexeme: s.to_string(),
                value: Some(LiteralKind::String { value: s.clone() }),
                line: 0,
                pos: (0, 0),
            },
            Self::Char(c) => Token {
                token: TokenType::CharLit,
                lexeme: c.to_string(),
                value: Some(LiteralKind::Char { value: *c }),
                line: 0,
                pos: (0, 0),
            },
            Self::Boolean(b) => Token {
                token: TokenType::BoolIdent,
                lexeme: b.to_string(),
                value: Some(LiteralKind::Bool { value: *b }),
                line: 0,
                pos: (0, 0),
            },
            Self::Vec(_) => Token {
                token: TokenType::AnyIdent,
                lexeme: "any".to_string(),
                value: None,
                line: 0,
                pos: (0, 0),
            },
            Self::Func(_) => Token {
                token: TokenType::AnyIdent,
                lexeme: "any".to_string(),
                value: None,
                line: 0,
                pos: (0, 0),
            },
            Self::Void => Token {
                token: TokenType::VoidIdent,
                lexeme: "void".to_string(),
                value: None,
                line: 0,
                pos: (0, 0),
            },
            Self::DeclrFunc(_) => Token {
                token: TokenType::AnyIdent,
                lexeme: "any".to_string(),
                value: None,
                line: 0,
                pos: (0, 0),
            },
            Self::Null => Token {
                token: TokenType::Null,
                lexeme: "null".to_string(),
                value: Some(LiteralKind::Null),
                line: 0,
                pos: (0, 0),
            },
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Number(val) => *val != 0.0,
            Self::String(val) => !val.is_empty(),
            Self::Char(val) => *val != '\0',
            Self::Boolean(val) => *val,
            Self::Null => false,
            Self::Vec(val) => !val.is_empty(),
            Self::Obj(obj) => !obj.is_empty(),
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
            Self::Obj(val) => {
                let n: Vec<String> = val
                    .iter()
                    .map(|(name, value)| format!("{}:{}", name, value.to_string()))
                    .collect();
                let c = n.join(", ");
                write!(f, "{{ {} }}", c)
            }
            Self::Number(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
            Self::Char(val) => write!(f, "{}", val),
            Self::Boolean(val) => write!(f, "{}", val),
            Self::Null => write!(f, "null"),
            Self::Vec(val) => {
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
            Self::Void => write!(f, "void"),
            Self::Func(func) => write!(f, "{:?}", func.name),
            Self::DeclrFunc(declr_func) => write!(f, "{}()", declr_func.name),
        }
    }
}
