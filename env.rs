use alloc::rc::Rc;
use core::cell::RefCell;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

use crate::FnvHashMap;

use crate::types::MalVal::{List, Sym, Vector};
use crate::types::{error, list, MalRet, MalVal};

pub struct EnvStruct {
    data: RefCell<FnvHashMap<String, MalVal>>,
    outer: Option<Env>,
}

pub type Env = Rc<EnvStruct>;

// TODO: it would be nice to use impl here but it doesn't work on
// a deftype (i.e. Env)

pub fn env_new(outer: Option<Env>) -> Env {
    Rc::new(EnvStruct {
        data: RefCell::new(FnvHashMap::default()),
        outer,
    })
}

// TODO: mbinds and exprs as & types
pub fn env_bind(outer: Env, mbinds: &MalVal, exprs: Vec<MalVal>) -> Result<Env, MalVal> {
    let env = env_new(Some(outer));
    match mbinds {
        List(binds, _) | Vector(binds, _) => {
            let mut has_variadic = false;
            for (i, b) in binds.iter().enumerate() {
                match b {
                    Sym(s) if s == "&" => {
                        env_set(&env, &binds[i + 1], list(exprs[i..].to_vec()))?;
                        has_variadic = true;
                        break;
                    }
                    _ => {
                        if i >= exprs.len() {
                            return error("wrong number of arguments: function requires more arguments than provided");
                        }
                        env_set(&env, b, exprs[i].clone())?;
                    }
                }
            }
            // Check for too many arguments (only if not variadic)
            if !has_variadic && exprs.len() > binds.len() {
                return error("wrong number of arguments: function received more arguments than expected");
            }
            Ok(env)
        }
        _ => error("env_bind binds not List/Vector"),
    }
}

pub fn env_get(env: &Env, key: &str) -> Option<MalVal> {
    let mut mut_env = env;
    loop {
        if let Some(value) = mut_env.data.borrow().get(key) {
            return Some(value.clone());
        } else if let Some(outer) = &mut_env.outer {
            mut_env = outer;
        } else {
            return None;
        }
    }
}

pub fn env_set(env: &Env, key: &MalVal, val: MalVal) -> MalRet {
    match key {
        Sym(s) => {
            env_sets(env, s, val.clone());
            Ok(val)
        }
        _ => error("Env.set called with non-Str"),
    }
}

pub fn env_sets(env: &Env, key: &str, val: MalVal) {
    env.data.borrow_mut().insert(key.to_string(), val);
}
