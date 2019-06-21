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
pub struct REnv<'a> {
    pub symbols: FnvHashMap<String, RVal>,
    pub parent: Option<&'a REnv<'a>>,
}

impl REnv<'_> {
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

impl REnv<'_> {
    pub fn try_builtin(&mut self, x: &RVal, xs: &[RVal]) -> RVal {
        match x {
            _RSym(s) => match &s[..] {
                "def" => self.builtin_def(&xs[0], &xs[1..]),
                "if" => self.builtin_if(&xs[0], &xs[1..]),
                "fn" => self.builtin_lfn(&xs),
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
                        RVec(Arc::new(xs.to_vec())).variant()  // TODO: here and below
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
    fn builtin_lfn(&mut self, xs: &[RVal]) -> RVal {
        let params = xs.first()
            .ok_or_else(|| RErr("function parameters"));
        let body = xs.get(1)
            .ok_or_else(|| RErr("function body"));
        if xs.len() > 2 {
            return RErr(
            "fn construct only takes a list of parameters and a function body"); 
        }
        match (params, body) {
            (Ok(p), Ok(b)) => match (&p, &b) {
                (RVec(_), RVec(_)) => RLfn(Arc::new(RLambda {
                        params: Arc::new(p.clone()),
                        body: Arc::new(b.clone()),
                    })),
                _ => RErr("invalid fn construct"),
            }
            _ => RErr("invalid fn construct"),
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
