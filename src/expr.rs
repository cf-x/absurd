use crate::env::{ValueKind, ValueType, VarKind};
use crate::errors::{Error, ErrorCode::*};
use crate::{
    ast::{CallType, FuncBody, FuncImpl, FuncValueType, LiteralType, Token, TokenType::*},
    env::Env,
    interpreter::run_func,
};
use core::{
    cmp::Eq,
    hash::{Hash, Hasher},
};
use std::process::exit;
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Assign {
        id: usize,
        name: Token,
        value: Box<Expression>,
    },
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
    fn err(&self) -> Error {
        Error::new("")
    }

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
            Expression::Assign { id, .. } => *id,
        }
    }

    pub fn eval(&self, env: Rc<RefCell<Env>>) -> LiteralType {
        match self {
            Expression::Assign { name, value, .. } => {
                let val = (*value).eval(Rc::clone(&env));
                let t = env.borrow().get(name.lexeme.clone(), self.id());
                match t {
                    Some(v) => match v.kind {
                        ValueKind::Var(s) => {
                            if !s.is_mut {
                                self.err().throw(E0x410, name.line, name.pos, vec![]);
                            }
                            if s.is_pub {
                                self.err().throw(E0x411, name.line, name.pos, vec![]);
                            }
                            if v.value.type_name() != val.type_name() {
                                self.err().throw(
                                    E0x412,
                                    name.line,
                                    name.pos,
                                    vec![name.clone().lexeme],
                                );
                            }
                        }
                        _ => {
                            self.err().throw(E0x413, name.line, name.pos, vec![]);
                        }
                    },
                    None => {}
                }
                let ass_val = ValueType {
                    kind: ValueKind::Var(VarKind {
                        is_mut: false,
                        is_pub: false,
                        is_func: false,
                    }),
                    value: val.clone(),
                };
                let assigned = env
                    .borrow_mut()
                    .assing(name.lexeme.clone(), ass_val, self.id());

                if assigned {
                    val
                } else {
                    self.err().throw(E0x414, name.line, name.pos, vec![]);
                    exit(1);
                }
            }
            Expression::Var { name, .. } => {
                match env.borrow().get(name.lexeme.clone(), self.id()) {
                    Some(v) => v.clone().value,
                    None => match env.borrow().values.borrow().get(name.lexeme.as_str()) {
                        Some(v) => v.clone().value,
                        None => LiteralType::Null,
                    },
                }
            }
            Expression::Call {
                name,
                args,
                call_type: _,
                ..
            } => {
                let call: LiteralType = name.eval(Rc::clone(&env));
                match call {
                    LiteralType::Func(func) => match func {
                        FuncValueType::Func(func) => run_func(func, args, env),
                        _ => {
                            self.err().throw(E0x407, 0, (0, 0), vec![]);
                            exit(0);
                        }
                    },
                    LiteralType::DeclrFunc(func) => {
                        let mut args_eval = vec![];
                        for arg in args {
                            args_eval.push(arg.eval(Rc::clone(&env)))
                        }

                        (*func.func).call(args_eval)
                    }
                    // @todo add other call types
                    _ => {
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
                            self.err().throw(E0x403, 0, (0, 0), vec![]);
                            exit(0);
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
                    env: Rc::clone(&env),
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

    fn eval_unary(
        &self,
        operator: &Token,
        left: &Expression,
        env: Rc<RefCell<Env>>,
    ) -> LiteralType {
        let left = left.eval(Rc::clone(&env));
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
        env: Rc<RefCell<Env>>,
    ) -> LiteralType {
        let left = left.eval(Rc::clone(&env));
        let right = right.eval(Rc::clone(&env));
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
            (LiteralType::Number(a), Eq, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a == b);
            }
            (LiteralType::Number(a), NotEq, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a != b);
            }
            (LiteralType::String(a), Eq, LiteralType::String(b)) => {
                return LiteralType::Boolean(a == b);
            }
            (LiteralType::String(a), NotEq, LiteralType::String(b)) => {
                return LiteralType::Boolean(a != b);
            }
            (LiteralType::Char(a), Eq, LiteralType::Char(b)) => {
                return LiteralType::Boolean(a == b);
            }
            (LiteralType::Char(a), NotEq, LiteralType::Char(b)) => {
                return LiteralType::Boolean(a != b);
            }
            (LiteralType::Boolean(a), Eq, LiteralType::Boolean(b)) => {
                return LiteralType::Boolean(a == b);
            }
            (LiteralType::Boolean(a), NotEq, LiteralType::Boolean(b)) => {
                return LiteralType::Boolean(a != b);
            }
            (LiteralType::Null, Eq, LiteralType::Null) => {
                return LiteralType::Boolean(true);
            }
            (LiteralType::Null, NotEq, LiteralType::Null) => {
                return LiteralType::Boolean(false);
            }
            (_, Eq, _) => {
                return LiteralType::Boolean(false);
            }
            (_, NotEq, _) => {
                return LiteralType::Boolean(false);
            }
            _ => LiteralType::Null,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Assign { name, value, .. } => write!(f, "{} = {}", name.lexeme, value),
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
