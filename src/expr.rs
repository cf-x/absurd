use crate::{
    ast::{CallType, FuncBody, FuncValueType, LiteralType, Token},
    env::Env,
    interpreter::run_func,
};
use core::hash::{Hash, Hasher};

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
        name: Box<Expression>,
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
            // @todo add assign expression
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

    pub fn eval(&self, env: Env) -> LiteralType {
        match self {
            Expression::Var { name, .. } => match env.get(name.lexeme.clone(), self.id()) {
                Some(v) => v.clone(),
                None => LiteralType::Null,
            },
            Expression::Call {
                name,
                args,
                // call_type,
                ..
            } => {
                let call: LiteralType = name.eval(env.clone());

                match call {
                    LiteralType::Func(func) => match func {
                        FuncValueType::Func(func) => run_func(func, args, env),
                        _ => {
                            // @error invalid function call
                            LiteralType::Null
                        }
                    },
                    LiteralType::DeclrFunc(func) => {
                        let mut args_eval = vec![];
                        for arg in args {
                            args_eval.push(arg.eval(env.clone()))
                        }

                        (*func.func).call(args_eval)
                    }
                    // @todo add other call types
                    _ => {
                        // @error invalid call
                        LiteralType::Null
                    }
                }
            }
            Expression::Grouping { expression, .. } => expression.eval(env),
            Expression::Value { value, .. } => value.clone(),
            // @todo add func expression
            Expression::Func { .. } => LiteralType::Null,
            // @todo change array items to expressions
            Expression::Array { .. } => LiteralType::Null,
            // @todo handle after adding asynchronocity
            Expression::Await { .. } => LiteralType::Null,
            _ => LiteralType::Any,
        }
    }
}
