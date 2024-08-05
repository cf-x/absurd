use ape_ast::{CallType, FuncBody, LiteralType, Token};
use std::hash::Hasher;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Array {
        id: usize,
        items: Vec<LiteralType>,
    },
    Var {
        id: usize,
        name: Token,
    },
    Call {
        id: usize,
        name: Box<Token>,
        args: Vec<Expression>,
        call_type: CallType,
    },
    Unary {
        id: usize,
        left: Box<Expression>,
        operator: Token,
    },
    Binary {
        id: usize,
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        id: usize,
        expression: Box<Expression>,
    },
    Value {
        id: usize,
        value: LiteralType,
    },
    Func {
        id: usize,
        name: Token,
        value_type: Token,
        body: FuncBody,
        params: Vec<(Token, Token)>,
        is_async: bool,
        is_pub: bool,
    },
    Await {
        id: usize,
        expr: Box<Expression>,
    },
}

impl Hash for Expression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state)
    }
}

impl Eq for Expression {}
impl Expression {
    pub fn id(&self) -> usize {
        match self {
            Expression::Var { id, .. } => *id,
            Expression::Call { id, .. } => *id,
            Expression::Func { id, .. } => *id,
            Expression::Array { id, .. } => *id,
            Expression::Await { id, .. } => *id,
            Expression::Binary { id, .. } => *id,
            Expression::Unary { id, .. } => *id,
            Expression::Value { id, .. } => *id,
            Expression::Grouping { id, .. } => *id,
        }
    }
}
