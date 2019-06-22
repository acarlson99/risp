/******************************************************************************
** @crates and modules
******************************************************************************/

use fnv::FnvHashMap;

use std::fs;
use std::sync::Arc;

use crate::risp::{eval, rep, eval_lambda, load_arithmetic, load_logic, RErr, RVal, RVal::*, RLambda};

/******************************************************************************
** @environment
******************************************************************************/

#[derive(Clone)]
pub struct REnv {
    pub symbols: FnvHashMap<String, RVal>,
}

impl REnv {
    pub fn new() -> Self {
        let mut env = REnv {
            symbols: FnvHashMap::default(),
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
            None => None,
        }
    }
}

/******************************************************************************
** @builtins
******************************************************************************/

impl REnv {
    pub fn try_builtin(&mut self, x: &RVal, xs: &[RVal]) -> RVal {
        match &x {
            _RSym(s) => match &s[..] {
                "do" => self.builtin_do(&xs[..]),
                "let" => self.builtin_def(&xs[..]),
                "if" => self.builtin_if(&xs[0], &xs[1..]),  // TODO: fix segv
                "fn" => self.builtin_lfn(&xs[..]),
                "load" => self.builtin_load(&xs[..]),
                "mod" => self.builtin_mod(&xs[..]),
                _ => RNil,
            },
            RVec(vs) => {
                if vs.is_empty() {
                    return RErrExpected!("Sym", x.clone().variant())
                }
                let new_val = eval(&x, self);
                match &new_val {
                    RLfn(lambda) => eval_lambda(lambda, &xs[..], self),
                    _ => RErrExpected!("Sym", x.clone().variant()),
                }
            }
            _ => RErrExpected!("Sym", x.clone().variant()),
        }
    }
    fn builtin_do(&mut self, xs: &[RVal]) -> RVal {
        let mut val = RNil;
        for v in xs[..].iter() {
            val = eval(&v, self);
            println!("{}", &val);
        }
        val
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
            2 => match &xs[0] {
                RVec(ps) => {
                    if REnv::are_symbols(&ps[..]) {
                        RLfn(Arc::new(RLambda {
                            params: Arc::new(xs[0].clone()),
                            body: Arc::new(xs[1].clone()),
                        }))
                    } else {
                        RErr("parameters must be symbols")
                    }
                },
                _=> RErr("parameters must be in list form"),
            }
            _ => RErrExpected!("(parameters) body")
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

    fn builtin_load(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            1 => match &xs[0] {
                _RStr(path) => self.load(&path[..]),
                _ => RErrExpected!("(Str)", RVecArgs!(xs).variant()),
            },
            _ => RErrExpected!("(Str)", RVecArgs!(xs).variant()),
        }
    }
    fn builtin_mod(&mut self, xs: &[RVal]) -> RVal {
        if xs.len() >= 2 {
            for v in xs[1..].iter() {
                eval(v, self);
            }
            RNil
        } else {
            RErr("invalid module")
        }
    }
}

/******************************************************************************
** @repl io
******************************************************************************/

impl REnv {
    pub fn load<S>(&mut self, path: S) -> RVal
        where
        S: Into<String>
        {
            let new_path = path.into();
            if let Ok(src) = fs::read_to_string(&new_path) {
                rep(src, self)
            } else {
                RErr(format!("could not load {}", &new_path))
            }
        }
}
