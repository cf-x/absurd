use super::{Base, LiteralKind, Token, TokenType::*};
use crate::ast::LiteralType;
use std::fmt;

impl LiteralType {
    pub fn type_name(&self) -> String {
        match self {
            Self::Enum { .. } => "enum".to_string(),
            Self::Tuple(_) => "tuple".to_string(),
            Self::Record(_) => "record".to_string(),
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
            Self::Number(n) => Token::empty(
                NumLit,
                n.to_string().as_str(),
                Some(LiteralKind::Number {
                    base: Base::Decimal,
                    value: *n,
                }),
            ),
            Self::String(s) => {
                Token::empty(StrLit, s, Some(LiteralKind::String { value: s.clone() }))
            }
            Self::Char(c) => Token::empty(
                StrLit,
                c.to_string().as_str(),
                Some(LiteralKind::Char { value: c.clone() }),
            ),
            Self::Boolean(b) => Token::empty(
                BoolIdent,
                b.to_string().as_str(),
                Some(LiteralKind::Bool { value: *b }),
            ),
            Self::Void => Token::empty(VoidIdent, "void", None),
            Self::Null => Token::null(),
            _ => Token::empty(AnyIdent, "any", None),
        }
    }

    #[allow(dead_code)]
    pub fn to_type_token(&self) -> Token {
        match self {
            Self::Number(_) => Token::empty(NumIdent, "number", None),
            Self::String(_) => Token::empty(StrIdent, "string", None),
            Self::Char(_) => Token::empty(CharIdent, "char", None),
            Self::Boolean(_) => Token::empty(BoolIdent, "bool", None),
            Self::Void => Token::empty(VoidIdent, "void", None),
            Self::Null => Token::null(),
            _ => Token::empty(AnyIdent, "any", None),
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
            Self::Tuple(val) => !val.is_empty(),
            Self::Record(rec) => !rec.is_empty(),
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
            Self::Enum { name, .. } => write!(f, "{}", name.lexeme),
            Self::Tuple(val) => {
                let mut s = String::new();
                for (i, v) in val.iter().enumerate() {
                    s.push_str(&v.to_string());
                    if i != val.len() - 1 {
                        s.push_str(", ");
                    }
                }
                write!(f, "({})", s)
            }
            Self::Record(val) => {
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
                for (i, v) in val.iter().enumerate() {
                    s.push_str(&v.to_string());
                    if i != val.len() - 1 {
                        s.push_str(", ");
                    }
                }
                write!(f, "[{}]", s)
            }
            Self::Void => write!(f, "void"),
            Self::Func(func) => write!(f, "{:?}", func.name),
            Self::DeclrFunc(declr_func) => write!(f, "{}()", declr_func.name),
        }
    }
}
