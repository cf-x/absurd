use std::{cell::RefCell, rc::Rc};

use crate::ast::{
    LiteralKind, LiteralType, Token, TokenType,
};

use super::env::Env;

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    Array {
        // <type>
        kind: Option<Box<TypeKind>>,
        // <(type, type, ..)>
        statics: Option<Vec<TypeKind>>,
    },
    Var {
        // identifier for calling types
        name: Token,
    },
    Or {
        left: Box<TypeKind>,
        right: Box<TypeKind>,
    },
    Value {
        // "string" 5.21 false
        kind: LiteralKind,
    },
    Func {
        // |type, type, ..| type
        params: Vec<TypeKind>,
        ret: Box<TypeKind>,
    },
}
pub fn type_check(value_type: &Token, val: &LiteralType, env: &Rc<RefCell<Env>>) -> bool {
    match value_type.token {
        TokenType::FuncIdent => {
            true
        }
        TokenType::Ident => {
            let d = env.borrow().get_type(&value_type.lexeme);
            type_check(&d, val, env)
        }
        TokenType::Type => {
            if let Some(LiteralKind::Type(ref t)) = value_type.value {
                if let TypeKind::Or { ref left, ref right } = **t {
                    let left_t = typekind_to_literaltype(*left.clone());
                    let right_t = typekind_to_literaltype(*right.clone());
                    if left_t == *val || right_t == *val {
                        return true;
                    } else {
                        let left_n = match **left {
                            TypeKind::Var { ref name } => type_check(name, val, env),
                            _ => left_t == *val,
                        };

                        let right_n = match **right {
                            TypeKind::Var { ref name } => type_check(name, val, env),
                            _ => right_t == *val,
                        };
                        return left_n || right_n;
                    }
                }
            }
            false
        }
        TokenType::NumberIdent => {
            if val.type_name() == "function" {
                return true;
            }
            matches!(val, LiteralType::Number(_))},
        TokenType::StringIdent => matches!(val, LiteralType::String(_)),
        TokenType::BoolIdent => matches!(val, LiteralType::Boolean(_)),
        TokenType::CharIdent => matches!(val, LiteralType::Char(_)),
        TokenType::NullIdent => matches!(val, LiteralType::Null),
        TokenType::VoidIdent => matches!(val, LiteralType::Void),
        TokenType::ArrayIdent => {
            if let LiteralType::Array(ref array) = *val {
                if let Some(LiteralKind::Type(ref t)) = value_type.value {
                    if let TypeKind::Array { ref statics, .. } = **t {
                        if let Some(ref statics) = *statics {
                            if statics.len() != array.len() {
                                return false;
                            }

                            return statics.iter().zip(array.iter()).all(|(stat, item)| {
                                let stat_token = match *stat {
                                    TypeKind::Var { ref name } => name.clone(),
                                    _ => value_type.clone(),
                                };
                                type_check(
                                    &Token {
                                        token: stat_token.token,
                                        lexeme: stat_token.lexeme.clone(),
                                        value: None,
                                        line: stat_token.line,
                                        pos: stat_token.pos,
                                    },
                                    &item.to_literal(),
                                    env,
                                )
                            });
                        }

                        return array.iter().all(|item| {
                            type_check(
                                &Token {
                                    token: string_to_token_type(&value_type.lexeme),
                                    lexeme: value_type.lexeme.clone(),
                                    value: None,
                                    line: value_type.line,
                                    pos: value_type.pos,
                                },
                                &item.to_literal(),
                                env,
                            )
                        });
                    }
                }
                false
            } else {
                false
            }
        }
        TokenType::AnyIdent => true,
        TokenType::NumberLit
        | TokenType::StringLit
        | TokenType::TrueLit
        | TokenType::FalseLit
        | TokenType::CharLit
        | TokenType::NullLit
        | TokenType::ArrayLit => {
            match *val {
                LiteralType::Number(ref num) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::Number(ref n) if n == num);
                }
                LiteralType::String(ref s) => {
                    return matches!(value_type.token, TokenType::StringLit)
                        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::String(ref n) if n == s);
                }
                LiteralType::Boolean(ref b) => {
                    return matches!(value_type.token, TokenType::TrueLit | TokenType::FalseLit)
                        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::Boolean(ref n) if n == b);
                }
                LiteralType::Char(ref c) => {
                    return matches!(value_type.token, TokenType::CharLit)
                        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::Char(ref n) if n == c);
                }
                LiteralType::Array(ref v) => {
                    return matches!(value_type.token, TokenType::ArrayLit)
                        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::Array(ref n) if n == v);
                }
                LiteralType::Null => {
                    return matches!(value_type.token, TokenType::NullLit)
                        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::Null);
                }
                _ => {}
            }
            false
        }
        _ => false,
    }
}

pub fn literalkind_to_literaltype(kind: LiteralKind) -> LiteralType {
    match kind {
        LiteralKind::Bool { value } => LiteralType::Boolean(value),
        LiteralKind::Null => LiteralType::Null,
        LiteralKind::Char { value } => LiteralType::Char(value),
        LiteralKind::Number { value, .. } => LiteralType::Number(value),
        LiteralKind::String { value } => LiteralType::String(value),
        LiteralKind::Type(t) => typekind_to_literaltype(*t),
    }
}

pub fn typekind_to_literaltype(kind: TypeKind) -> LiteralType {
    match kind.clone() {
        TypeKind::Var { name } => {
            let n = match name.clone().value {
                Some(v) => v,
                None => LiteralKind::Null,
            };
            literalkind_to_literaltype(n)
        }
        TypeKind::Func { ret, .. } => typekind_to_literaltype(*ret),
        TypeKind::Array { kind, statics } => {
            if kind.is_some() {
                typekind_to_literaltype(*kind.unwrap_or(Box::new(TypeKind::Value {
                    kind: LiteralKind::Null,
                })))
            } else {
                typekind_to_literaltype(
                    statics
                        .unwrap_or(vec![])
                        .get(0)
                        .unwrap_or(&TypeKind::Value {
                            kind: LiteralKind::Null,
                        })
                        .clone(),
                )
            }
        }
        TypeKind::Value { kind } => literalkind_to_literaltype(kind),
        TypeKind::Or { left, .. } => typekind_to_literaltype(*left),
    }
}

pub fn string_to_token_type(s: &str) -> TokenType {
    match s {
        "number" => TokenType::NumberIdent,
        "string" => TokenType::StringIdent,
        "boolean" => TokenType::BoolIdent,
        "char" => TokenType::CharIdent,
        "null" => TokenType::NullIdent,
        "void" => TokenType::VoidIdent,
        _ => TokenType::AnyIdent,
    }
}
