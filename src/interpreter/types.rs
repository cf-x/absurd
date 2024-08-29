use std::{cell::RefCell, rc::Rc};

use crate::ast::{
    literals::token_to_literal, FuncImpl, FuncValueType, LiteralKind, LiteralType, Token, TokenType,
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
            if let Some(LiteralKind::Type(x)) = value_type.clone().value {
                if let TypeKind::Func {
                    params: pas,
                    ret: _,
                } = *x
                {
                    if let LiteralType::Func(FuncValueType::Func(FuncImpl {
                        params,
                        value_type: _,
                        ..
                    })) = val.clone()
                    {
                        if params.len() == pas.len() {
                            let _ts = params
                                .clone()
                                .iter()
                                .map(|x| token_to_literal(x.clone().1))
                                .collect::<Vec<LiteralType>>();
                            let _ps = pas
                                .iter()
                                .map(|x| typekind_to_literaltype(x.clone()))
                                .collect::<Vec<LiteralType>>();

                            return true;
                        }
                    }
                }
            }
            false
        }
        TokenType::Ident => {
            let d = env.borrow_mut().get_type(&value_type.lexeme.as_str());
            type_check(&d, val, env)
        }
        TokenType::Type => {
            let t = value_type.clone().value.unwrap_or(LiteralKind::Null);

            match t.clone() {
                LiteralKind::Type(t) => {
                    let t = *t;
                    if let TypeKind::Or { left, right } = t.clone() {
                        let left_t = typekind_to_literaltype(*left.clone());
                        let right_t = typekind_to_literaltype(*right.clone());
                        if left_t == *val || right_t == *val {
                            return true;
                        } else {
                            let left_n = match *left {
                                TypeKind::Var { name } => type_check(&name, val, env),
                                _ => left_t == *val,
                            };

                            let right_n = match *right {
                                TypeKind::Var { name } => type_check(&name, val, env),
                                _ => right_t == *val,
                            };
                            return left_n || right_n;
                        }
                    }
                }
                _ => return false,
            }
            false
        }
        TokenType::NumberIdent => matches!(val, LiteralType::Number(_)),
        TokenType::StringIdent => matches!(val, LiteralType::String(_)),
        TokenType::BoolIdent => matches!(val, LiteralType::Boolean(_)),
        TokenType::CharIdent => matches!(val, LiteralType::Char(_)),
        TokenType::NullIdent => matches!(val, LiteralType::Null),
        TokenType::VoidIdent => matches!(val, LiteralType::Void),
        TokenType::ArrayIdent => {
            if let LiteralType::Array(array) = val {
                match value_type.clone().value {
                    Some(value) => match value {
                        LiteralKind::Type(t) => {
                            let t = *t;
                            match t {
                                TypeKind::Array { statics, .. } => {
                                    if let Some(statics) = statics {
                                        if statics.len() != array.len() {
                                            return false;
                                        }

                                        return statics.iter().zip(array.iter()).all(
                                            |(stat, item)| {
                                                let stat_token = match stat.clone() {
                                                    TypeKind::Var { name } => name,
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
                                            },
                                        );
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
                                _ => {}
                            }
                        }
                        _ => {}
                    },
                    None => {}
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
            match val.clone() {
                LiteralType::Number(num) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap_or(LiteralKind::Null)), LiteralType::Number(n) if n == num);
                }
                LiteralType::String(s) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap_or(LiteralKind::Null)), LiteralType::String(n) if n == s);
                }
                LiteralType::Boolean(b) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap_or(LiteralKind::Null)), LiteralType::Boolean(n) if n == b);
                }
                LiteralType::Char(c) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap_or(LiteralKind::Null)), LiteralType::Char(n) if n == c);
                }
                LiteralType::Array(v) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap_or(LiteralKind::Null)), LiteralType::Array(n) if n == v);
                }
                LiteralType::Null => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(
                            literalkind_to_literaltype(
                                value_type.clone().value.unwrap_or(LiteralKind::Null)
                            ),
                            LiteralType::Null
                        );
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
