use super::env::{Env, ValueKind, ValueType, VarKind};
use super::types::TypeKind;
use crate::ast::{LiteralKind, Statement};
use crate::bundler::parse_expr;
use crate::errors::{Error, ErrorCode::*};
use crate::interpreter::types::{type_check, typekind_to_literaltype};
use crate::{
    ast::{CallType, FuncImpl, LiteralType, Token, TokenType::*},
    interpreter::run_func,
};
use core::cmp::Eq;
use std::process::exit;
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum AssignKind {
    Normal,
    Plus,
    Minus,
    Mult,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    If {
        id: usize,
        cond: Box<Expression>,
        body: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },
    Record {
        id: usize,
        fields: Vec<(String, Expression)>,
    },
    Assign {
        id: usize,
        name: Token,
        value: Box<Expression>,
        kind: AssignKind,
    },
    Vec {
        id: usize,
        items: Vec<Expression>,
    },
    Tuple {
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
        body: Box<Statement>,
        params: Vec<(Token, Token)>,
        is_async: bool,
        is_pub: bool,
    },
    Await {
        id: usize,
        expr: Box<Expression>,
    },
    Range {
        id: usize,
        lhs: usize,
        rhs: usize,
    },
}

impl Eq for Expression {}
impl Expression {
    fn err(&self) -> Error {
        Error::new("")
    }

    pub fn id(&self) -> usize {
        match self {
            Expression::Range { id, .. } => *id,
            Expression::Record { id, .. } => *id,
            Expression::Var { id, .. } => *id,
            Expression::Tuple { id, .. } => *id,
            Expression::Call { id, .. } => *id,
            Expression::Func { id, .. } => *id,
            Expression::Vec { id, .. } => *id,
            Expression::Await { id, .. } => *id,
            Expression::Binary { id, .. } => *id,
            Expression::Unary { id, .. } => *id,
            Expression::Value { id, .. } => *id,
            Expression::Grouping { id, .. } => *id,
            Expression::Assign { id, .. } => *id,
            Expression::If { id, .. } => *id,
        }
    }

    pub fn to_literal(&self) -> LiteralType {
        match self {
            Expression::Value { value, .. } => value.clone(),
            _ => LiteralType::Null,
        }
    }

    pub fn eval(&self, env: Rc<RefCell<Env>>) -> LiteralType {
        match self {
            Expression::Range { lhs, rhs, .. } => {
                let range = lhs.clone()..rhs.clone();
                let vec: Vec<LiteralType> = range.map(|x| LiteralType::Number(x as f32)).collect();
                LiteralType::Vec(vec)
            }
            Expression::If {
                cond,
                body,
                else_branch,
                ..
            } => {
                let val = cond.clone().eval(Rc::clone(&env));
                if val.is_truthy() {
                    return body.clone().eval(Rc::clone(&env));
                } else if else_branch.is_some() {
                    return else_branch.as_ref().unwrap().clone().eval(Rc::clone(&env));
                }

                LiteralType::Null
            }
            Expression::Record { fields, .. } => LiteralType::Record(fields.clone()),
            Expression::Assign {
                name, value, kind, ..
            } => {
                let mut val = (*value).eval(Rc::clone(&env));
                let mut is_mut = false;
                match env.borrow().get(name.lexeme.clone(), self.id()) {
                    Some(v) => match v.clone().kind {
                        ValueKind::Var(s) => {
                            if !s.is_mut {
                                self.err().throw(E0x410, name.line, name.pos, vec![]);
                            }
                            is_mut = true;
                            if s.is_pub {
                                self.err().throw(E0x411, name.line, name.pos, vec![]);
                            }
                            match v.value {
                                LiteralType::Number(x) => match kind {
                                    AssignKind::Plus => match val {
                                        LiteralType::Number(n) => {
                                            val = LiteralType::Number(n + x);
                                        }
                                        _ => {}
                                    },
                                    AssignKind::Minus => match val {
                                        LiteralType::Number(n) => {
                                            val = LiteralType::Number(n - x);
                                        }
                                        _ => {}
                                    },
                                    AssignKind::Mult => match val {
                                        LiteralType::Number(n) => {
                                            val = LiteralType::Number(n * x);
                                        }
                                        _ => {}
                                    },
                                    AssignKind::Div => match val {
                                        LiteralType::Number(n) => {
                                            val = LiteralType::Number(n / x);
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                },
                                _ => {
                                    if kind.clone() != AssignKind::Normal {
                                        self.err().throw(E0x414, name.line, name.pos, vec![]);
                                        exit(1);
                                    }
                                }
                            }

                            if v.value.type_name() != val.type_name() {
                                if let ValueKind::Var(s) = v.kind {
                                    if let Some(LiteralKind::Type(c)) = s.value_type.value.clone() {
                                        if let TypeKind::Either { lhs, rhs } = *c {
                                            let left_true = if let TypeKind::Var { name } = *lhs {
                                                type_check(&name, &val, &env)
                                            } else {
                                                false
                                            };

                                            let right_true = if let TypeKind::Var { name } = *rhs {
                                                type_check(&name, &val, &env)
                                            } else {
                                                false
                                            };

                                            if !left_true && !right_true {
                                                self.err().throw(
                                                    E0x412,
                                                    name.line,
                                                    name.pos,
                                                    vec![name.clone().lexeme],
                                                );
                                            }
                                        } else {
                                            let expected_type = typekind_to_literaltype(*c);
                                            if val != expected_type {
                                                self.err().throw(
                                                    E0x412,
                                                    name.line,
                                                    name.pos,
                                                    vec![name.clone().lexeme],
                                                );
                                            }
                                        }
                                    } else {
                                        self.err().throw(
                                            E0x412,
                                            name.line,
                                            name.pos,
                                            vec![name.clone().lexeme],
                                        );
                                    }
                                }
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
                        is_mut,
                        is_pub: false,
                        is_func: false,
                        value_type: name.clone(),
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
                let lexeme = name.lexeme.as_str();
                let env_borrow = env.borrow();

                if let Some(v) = env_borrow.get(name.lexeme.clone(), self.id()) {
                    v.value.clone()
                } else if let Some(v) = env_borrow.values.borrow().get(lexeme) {
                    v.value.clone()
                } else if env_borrow.enums.borrow().contains_key(lexeme) {
                    LiteralType::Enum {
                        parent: name.clone(),
                        name: Token::null(),
                        value: None,
                    }
                } else {
                    LiteralType::Null
                }
            }

            Expression::Call {
                name,
                args,
                call_type,
                ..
            } => {
                let call: LiteralType = name.eval(Rc::clone(&env));
                match call {
                    LiteralType::Enum { .. } => {
                        if let CallType::Enum = call_type {
                            let parent = if let Expression::Var { name, .. } = *name.clone() {
                                name
                            } else {
                                Token::null()
                            };
                            let name = if let Expression::Var { name, .. } = args.get(0).unwrap() {
                                name.clone()
                            } else {
                                Token::null()
                            };

                            let value = if let Some(v) = args.get(1) {
                                Some(Box::new(v.eval(env)))
                            } else {
                                None
                            };

                            return LiteralType::Enum {
                                parent,
                                name,
                                value,
                            };
                        }

                        LiteralType::Null
                    }
                    LiteralType::Func(func) => run_func(func, args, env),
                    LiteralType::DeclrFunc(func) => {
                        let evals = args
                            .iter()
                            .map(|arg| Some(arg.eval(Rc::clone(&env))))
                            .collect();

                        (*func.func).call(evals)
                    }
                    LiteralType::Vec(res) => match args.get(0).unwrap() {
                        Expression::Value { value, .. } => {
                            if let LiteralType::Number(n) = value {
                                res.get(*n as usize).unwrap().clone()
                            } else {
                                LiteralType::Null
                            }
                        }
                        _ => LiteralType::Null,
                    },
                    LiteralType::Tuple(res) => match args.get(0).unwrap() {
                        Expression::Value { value, .. } => {
                            if let LiteralType::Number(n) = value {
                                res.get(*n as usize).unwrap().clone()
                            } else {
                                LiteralType::Null
                            }
                        }
                        _ => LiteralType::Null,
                    },
                    LiteralType::String(s) => match args.get(0).unwrap().eval(env) {
                        LiteralType::Number(n) => {
                            let mut res = LiteralType::Null;
                            for (i, c) in s.chars().enumerate() {
                                if i == n as usize {
                                    res = LiteralType::Char(c);
                                }
                            }
                            res
                        }
                        _ => LiteralType::Null,
                    },
                    LiteralType::Record(rec) => match args.get(0).unwrap() {
                        Expression::Value { value, .. } => {
                            if let LiteralType::String(s) = value {
                                let mut res = LiteralType::Null;
                                for (k, v) in rec {
                                    if k == *s {
                                        res = v.eval(Rc::clone(&env));
                                    }
                                }
                                res
                            } else {
                                LiteralType::Null
                            }
                        }
                        _ => LiteralType::Null,
                    },
                    _ => LiteralType::Null,
                }
            }
            Expression::Grouping { expression, .. } => expression.eval(env),
            Expression::Value { value, .. } => {
                let v = match value.clone() {
                    LiteralType::String(s) => {
                        let mut result = String::new();
                        let mut idx = 0;

                        while let Some(start) = s[idx..].find('{') {
                            result.push_str(&s[idx..idx + start]);
                            let start_idx = idx + start + 1;

                            if let Some(end) = s[start_idx..].find('}') {
                                let expr = &s[start_idx..start_idx + end];
                                let eval_result =
                                    match parse_expr(expr, self.err()).eval(Rc::clone(&env)) {
                                        LiteralType::String(eval_s) => eval_s,
                                        LiteralType::Number(eval_n) => eval_n.to_string(),
                                        LiteralType::Boolean(eval_b) => eval_b.to_string(),
                                        _ => "null".to_string(),
                                    };

                                result.push_str(&eval_result);
                                idx = start_idx + end + 1;
                            } else {
                                break;
                            }
                        }

                        result.push_str(&s[idx..]);
                        LiteralType::String(result)
                    }
                    c => c,
                };
                v.clone()
            }

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
                    body: Box::new(match *body.clone() {
                        Statement::Block { stmts } => {
                            stmts.iter().for_each(|stmt| {
                                if let Statement::Return { expr } = stmt {
                                    let v = &(*expr).eval(Rc::clone(&env));
                                    if !type_check(value_type, v, &env) {
                                        self.err().throw(
                                            E0x301,
                                            name.line,
                                            name.pos,
                                            vec![value_type.clone().lexeme, v.to_string()],
                                        );
                                    }
                                }
                            });
                            *body.clone()
                        }
                        Statement::Expression { expr } => {
                            let v = &(expr).eval(Rc::clone(&env));
                            if !type_check(value_type, v, &env) {
                                self.err().throw(
                                    E0x301,
                                    name.line,
                                    name.pos,
                                    vec![value_type.clone().lexeme, v.to_string()],
                                );
                            }
                            *body.clone()
                        }
                        _ => Statement::Break {},
                    }),
                    params: params
                        .iter()
                        .map(|(name, value_type)| (name.clone(), value_type.clone()))
                        .collect(),
                    is_pub: *is_pub,
                    is_async: *is_async,
                    env: Rc::clone(&env),
                };
                let func = LiteralType::Func(call);
                func
            }
            Expression::Vec { items, .. } => LiteralType::Vec(
                items
                    .iter()
                    .map(|f| f.eval(Rc::clone(&env)))
                    .collect::<Vec<LiteralType>>()
                    .clone(),
            ),
            Expression::Tuple { items, .. } => LiteralType::Tuple(
                items
                    .iter()
                    .map(|f| f.eval(Rc::clone(&env)))
                    .collect::<Vec<LiteralType>>()
                    .clone(),
            ),
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
            (Min, LiteralType::Number(a)) => LiteralType::Number(-a),
            (Bang, _) => LiteralType::Boolean(!left.is_truthy()),
            (DblBang, _) => LiteralType::Boolean(!!left.is_truthy()),
            (Sqr, LiteralType::Number(a)) => LiteralType::Number(a * a),
            (Decr, LiteralType::Number(a)) => LiteralType::Number(a - 1.0),
            (Incr, LiteralType::Number(a)) => LiteralType::Number(a + 1.0),
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
        match (left.clone(), operator.token.clone(), right.clone()) {
            (_, Or, _) => {
                if left.is_truthy() {
                    return left;
                }
                return right;
            }
            (_, DblAnd, _) => {
                if !left.is_truthy() {
                    return left.is_truthy_literal();
                }
                return right;
            }
            (LiteralType::Number(a), Prcnt, LiteralType::Number(b)) => {
                return LiteralType::Number(a % b);
            }
            (LiteralType::Number(a), Mul, LiteralType::Number(b)) => {
                return LiteralType::Number(a * b);
            }
            (LiteralType::Number(a), Min, LiteralType::Number(b)) => {
                return LiteralType::Number(a - b);
            }
            (LiteralType::Number(a), Div, LiteralType::Number(b)) => {
                return LiteralType::Number(a / b);
            }
            (LiteralType::Number(a), Plus, LiteralType::Number(b)) => {
                return LiteralType::Number(a + b);
            }
            (LiteralType::Number(a), Gr, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a > b);
            }
            (LiteralType::Number(a), GrOrEq, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a >= b);
            }
            (LiteralType::Number(a), Ls, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a < b);
            }
            (LiteralType::Number(a), LsOrEq, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a <= b);
            }
            (LiteralType::Number(a), Eq, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a == b);
            }
            (LiteralType::Number(a), BangEq, LiteralType::Number(b)) => {
                return LiteralType::Boolean(a != b);
            }
            (LiteralType::Number(a), Plus, LiteralType::Vec(v)) => {
                let nums = v
                    .iter()
                    .map(|p| {
                        if let LiteralType::Number(c) = p {
                            LiteralType::Number(c.clone() + a)
                        } else {
                            LiteralType::Null
                        }
                    })
                    .collect();

                return LiteralType::Vec(nums);
            }
            (LiteralType::Number(a), Min, LiteralType::Vec(v)) => {
                let nums = v
                    .iter()
                    .map(|p| {
                        if let LiteralType::Number(c) = p {
                            LiteralType::Number(c.clone() - a)
                        } else {
                            LiteralType::Null
                        }
                    })
                    .collect();

                return LiteralType::Vec(nums);
            }
            (LiteralType::Number(a), Mul, LiteralType::Vec(v)) => {
                let nums = v
                    .iter()
                    .map(|p| {
                        if let LiteralType::Number(c) = p {
                            LiteralType::Number(c.clone() * a)
                        } else {
                            LiteralType::Null
                        }
                    })
                    .collect();

                return LiteralType::Vec(nums);
            }
            (LiteralType::Number(a), Div, LiteralType::Vec(v)) => {
                let nums = v
                    .iter()
                    .map(|p| {
                        if let LiteralType::Number(c) = p {
                            LiteralType::Number(c.clone() / a)
                        } else {
                            LiteralType::Null
                        }
                    })
                    .collect();

                return LiteralType::Vec(nums);
            }
            (LiteralType::String(a), Eq, LiteralType::String(b)) => {
                return LiteralType::Boolean(a == b);
            }
            (LiteralType::String(a), BangEq, LiteralType::String(b)) => {
                return LiteralType::Boolean(a != b);
            }
            (LiteralType::Char(a), Eq, LiteralType::Char(b)) => {
                return LiteralType::Boolean(a == b);
            }
            (LiteralType::Char(a), BangEq, LiteralType::Char(b)) => {
                return LiteralType::Boolean(a != b);
            }
            (LiteralType::Boolean(a), Eq, LiteralType::Boolean(b)) => {
                return LiteralType::Boolean(a == b);
            }
            (LiteralType::Boolean(a), BangEq, LiteralType::Boolean(b)) => {
                return LiteralType::Boolean(a != b);
            }
            (LiteralType::Null, Eq, LiteralType::Null) => {
                return LiteralType::Boolean(true);
            }
            (LiteralType::Null, BangEq, LiteralType::Null) => {
                return LiteralType::Boolean(false);
            }
            (_, Eq, _) => {
                return LiteralType::Boolean(false);
            }
            (_, BangEq, _) => {
                return LiteralType::Boolean(false);
            }
            _ => LiteralType::Null,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Range { lhs, rhs, .. } => {
                write!(f, "{}..{}", lhs, rhs)
            }
            Expression::If {
                cond,
                body,
                else_branch,
                ..
            } => {
                if else_branch.is_some() {
                    return write!(
                        f,
                        "if {}: {} ? {}",
                        cond,
                        body,
                        else_branch.as_ref().unwrap()
                    );
                }

                write!(f, "if {}: {}", cond, body)
            }
            Expression::Record { fields, .. } => {
                let mut fields_str = String::new();
                for (name, value) in fields {
                    fields_str.push_str(&format!("{}: {}, ", name, value));
                }
                write!(f, "{{{}}}", fields_str)
            }
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
            Expression::Vec { items, .. } => {
                let mut items_str = String::new();
                for item in items {
                    items_str.push_str(&format!("{}, ", item));
                }
                write!(f, "[{}]", items_str)
            }
            Expression::Tuple { items, .. } => {
                let mut items_str = String::new();
                for item in items {
                    items_str.push_str(&format!("{}, ", item));
                }
                write!(f, "({})", items_str)
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

// @todo better organize it
