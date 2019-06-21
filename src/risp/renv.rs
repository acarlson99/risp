/******************************************************************************
** @crates and modules
******************************************************************************/

use fnv::FnvHashMap;

use std::sync::Arc;

use crate::risp::{eval, load_arithmetic, load_logic, RErr, RVal, RVal::*, RLambda};

/******************************************************************************
** @environment
******************************************************************************/

#[derive(Clone)]
pub struct REnv {
    pub symbols: FnvHashMap<String, RVal>,
    pub parent: Option<Arc<REnv>>,
}

impl REnv {
    pub fn new() -> Self {
        let mut env = REnv {
            symbols: FnvHashMap::default(),
            parent: None,
        };
        load_arithmetic(&mut env);
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
    pub fn get<S>(&self, key: S) -> Option<RVal>
    where
        S: Copy + Into<String>
    {
        match self.symbols.get(&key.into()[..]) {
            Some(v) => Some(v.clone()),
            None => {
                match &self.parent {
                    Some(parent) => parent.get(&key.into()[..]),
                    None => None,
                }
            },
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
                "def" => self.builtin_def(&xs[..]),
                "if" => self.builtin_if(&xs[0], &xs[1..]),  // TODO: fix segv
                "fn" => self.builtin_lfn(&xs[..]),
                _ => RNil,
            },
            _ => RErrExpected!("Sym", x.clone().variant()),
        }
    }
    pub fn builtin_def(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            2 => match &xs[0] {
                _RSym(s) => {
                    let new_val = eval(&xs[1], self);
                    self.def(&s[..], new_val)
                },
                _ => RErrExpected!("(Sym Any)", RVecArgs!(xs).variant()),
            },
            _ => RErrExpected!("(Sym Any)", RVecArgs!(xs).variant()),
        }
    }
    // TODO: fix possible segfault here
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
    fn builtin_lfn(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            2 => match (&xs[0], &xs[1]) {
                (RVec(ps), RVec(bs)) => {
                    if REnv::are_symbols(&ps[..]) {
                        RLfn(Arc::new(RLambda {
                            params: Arc::new(xs[0].clone()),
                            body: Arc::new(xs[1].clone()),
                        }))
                    } else {
                        RErr("parameters must be symbols")
                    }
                },
                _=> RErr("parameters and body must be in list form"),
            }
            _ => RErrExpected!("(parameters) (body)")
        }
    }
    fn are_symbols(params: &[RVal]) -> bool {
        for v in params.iter() {
            match &v {
                _RSym(_) => (),
                _ => return false,
            }
        }
        return true;
    }

    pub fn is_function(&self, x: &RVal) -> RVal {
        match &x {
            _RSym(s) => {
                let v = self.symbols.get(&s.to_string());
                match v {
                    Some(v) => v.clone(),
                    None => RNil,
                }
            }
            _ => RNil,
        }
    }

}
