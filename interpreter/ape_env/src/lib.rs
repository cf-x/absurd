use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ape_ast::LiteralType;

type EnvValueType = Rc<RefCell<HashMap<String, LiteralType>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub values: EnvValueType,
    pub pub_vals: EnvValueType,
    pub mod_vals: EnvValueType,
    pub mods: Vec<Env>,
    pub locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Env>>,
}

fn init_vals() -> EnvValueType {
    let env = HashMap::new();
    Rc::new(RefCell::new(env))
}

impl Env {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        Self {
            values: init_vals(),
            pub_vals: init_vals(),
            mod_vals: init_vals(),
            mods: Vec::new(),
            locals: Rc::new(RefCell::new(locals)),
            enclosing: None,
        }
    }

    pub fn enclose(&self) -> Env {
        Self {
            values: self.get_empty_rc(),
            pub_vals: self.get_empty_rc(),
            mod_vals: self.get_empty_rc(),
            mods: self.mods.clone(),
            locals: self.locals.clone(),
            enclosing: Some(Box::new(self.clone())),
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
        let d = self.locals.borrow().get(&id).cloned();
        self.get_int(name.as_str(), d)
    }

    pub fn get_int(&self, name: &str, d: Option<usize>) -> Option<LiteralType> {
        if d.is_none() {
            match &self.enclosing {
                Some(env) => env.get_int(name, d),
                None => self.values.borrow().get(name).cloned(),
            }
        } else {
            let d = d.expect("@error failed to get a distance");
            if d <= 0 {
                self.values.borrow().get(name).cloned()
            } else {
                match &self.enclosing {
                    Some(env) => env.get_int(name, Some(d - 1)),
                    None => {
                        panic!("@error failed to resolve a value");
                    }
                }
            }
        }
    }

    fn get_empty_rc(&self) -> EnvValueType {
        Rc::new(RefCell::new(HashMap::new()))
    }
}
