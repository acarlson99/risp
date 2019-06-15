/******************************************************************************
** @crates and modules
******************************************************************************/

use fnv::FnvHashMap;

use std::sync::Arc;

use crate::risp::{eval, load_logic, RErr, RVal, RVal::*};

/******************************************************************************
** @environment
******************************************************************************/

pub struct REnv {
    pub symbols: FnvHashMap<String, RVal>,
}

impl REnv {
    pub fn new() -> Self {
        let mut env = REnv {
            symbols: FnvHashMap::default(),
        };
        load_logic(&mut env);
        env
    }
    pub fn def<S>(&mut self, key: S, val: RVal) -> RVal
    where
        S: Into<String>,
    {
        self.symbols.insert(key.into(), val.clone());
        val
    }
    pub fn get_value(&self, key: &RVal) -> RVal {
        match &key {
            _RSym(s) => match self.symbols.get(&s.to_string()) {
                Some(v) => v.clone(),
                None => RErrUnboundSymbol!(key),
            },
            _ => RNil,
        }
    }
}

/******************************************************************************
** @builtins
******************************************************************************/

impl REnv {
    pub fn try_builtin(&mut self, x: &RVal, xs: &[RVal]) -> RVal {
        match x {
            _RSym(s) => match &s[..] {
                "def" => self.builtin_def(&xs[0], &xs[1..]),
                "if" => self.builtin_if(&xs[0], &xs[1..]),
                _ => RNil,
            },
            _ => RErrExpected!("Sym", x.clone().variant()),
        }
    }
    // TODO: fix error messages
    fn builtin_def(&mut self, x: &RVal, xs: &[RVal]) -> RVal {
        match xs.len() {
            0 => RErrExpected!("(Sym Any)", x.variant()),
            1 => match x {
                _RSym(s) => self.def(&s[..], xs[0].clone()),
                _ => RErrExpected!(
                    "(Sym Any)",
                    format!(
                        "{} {}",
                        x.clone().variant(),
                        RVec(Arc::new(xs.to_vec())).variant()
                    )
                ),
            },
            _ => RErrExpected!(
                "(Sym Any)",
                format!(
                    "{} {}",
                    x.clone().variant(),
                    RVec(Arc::new(xs.to_vec())).variant()
                )
            ),
        }
    }
    fn builtin_if(&mut self, x: &RVal, xs: &[RVal]) -> RVal {
        match xs.len() {
            0 => RErrExpected!("(Bool Any Any)", x.variant()),
            2 => match eval(&x, self) {
                RBool(b) => {
                    let idx = if b { 0 } else { 1 };
                    eval(&xs[idx], self)
                }
                _ => RErrExpected!(
                    "(Bool Any Any)",
                    format!(
                        "{} {}",
                        x.clone().variant(),
                        RVec(Arc::new(xs.to_vec())).variant()
                    )
                ),
            },
            _ => RErrExpected!(
                "(Bool Any Any)",
                format!(
                    "{} {}",
                    x.clone().variant(),
                    RVec(Arc::new(xs.to_vec())).variant()
                )
            ),
        }
    }

    pub fn is_function(&self, x: &RVal) -> RVal {
        match &x {
            _RSym(s) => {
                let v = self.symbols.get(&s.to_string());
                match v {
                    Some(f) => f.clone(),
                    None => RNil,
                }
            }
            _ => RNil,
        }
    }
}
