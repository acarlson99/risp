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
                // TODO: fix segfaults
                "def" => self.builtin_def(&xs[0], &xs[1..]),
                "if" => self.builtin_if(&xs[0], &xs[1..]),
                "fn" => self.builtin_lfn(&xs[..]),
                _ => RNil,
            },
            _ => RErrExpected!("Sym", x.clone().variant()),
        }
    }
    // TODO: fix segfault when only def is passed
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
                        RVecArgs!(xs.to_vec()).variant()
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
                (RVec(ps), RVec(bs)) => RLfn(Arc::new(RLambda {
                    env: Arc::new(REnv {
                        symbols: FnvHashMap::default(),
                        parent: Some(Arc::new(self.clone())),
                    }),
                    params: Arc::new(xs[0].clone()),  // TODO: assure they are symbols
                    body: Arc::new(xs[1].clone()),
                })),
                _=> RErr("parameters and body must be in list form"),
            }
            _ => RErrExpected!("(parameters) (body)")
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

impl REnv {
    pub fn lambda_env(&mut self, params: &Arc<RVal>, args: &[RVal]) -> Result<REnv, RVal> {
        let ks = parse_syms(params.clone())?;
        if ks.len() != args.len() {
            return Err(RErrExpected!(format!("{} arguments", ks.len()), args.len()));
        }
        let vs = self.eval_syms(args);

        let mut symbols = FnvHashMap::default();
        for (k, v) in ks.iter().zip(vs.iter()) {
            symbols.insert(k.clone(), v.clone());
        }
        Ok( REnv {
            symbols: symbols,
            parent: Some(Arc::new(self.clone())),
        })

    }
    fn eval_syms(&mut self, args: &[RVal]) -> Vec<RVal> {
        args
            .iter()
            .map(|x| eval(x, self))
            .collect()
    }
}

fn parse_syms(params: Arc<RVal>) -> Result<Vec<String>, RVal> {
    match &*params {
        RVec(vs) => {
            vs
                .iter()
                .map(|x| {
                    match x {
                        _RSym(s) => Ok(s.to_string()),
                        _ => Err(RErr("TODO")),
                    }}).collect()
        },
        _ => Err(RErr("expected parameters to be in a list")),
    }
}
