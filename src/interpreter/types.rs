use super::{env::Env, expr::Expression};
use crate::ast::{LiteralKind, LiteralType, Token, TokenType};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};


#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    Vec {
        // <type>
        kind: Box<TypeKind>,
    },
    Object {
        // {name: type, name: type, ..}
        fields: Vec<(Token, TypeKind)>,
    },
    Var {
        // identifier for calling types
        name: Token,
    },
    Either {
        // T || T
        lhs: Box<TypeKind>,
        rhs: Box<TypeKind>,
    },
    Maybe {
        // T?
        lhs: Box<TypeKind>,
    },
    Important {
        // T!
        lhs: Box<TypeKind>,
    },
    Callback {
        // |type, type, ..| type
        params: Vec<TypeKind>,
        ret: Box<TypeKind>,
    },
    Literal {
        // "string" 5.21 false
        kind: LiteralKind,
    },
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeKind::Vec { kind } => {
                write!(f, "<{}>", kind)
            }
            TypeKind::Object { fields } => {
                write!(f, "{{")?;
                for (i, (name, t)) in fields.iter().enumerate() {
                    write!(f, "{}: {}", name.lexeme, t)?;
                    if i != fields.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            TypeKind::Var { name } => write!(f, "{}", name.lexeme),
            TypeKind::Either { lhs, rhs } => write!(f, "{} || {}", lhs, rhs),
            TypeKind::Maybe { lhs } => write!(f, "{}?", lhs),
            TypeKind::Important { lhs } => write!(f, "{}!", lhs),
            TypeKind::Literal { kind } => write!(f, "{:?}", kind),
            TypeKind::Callback { params, ret } => {
                write!(f, "|")?;
                for (i, p) in params.iter().enumerate() {
                    write!(f, "{}", p)?;
                    if i != params.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "| {}", ret)
            }
        }
    }
}

pub fn type_check(value_type: &Token, val: &LiteralType, env: &Rc<RefCell<Env>>) -> bool {
    match value_type.token {
        TokenType::FuncIdent => true,
        TokenType::Ident => {
            let d = env.borrow().get_type(&value_type.lexeme);
            type_check(&d, val, env)
        }
        TokenType::Type => {
            if let Some(LiteralKind::Type(ref t)) = value_type.value {
                let lt = *t.clone();
                match lt {
                    TypeKind::Either { ref lhs, ref rhs } => {
                        let lhs_t = typekind_to_literaltype(*lhs.clone());
                        let rhs_t = typekind_to_literaltype(*rhs.clone());
                        if lhs_t == *val || rhs_t == *val {
                            return true;
                        } else {
                            let lhs_n = match **lhs {
                                TypeKind::Var { ref name } => type_check(name, val, env),
                                _ => lhs_t == *val,
                            };
                            let rhs_n = match **rhs {
                                TypeKind::Var { ref name } => type_check(name, val, env),
                                _ => rhs_t == *val,
                            };
                            return lhs_n || rhs_n;
                        }
                    }
                    TypeKind::Maybe { ref lhs } => {
                        let lhs_t = typekind_to_literaltype(*lhs.clone());
                        if lhs_t == *val || LiteralType::Null == *val {
                            return true;
                        } else {
                            let lhs_n = match **lhs {
                                TypeKind::Var { ref name } => type_check(name, val, env),
                                _ => lhs_t == *val,
                            };
                            return lhs_n;
                        }
                    }
                    TypeKind::Object { fields } => {
                        if let LiteralType::Obj(ref obj) = *val {
                            let obj_map: HashMap<_, _> = obj.iter().cloned().collect();
                            return fields.iter().all(|(name, field_type)| {
                                if let Some(v) = obj_map.get(&name.lexeme) {
                                    let field_token = Token {
                                        token: string_to_tokentype(&field_type.to_string()),
                                        lexeme: field_type.to_string(),
                                        value: Some(LiteralKind::Type(Box::new(
                                            field_type.clone(),
                                        ))),
                                        line: name.line,
                                        pos: name.pos,
                                    };
                                    return type_check(&field_token, &v.eval(env.clone()), env);
                                } else {
                                    false
                                }
                            });
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
        TokenType::NumIdent => {
            if val.type_name() == "function" {
                return true;
            }
            matches!(val, LiteralType::Number(_))
        }
        TokenType::StrIdent => matches!(val, LiteralType::String(_)),
        TokenType::BoolIdent => matches!(val, LiteralType::Boolean(_)),
        TokenType::CharIdent => matches!(val, LiteralType::Char(_)),
        TokenType::Null => matches!(val, LiteralType::Null),
        TokenType::VoidIdent => matches!(val, LiteralType::Void),
        TokenType::ArrayIdent => {
            if let LiteralType::Vec(ref array) = *val {
                if let Some(LiteralKind::Type(ref t)) = value_type.value {
                    if let TypeKind::Vec { .. } = **t {
                        return array.iter().all(|item| {
                            type_check(
                                &Token {
                                    token: string_to_tokentype(&value_type.lexeme),
                                    lexeme: value_type.lexeme.clone(),
                                    value: None,
                                    line: value_type.line,
                                    pos: value_type.pos,
                                },
                                &item,
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
        TokenType::NumLit
        | TokenType::StrLit
        | TokenType::TrueLit
        | TokenType::FalseLit
        | TokenType::CharLit
       => {
            match *val {
                LiteralType::Number(ref n) => return check_num(n, value_type),
                LiteralType::String(ref s) => return check_str(s, value_type),
                LiteralType::Boolean(ref b) => return check_bool(b, value_type),
                LiteralType::Char(ref c) => return check_char(c, value_type),
                LiteralType::Null => return check_null(value_type),
                _ => {}
            }
            false
        }
        _ => false,
    }
}

fn check_num(n: &f32, value_type: &Token) -> bool {
    matches!(value_type.token, TokenType::NumLit)
        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::Number(ref m) if m == n)
}

fn check_str(s: &String, value_type: &Token) -> bool {
    matches!(value_type.token, TokenType::StrLit)
        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::String(ref n) if n == s)
}

fn check_bool(b: &bool, value_type: &Token) -> bool {
    matches!(value_type.token, TokenType::TrueLit | TokenType::FalseLit)
        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::Boolean(ref n) if n == b)
}

fn check_char(c: &char, value_type: &Token) -> bool {
    matches!(value_type.token, TokenType::CharLit)
        && matches!(literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)), LiteralType::Char(ref n) if n == c)
}

fn check_null(value_type: &Token) -> bool {
    matches!(value_type.token, TokenType::Null)
        && matches!(
            literalkind_to_literaltype(value_type.value.clone().unwrap_or(LiteralKind::Null)),
            LiteralType::Null
        )
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
        TypeKind::Object { fields } => obj_to_lt(fields),
        TypeKind::Var { name } => var_to_lt(name),
        TypeKind::Callback { ret, .. } => typekind_to_literaltype(*ret),
        TypeKind::Vec { kind } => typekind_to_literaltype(*kind),
        TypeKind::Literal { kind } => literalkind_to_literaltype(kind),
        TypeKind::Either { lhs, .. } => typekind_to_literaltype(*lhs),
        TypeKind::Maybe { lhs } => typekind_to_literaltype(*lhs),
        TypeKind::Important { lhs } => typekind_to_literaltype(*lhs),
    }
}

fn var_to_lt(name: Token) -> LiteralType {
    let n = match name.clone().value {
        Some(v) => v,
        None => LiteralKind::Null,
    };
    literalkind_to_literaltype(n)
}
fn obj_to_lt(fields: Vec<(Token, TypeKind)>) -> LiteralType {
    let mut obj = vec![];
    for (k, v) in fields {
        let v = typekind_to_literaltype(v);
        obj.push((k.lexeme, Expression::Value { id: 0, value: v }));
    }
    LiteralType::Obj(obj)
}

pub fn string_to_tokentype(s: &str) -> TokenType {
    match s {
        "number" => TokenType::NumIdent,
        "string" => TokenType::StrIdent,
        "boolean" => TokenType::BoolIdent,
        "char" => TokenType::CharIdent,
        "null" => TokenType::Null,
        "void" => TokenType::VoidIdent,
        _ => TokenType::AnyIdent,
    }
}
