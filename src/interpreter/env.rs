use crate::{
    ast::{LiteralType, Token},
    errors::{Error, ErrorCode::*},
};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, process::exit, rc::Rc};

type EnvValueType = Rc<RefCell<HashMap<String, ValueType>>>;
type EnvTypeValueType = Rc<RefCell<HashMap<String, Token>>>;
type EnvEnumValueType = Rc<RefCell<HashMap<String, Vec<(Token, Option<Token>)>>>>;
type ModEnvValueType = Rc<RefCell<HashMap<String, Vec<(String, ValueType)>>>>;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueKind {
    Var(VarKind),
    Func(FuncKind),
    Type(Token),
    Enum(Vec<(Token, Option<Token>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarKind {
    pub is_mut: bool,
    pub is_pub: bool,
    pub is_func: bool,
    pub value_type: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncKind {
    // (param_name, param_type)
    pub params: Vec<(String, String)>,
    pub is_async: bool,
    pub is_pub: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValueType {
    pub value: LiteralType,
    pub kind: ValueKind,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub values: EnvValueType,
    pub pub_vals: EnvValueType,
    pub mod_vals: ModEnvValueType,
    pub type_values: EnvTypeValueType,
    pub enums: EnvEnumValueType,
    pub mods: Vec<Env>,
    pub locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Rc<RefCell<Env>>>,
    err: Error,
}

impl Env {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        Self {
            values: get_empty_rc(),
            pub_vals: get_empty_rc(),
            mod_vals: get_empty_md(),
            type_values: get_empty_tc(),
            enums: get_empty_ec(),
            mods: Vec::new(),
            locals: Rc::new(RefCell::new(locals)),
            enclosing: None,
            err: Error::new(""),
        }
    }

    pub fn enclose(&self) -> Env {
        Self {
            values: get_empty_rc(),
            pub_vals: get_empty_rc(),
            mod_vals: get_empty_md(),
            type_values: get_empty_tc(),
            enums: get_empty_ec(),
            mods: self.mods.clone(),
            locals: Rc::clone(&self.locals),
            enclosing: Some(Rc::new(RefCell::new(self.clone()))),
            err: Error::new(""),
        }
    }

    pub fn resolve(&self, locals: HashMap<usize, usize>) {
        for (k, v) in locals.iter() {
            self.locals.borrow_mut().insert(*k, *v);
        }
    }

    // variable value definitions

    pub fn define_var(&self, k: String, v: LiteralType, f: VarKind) {
        self.values.borrow_mut().insert(
            k,
            ValueType {
                value: v,
                kind: ValueKind::Var(f),
            },
        );
    }

    pub fn define_pub_var(&self, k: String, v: LiteralType, f: VarKind) {
        self.pub_vals.borrow_mut().insert(
            k,
            ValueType {
                value: v,
                kind: ValueKind::Var(f),
            },
        );
    }

    pub fn define_mod_var(&self, source: String, f: LiteralType, k: String, v: VarKind) {
        let mut mod_vals = self.mod_vals.borrow_mut();
        let entry = mod_vals.entry(source).or_insert_with(Vec::new);
        entry.push((
            k,
            ValueType {
                value: f,
                kind: ValueKind::Var(v),
            },
        ));
    }

    // function value definitions

    pub fn define_func(&self, k: String, v: LiteralType, f: FuncKind) {
        self.values.borrow_mut().insert(
            k,
            ValueType {
                value: v,
                kind: ValueKind::Func(f),
            },
        );
    }

    pub fn define_pub_func(&self, k: String, v: LiteralType, f: FuncKind) {
        self.pub_vals.borrow_mut().insert(
            k,
            ValueType {
                value: v,
                kind: ValueKind::Func(f),
            },
        );
    }

    pub fn define_mod_func(&self, source: String, f: LiteralType, k: String, v: FuncKind) {
        let mut mod_vals = self.mod_vals.borrow_mut();
        let entry = mod_vals.entry(source).or_insert_with(Vec::new);
        entry.push((
            k,
            ValueType {
                value: f,
                kind: ValueKind::Func(v),
            },
        ));
    }

    // type value definitions

    pub fn define_type(&self, k: String, v: Token) {
        self.type_values.borrow_mut().insert(k, v);
    }

    pub fn define_pub_type(&self, k: String, v: Token) {
        self.pub_vals.borrow_mut().insert(
            k,
            ValueType {
                value: LiteralType::Void,
                kind: ValueKind::Type(v),
            },
        );
    }

    pub fn define_mod_type(&self, source: String, f: LiteralType, k: String, v: Token) {
        let mut mod_vals = self.mod_vals.borrow_mut();
        let entry = mod_vals.entry(source).or_insert_with(Vec::new);
        entry.push((
            k,
            ValueType {
                value: f,
                kind: ValueKind::Type(v),
            },
        ));
    }

    pub fn get_type(&self, k: &str) -> Token {
        self.type_values
            .borrow_mut()
            .get(k)
            .unwrap_or(&Token::null())
            .clone()
    }

    // enum value definitions

    pub fn define_enum(&self, k: String, v: Vec<(Token, Option<Token>)>) {
        self.enums.borrow_mut().insert(k, v);
    }

    pub fn define_pub_enum(&self, k: String, v: Vec<(Token, Option<Token>)>) {
        self.pub_vals.borrow_mut().insert(
            k,
            ValueType {
                value: LiteralType::Void,
                kind: ValueKind::Enum(v),
            },
        );
    }

    pub fn define_mod_enum(
        &self,
        source: String,
        f: LiteralType,
        k: String,
        v: Vec<(Token, Option<Token>)>,
    ) {
        let mut mod_vals = self.mod_vals.borrow_mut();
        let entry = mod_vals.entry(source).or_insert_with(Vec::new);
        entry.push((
            k,
            ValueType {
                value: f,
                kind: ValueKind::Enum(v),
            },
        ));
    }

    pub fn get_enum(&self, k: &str) -> Vec<(Token, Option<Token>)> {
        self.enums.borrow_mut().get(k).unwrap_or(&vec![]).clone()
    }

    // global
    pub fn remove(&self, name: String) {
        self.values.borrow_mut().remove(&name);
    }

    pub fn get(&self, name: String, id: usize) -> Option<ValueType> {
        let d = self.locals.borrow_mut().get(&id).cloned();
        self.get_int(name.as_str(), d)
    }

    pub fn get_int(&self, name: &str, d: Option<usize>) -> Option<ValueType> {
        match d {
            None => match &self.enclosing {
                Some(env) => env.borrow_mut().get_int(name, None),
                None => self
                    .values
                    .borrow_mut()
                    .get(name)
                    .cloned()
                    .or_else(|| self.pub_vals.borrow_mut().get(name).cloned()),
            },
            Some(depth) => {
                if depth == 0 {
                    self.values.borrow_mut().get(name).cloned()
                } else {
                    match &self.enclosing {
                        Some(env) => env.borrow_mut().get_int(name, Some(depth - 1)),
                        None => {
                            self.err.throw(E0x502, 0, (0, 0), vec![]);
                            exit(1);
                        }
                    }
                }
            }
        }
    }

    pub fn assing(&self, name: String, value: ValueType, id: usize) -> bool {
        let d = self.locals.borrow_mut().get(&id).cloned();
        self.set_int(name.as_str(), value, d)
    }

    pub fn set_int(&self, name: &str, value: ValueType, d: Option<usize>) -> bool {
        if d.is_none() {
            match &self.enclosing {
                Some(env) => env.borrow_mut().set_int(name, value, d),
                None => self
                    .borrow()
                    .values
                    .borrow_mut()
                    .insert(name.to_string(), value)
                    .is_some(),
            }
        } else {
            let d = match d {
                Some(d) => d,
                None => {
                    self.err.throw(E0x501, 0, (0, 0), vec![]);
                    exit(1)
                }
            };
            if d <= 0 {
                self.values.borrow_mut().remove(name);
                true
            } else {
                match &self.enclosing {
                    Some(env) => env.borrow_mut().set_int(name, value, Some(d - 1)),
                    None => {
                        self.err.throw(E0x502, 0, (0, 0), vec![]);
                        exit(1);
                    }
                }
            }
        }
    }
}

fn get_empty_rc() -> EnvValueType {
    Rc::new(RefCell::new(HashMap::new()))
}

fn get_empty_tc() -> EnvTypeValueType {
    Rc::new(RefCell::new(HashMap::new()))
}

fn get_empty_ec() -> EnvEnumValueType {
    Rc::new(RefCell::new(HashMap::new()))
}

fn get_empty_md() -> ModEnvValueType {
    Rc::new(RefCell::new(HashMap::new()))
}

// @todo better memory manager
