use std::fmt;

use crate::ast::{LiteralKind, LiteralType, Token, TokenType};

impl LiteralType {
    pub fn token_to_literal(token: Token) -> LiteralType {
        match token.token {
            TokenType::NumberLit => {
                let val = match token.value {
                    Some(LiteralKind::Number { base: _, value }) => value,
                    _ => {
                        panic!("failed to parse number literal");
                    }
                };
                Self::Number(val)
            }
            TokenType::StringLit => {
                let val = match token.value {
                    Some(LiteralKind::String { value }) => value,
                    _ => {
                        panic!("failed to parse string literal");
                    }
                };
                Self::String(val)
            }
            TokenType::CharLit => {
                let val = match token.value {
                    Some(LiteralKind::Char { value }) => value,
                    _ => {
                        panic!("failed to parse char literal");
                    }
                };
                Self::Char(val)
            }
            TokenType::TrueLit | TokenType::FalseLit => {
                let val = match token.value {
                    Some(LiteralKind::Bool { value }) => value,
                    _ => {
                        panic!("failed to parse bool literal");
                    }
                };
                Self::Boolean(val)
            }
            TokenType::NullLit => Self::Null,
            // @todo array literal
            TokenType::ArrayLit => Self::Array(vec![]),
            _ => panic!("invalid token"),
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

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
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
            _ => write!(f, "unknown"),
        }
    }
}
