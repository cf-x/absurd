use crate::ast::{LiteralKind, Token};

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

