use super::env::{Env, ValueKind, ValueType, VarKind};
use super::types::TypeKind;
use crate::ast::{LiteralKind, Statement};
use crate::errors::{Error, ErrorCode::*};
use crate::interpreter::types::{type_check, typekind_to_literaltype};
use crate::manifest::Project;
use crate::{
    ast::{CallType, FuncBody, FuncImpl, FuncValueType, LiteralType, Token, TokenType::*},
    interpreter::run_func,
};
use core::cmp::Eq;
use std::borrow::Cow;
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
    Object {
        id: usize,
        fields: Vec<(String, Expression)>,
    },
    Assign {
        id: usize,
        name: Token,
        value: Box<Expression>,
        kind: AssignKind,
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
    Method {
        id: usize,
        left: Box<Expression>,
        name: Token,
        args: Vec<Expression>,
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

impl Eq for Expression {}
impl Expression {
    fn err(&self) -> Error {
        Error::new("", Project::new())
    }

    pub fn id(&self) -> usize {
        match self {
            Expression::Object { id, .. } => *id,
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
            Expression::Method { id, .. } => *id,
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
            Expression::Object { fields, .. } => LiteralType::Obj(fields.clone()),
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
                                        if let TypeKind::Or { left, right } = *c {
                                            let left_true = if let TypeKind::Var { name } = *left {
                                                type_check(&name, &val, &env)
                                            } else {
                                                false
                                            };

                                            let right_true = if let TypeKind::Var { name } = *right
                                            {
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
                match env.borrow().get(name.lexeme.clone(), self.id()) {
                    Some(v) => v.clone().value,
                    None => match env.borrow().values.borrow().get(name.lexeme.as_str()) {
                        Some(v) => v.clone().value,
                        None => LiteralType::Null,
                    },
                }
            }
            Expression::Method {
                left, name, args, ..
            } => {
                let literal = left.eval(env.clone());
                self.eval_literal_method_b(literal, name.clone(), args, env)
            }
            Expression::Call { name, args, .. } => {
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
                        let evals = args
                            .iter()
                            .map(|arg| Some(arg.eval(Rc::clone(&env))))
                            .collect();
                        (*func.func).call(evals)
                    }
                    LiteralType::Array(res) => match args.get(0).unwrap() {
                        Expression::Value { value, .. } => {
                            if let LiteralType::Number(n) = value {
                                res.get(*n as usize).unwrap().eval(env)
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
                    LiteralType::Obj(obj) => match args.get(0).unwrap() {
                        Expression::Value { value, .. } => {
                            if let LiteralType::String(s) = value {
                                let mut res = LiteralType::Null;
                                for (k, v) in obj {
                                    if k == *s {
                                        res = v.eval(env.clone());
                                    }
                                }
                                res
                            } else {
                                LiteralType::Null
                            }
                        }
                        _ => LiteralType::Null,
                    },
                    _ => self.eval_literal_method(call, args, env),
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
                        FuncBody::Statements(stmts) => stmts
                            .iter()
                            .map(|stmt| {
                                if let Statement::Return { expr } = stmt {
                                    let v = &(*expr).eval(env.clone());
                                    if !type_check(value_type, v, &env) {
                                        self.err().throw(
                                            E0x301,
                                            name.line,
                                            name.pos,
                                            vec![value_type.clone().lexeme, v.to_string()],
                                        );
                                    }
                                }
                                stmt.clone()
                            })
                            .collect(),
                        FuncBody::Expression(e) => {
                            let v = &(*e).eval(env.clone());
                            if !type_check(value_type, v, &env) {
                                self.err().throw(
                                    E0x301,
                                    name.line,
                                    name.pos,
                                    vec![value_type.clone().lexeme, v.to_string()],
                                );
                            }
                            vec![Statement::Expression { expr: *e.clone() }]
                        }
                    }),
                    params: params
                        .iter()
                        .map(|(name, value_type)| (name.clone(), value_type.clone()))
                        .collect(),
                    is_pub: *is_pub,
                    is_async: *is_async,
                    env: Rc::clone(&env),
                };
                let func = LiteralType::Func(FuncValueType::Func(call));
                func
            }
            Expression::Array { items, .. } => LiteralType::Array(items.clone()),
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

    fn eval_literal_method_b(
        &self,
        literal: LiteralType,
        name: Token,
        args: &[Expression],
        env: Rc<RefCell<Env>>,
    ) -> LiteralType {
        match literal {
            LiteralType::Number(n) => {
                let name_s = name.lexeme;
                match name_s.as_str() {
                    "sqr" => return LiteralType::Number(n * n),
                    "add" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n + m);
                            }
                        }
                    }
                    "sub" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n - m);
                            }
                        }
                    }
                    "mult" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n * m);
                            }
                        }
                    }
                    "div" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n / m);
                            }
                        }
                    }
                    "rem" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n % m);
                            }
                        }
                    }
                    "sqrt" => return LiteralType::Number(n.sqrt()),
                    "cbrt" => return LiteralType::Number(n.cbrt()),
                    "pow" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n.powf(m));
                            }
                        }
                    }
                    "log" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n.log(m));
                            }
                        }
                    }
                    "sin" => return LiteralType::Number(n.sin()),
                    "asin" => return LiteralType::Number(n.asin()),
                    "cos" => return LiteralType::Number(n.cos()),
                    "acos" => return LiteralType::Number(n.acos()),
                    "tan" => return LiteralType::Number(n.tan()),
                    "atan" => return LiteralType::Number(n.atan()),
                    "abs" => return LiteralType::Number(n.abs()),
                    "floor" => return LiteralType::Number(n.floor()),
                    "ceil" => return LiteralType::Number(n.ceil()),
                    "round" => return LiteralType::Number(n.round()),
                    "signum" => return LiteralType::Number(n.signum()),
                    "hypot" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n.hypot(m));
                            }
                        }
                    }
                    "exp" => return LiteralType::Number(n.exp()),
                    "exp2" => return LiteralType::Number(n.exp2()),
                    "exp_m1" => return LiteralType::Number(n.exp_m1()),
                    "ln" => return LiteralType::Number(n.ln()),
                    "max" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n.max(m));
                            }
                        }
                    }
                    "min" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number(n.min(m));
                            }
                        }
                    }
                    "avg" => {
                        if !args.is_empty() {
                            let m = args[0].eval(env.clone());
                            if let LiteralType::Number(m) = m {
                                return LiteralType::Number((n + m) / 2.0);
                            }
                        }
                    }
                    "to_degrees" => return LiteralType::Number(n.to_degrees()),
                    "to_radians" => return LiteralType::Number(n.to_radians()),
                    _ => {}
                }
            }
            LiteralType::String(s) => {
                let name_s = name.lexeme;
                match name_s.as_str() {
                    "len" => return LiteralType::Number(s.len() as f32),
                    "is_empty" => return LiteralType::Boolean(s.is_empty()),
                    "contains" => {
                        if !args.is_empty() {
                            let substr = args[0].eval(env.clone());
                            if let LiteralType::String(substr) = substr {
                                return LiteralType::Boolean(s.contains(&substr));
                            }
                        }
                    }
                    "starts_with" => {
                        if !args.is_empty() {
                            let prefix = args[0].eval(env.clone());
                            if let LiteralType::String(prefix) = prefix {
                                return LiteralType::Boolean(s.starts_with(&prefix));
                            }
                        }
                    }
                    "ends_with" => {
                        if !args.is_empty() {
                            let suffix = args[0].eval(env.clone());
                            if let LiteralType::String(suffix) = suffix {
                                return LiteralType::Boolean(s.ends_with(&suffix));
                            }
                        }
                    }
                    "to_uppercase" => return LiteralType::String(s.to_uppercase()),
                    "to_lowercase" => return LiteralType::String(s.to_lowercase()),
                    "trim" => return LiteralType::String(s.trim().to_string()),
                    "trim_start" => return LiteralType::String(s.trim_start().to_string()),
                    "trim_end" => return LiteralType::String(s.trim_end().to_string()),
                    "replace" => {
                        if args.len() > 1 {
                            let from = args[0].eval(env.clone());
                            let to = args[1].eval(env.clone());
                            if let (LiteralType::String(from), LiteralType::String(to)) = (from, to)
                            {
                                return LiteralType::String(s.replace(&from, &to));
                            }
                        }
                    }
                    "find" => {
                        if !args.is_empty() {
                            let substr = args[0].eval(env.clone());
                            if let LiteralType::String(substr) = substr {
                                if let Some(index) = s.find(&substr) {
                                    return LiteralType::Number(index as f32);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        LiteralType::Any
    }

    fn eval_literal_method(
        &self,
        literal: LiteralType,
        args: &[Expression],
        env: Rc<RefCell<Env>>,
    ) -> LiteralType {
        if let Expression::Call { name, .. } = &args[0] {
            if let Cow::Borrowed(Expression::Var { name, .. }) =
                Cow::Borrowed::<Expression>(name).clone()
            {
                return self.eval_literal_method_b(literal, name.clone(), &args[1..], env);
            } else {
                LiteralType::Null
            }
        } else {
            LiteralType::Null
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
            Expression::Object { fields, .. } => {
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
            Expression::Method {
                left, name, args, ..
            } => {
                let mut args_str = String::new();
                for arg in args {
                    args_str.push_str(&format!("{}, ", arg));
                }
                write!(f, "{}.{}({})", left, name.lexeme, args_str)
            }
        }
    }
}
