use crate::{
    ast::{CallType, FuncBody, FuncImpl, FuncValueType, LiteralType, Token, TokenType::*},
    env::Env,
    interpreter::run_func,
};
use core::{
    cmp::Eq,
    hash::{Hash, Hasher},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Array {
        id: usize,
        items: Vec<Expression>,
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
                call_type: _,
                ..
            } => {
                let call: LiteralType = name.eval(env.clone());

                match call {
                    LiteralType::Func(func) => match func {
                        FuncValueType::Func(func) => run_func(func, args, env),
                        _ => {
                            // @error invalid function call
                            panic!("Invalid function call");
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
                        // if call_type.clone() == CallType::Array {
                        //     return LiteralType::Array(vec![args[0].clone()]);
                        // }
                        LiteralType::Null
                    }
                }
            }
            Expression::Grouping { expression, .. } => expression.eval(env),
            Expression::Value { value, .. } => value.clone(),
            Expression::Func {
                name,
                value_type,
                body,
                params,
                is_pub,
                is_async,
                id: _,
            } => {
                let call = FuncImpl {
                    name: name.lexeme.clone(),
                    value_type: value_type.clone(),
                    body: FuncBody::Statements(match body {
                        FuncBody::Statements(stmts) => stmts.iter().map(|x| x.clone()).collect(),
                        _ => {
                            // @error invalid function body
                            panic!("invalid function body")
                        }
                    }),
                    params: params
                        .iter()
                        .map(|(name, value_type)| (name.clone(), value_type.clone()))
                        .collect(),
                    is_pub: *is_pub,
                    is_async: *is_async,
                    is_impl: false,
                    is_mut: false,
                    env: Env::new(HashMap::new()),
                };
                let func = LiteralType::Func(FuncValueType::Func(call));
                func
            }
            Expression::Array { items, .. } => LiteralType::Array(items.clone()),
            // @todo handle after adding asynchronocity
            Expression::Await { .. } => LiteralType::Null,
            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => self.eval_binary(left, operator, right, env),
            Expression::Unary { operator, left, .. } => self.eval_unary(operator, left, env),
        }
    }

    fn eval_unary(&self, operator: &Token, left: &Expression, env: Env) -> LiteralType {
        let left = left.eval(env.clone());
        match (operator.clone().token, left.clone()) {
            (Minus, LiteralType::Number(a)) => LiteralType::Number(-a),
            (Not, _) => LiteralType::Boolean(!left.is_truthy()),
            (NotNot, _) => LiteralType::Boolean(!!left.is_truthy()),
            (Square, LiteralType::Number(a)) => LiteralType::Number(a * a),
            (Decr, LiteralType::Number(a)) => LiteralType::Number(a - 1.0),
            (Increment, LiteralType::Number(a)) => LiteralType::Number(a + 1.0),
            _ => LiteralType::Null,
        }
    }

    fn eval_binary(
        &self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
        env: Env,
    ) -> LiteralType {
        let left = left.eval(env.clone());
        let right = right.eval(env.clone());
        match (left.clone(), operator.clone().token, right.clone()) {
            (_, Or, _) => {
                if left.is_truthy() == true {
                    return left;
                }
                return right;
            }
            (_, AndAnd, _) => {
                if left.is_truthy() == false {
                    return left.is_truthy_literal();
                }
                return right;
            }
            (LiteralType::Number(a), Percent, LiteralType::Number(b)) => {
                return LiteralType::Number(a % b);
            }
            (LiteralType::Number(a), Mult, LiteralType::Number(b)) => {
                return LiteralType::Number(a * b);
            }
            (LiteralType::Number(a), Minus, LiteralType::Number(b)) => {
                return LiteralType::Number(a - b);
            }
            (LiteralType::Number(a), Divide, LiteralType::Number(b)) => {
                return LiteralType::Number(a / b);
            }
            (LiteralType::Number(a), Plus, LiteralType::Number(b)) => {
                return LiteralType::Number(a + b);
            }
            (LiteralType::Number(a), Greater, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a > b);
            }
            (LiteralType::Number(a), GreaterOrEq, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a >= b);
            }
            (LiteralType::Number(a), Less, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a < b);
            }
            (LiteralType::Number(a), LessOrEq, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a <= b);
            }
            (_, Eq, _) => {
                return LiteralType::Boolean(left == right);
            }
            (_, NotEq, _) => {
                return LiteralType::Boolean(left != right);
            }
            _ => LiteralType::Any,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Var { name, .. } => write!(f, "{}", name.lexeme),
            Expression::Call { name, args, .. } => {
                let mut args_str = String::new();
                for arg in args {
                    args_str.push_str(&format!("{}, ", arg));
                }
                write!(f, "{}({})", name, args_str)
            }
            Expression::Grouping { expression, .. } => write!(f, "({})", expression),
            Expression::Value { value, .. } => write!(f, "{}", value),
            Expression::Func { name, .. } => write!(f, "{}", name.lexeme),
            Expression::Array { items, .. } => {
                let mut items_str = String::new();
                for item in items {
                    items_str.push_str(&format!("{}, ", item));
                }
                write!(f, "[{}]", items_str)
            }
            Expression::Await { expr, .. } => write!(f, "await {}", expr),
            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                write!(f, "{} {} {}", left, operator.lexeme, right)
            }
            Expression::Unary { left, operator, .. } => write!(f, "{}{}", operator.lexeme, left),
        }
    }
}
