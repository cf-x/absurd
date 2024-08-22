use crate::ast::LiteralType;
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

type EnvValueType = Rc<RefCell<HashMap<String, LiteralType>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub values: EnvValueType,
    pub pub_vals: EnvValueType,
    pub mod_vals: EnvValueType,
    pub mods: Vec<Env>,
    pub locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        Self {
            values: get_empty_rc(),
            pub_vals: get_empty_rc(),
            mod_vals: get_empty_rc(),
            mods: Vec::new(),
            locals: Rc::new(RefCell::new(locals)),
            enclosing: None,
        }
    }

    pub fn enclose(&self) -> Env {
        Self {
            values: get_empty_rc(),
            pub_vals: get_empty_rc(),
            mod_vals: get_empty_rc(),
            mods: self.mods.clone(),
            locals: Rc::clone(&self.locals),
            enclosing: Some(Rc::new(RefCell::new(self.clone()))),
        }
    }

    pub fn resolve(&self, locals: HashMap<usize, usize>) {
        for (k, v) in locals.iter() {
            self.locals.borrow_mut().insert(*k, *v);
        }
    }

    pub fn define(&self, k: String, v: LiteralType) {
        self.values.borrow_mut().insert(k, v);
    }

    pub fn define_pub(&self, k: String, v: LiteralType) {
        self.pub_vals.borrow_mut().insert(k, v);
    }

    pub fn define_mod(&self, k: String, v: LiteralType) {
        self.mod_vals.borrow_mut().insert(k, v);
    }

    pub fn get(&self, name: String, id: usize) -> Option<LiteralType> {
        let d = self.locals.borrow_mut().get(&id).cloned();
        self.get_int(name.as_str(), d)
    }

    pub fn get_int(&self, name: &str, d: Option<usize>) -> Option<LiteralType> {
        if d.is_none() {
            match &self.enclosing {
                Some(env) => env.borrow_mut().get_int(name, d),
                None => self.values.borrow_mut().get(name).cloned(),
            }
        } else {
            let d = d.expect("@error failed to get a distance");
            if d <= 0 {
                self.values.borrow_mut().get(name).cloned()
            } else {
                match &self.enclosing {
                    Some(env) => env.borrow_mut().get_int(name, Some(d - 1)),
                    None => {
                        panic!("@error failed to resolve a value");
                    }
                }
            }
        }
    }

    pub fn assing(&self, name: String, value: LiteralType, id: usize) -> bool {
        let d = self.locals.borrow_mut().get(&id).cloned();
        self.set_int(name.as_str(), value, d)
    }

    pub fn set_int(&self, name: &str, value: LiteralType, d: Option<usize>) -> bool {
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
            let d = d.expect("@error failed to get a distance");
            if d <= 0 {
                self.values.borrow_mut().remove(name);
                true
            } else {
                match &self.enclosing {
                    Some(env) => env.borrow_mut().set_int(name, value, Some(d - 1)),
                    None => {
                        panic!("@error failed to resolve a value");
                    }
                }
            }
        }
    }
}

fn get_empty_rc() -> EnvValueType {
    Rc::new(RefCell::new(HashMap::new()))
}
