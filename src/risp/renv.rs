/******************************************************************************
** @crates and modules
******************************************************************************/

use fnv::FnvHashMap;

use std::sync::Arc;

use crate::risp::{RErr, RStr, RSym, RVal, RVal::*};

/******************************************************************************
** @environment
******************************************************************************/

pub struct REnv {
    pub symbols: FnvHashMap<String, RVal>,
}

impl REnv {
    pub fn new() -> Self {
        REnv {
            symbols: FnvHashMap::default(),
        }
    }
    fn def<S>(&mut self, key: S, val: RVal) -> RVal
    where
        S: Into<String>,
    {
        self.symbols.insert(key.into(), val.clone());
        val
    }
}

/******************************************************************************
** @builtins
******************************************************************************/

impl REnv {
    // TODO: return nil when not native
    pub fn try_builtin(&mut self, x: &RVal, xs: &[RVal]) -> RVal {
        match x {
            _RSym(s) => match &s[..] {
                "def" => self.builtin_def(&xs[0], &xs[1..]),
                _ => RNil,
            },
            _ => RErrExpected!("TODO: BUILTIN", x.clone().variant()),
        }
    }
    fn builtin_def(&mut self, x: &RVal, xs: &[RVal]) -> RVal {
        match xs.len() {
            0 => RErrExpected!("(Sym Any)", x.clone().variant()),
            1 => match x {
                _RSym(s) => self.def(&s[..], xs[0].clone()),
                _ => RErrExpected!(
                    "(Sym Any)",
                    format!("({} {})", x.clone().variant(), RVec(Arc::new(xs.to_vec())).variant()))
            },
            _ => RErrExpected!(
                "(Sym Any)",
                    format!("({} {})", x.clone().variant(), RVec(Arc::new(xs.to_vec())).variant())),
        }
    }
}
