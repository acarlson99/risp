/******************************************************************************
** @crates and modules
******************************************************************************/

use fnv::FnvHashMap;

use std::fs;
use std::sync::Arc;

use crate::risp::{
    eval, eval_lambda, load_arithmetic, load_constructs, load_io, load_logic, rep, RErr, RLambda,
    RVal, RVal::*,
};

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
        load_io(&mut env);
        load_constructs(&mut env);
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
        S: Copy + Into<String>,
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
                "at" => self.builtin_at(xs),
                "head" => self.builtin_head(xs),
                "rest" => self.builtin_rest(xs),
                "do" => self.builtin_do(xs),
                "let" => self.builtin_def(xs),
                "if" => self.builtin_if(xs),
                "fn" => self.builtin_lfn(xs),
                "quote" => self.builtin_quote(xs),
                "eval" => self.builtin_eval(xs),
                "get" => self.builtin_get(xs),
                _ => RNil,
            },
            RLst(vs) => {
                if vs.is_empty() {
                    return RErrExpected!("Sym", x.clone().variant());
                }
                let new_val = eval(&x, self);
                match &new_val {
                    RBfn(f) => f(xs, self),
                    RLfn(lambda) => eval_lambda(lambda, &xs[..], self),
                    _ => RErrExpected!("(Sym)", x.clone().variant()),
                }
            }
            _ => RErrExpected!("(Sym)", x.clone().variant()),
        }
    }
    fn builtin_get(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            2 => match eval(&xs[1], self) {
                RMap(hm) => match hm.get(&eval(&xs[0], self)) {
                    Some(v) => v.clone(),
                    None => RLstArgs!(vec![]),
                },
                _ => RErrExpected!("(Any Map)", RLstArgs![xs].variant()),
            },
            _ => RErrExpected!("(Any Map)", RLstArgs![xs].variant()),
        }
    }
    fn builtin_at(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            2 => match (&xs[0], &eval(&xs[1], self)) {
                (RInt(i), RVec(vs)) => {
                    if (*i as usize) >= vs.len() {
                        RErr("index out of bounds")
                    } else {
                        vs[*i as usize].clone()
                    }
                }
                _ => RErrExpected!("Int (Vec)", RLstArgs![xs].variant()),
            },
            _ => RErrExpected!("Int (Vec)", RLstArgs![xs].variant()),
        }
    }
    fn builtin_head(&mut self, xs: &[RVal]) -> RVal {
        match eval(&xs[0], self) {
            RLst(vs) | RVec(vs) => {
                if vs.is_empty() {
                    RLstArgs!(vec![])
                } else {
                    eval(&vs[0], self)
                }
            }
            _ => RErrExpected!("(Lst | Vec)", RLstArgs![xs].variant()),
        }
    }
    fn builtin_rest(&mut self, xs: &[RVal]) -> RVal {
        match eval(&xs[0], self) {
            RLst(vs) => {
                if vs.len() < 2 {
                    RLstArgs!(vec![])
                } else {
                    RLstArgs!(&vs[1..])
                }
            }
            RVec(vs) => {
                if vs.len() < 2 {
                    RLstArgs!(vec![])
                } else {
                    eval(&RVecArgs!(&vs[1..]), self)
                }
            }
            _ => RErrExpected!("(Lst | Vec)", RLstArgs![xs].variant()),
        }
    }
    fn builtin_do(&mut self, xs: &[RVal]) -> RVal {
        let mut val = RNil;
        for v in xs[..].iter() {
            val = eval(&v, self);
            if let _RErr(_) = val.clone() {
                return val.clone();
            }
        }
        val
    }
    pub fn builtin_def(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            2 => match &xs[0] {
                _RSym(s) => {
                    let new_val = eval(&xs[1], self);
                    self.def(&s[..], new_val)
                }
                _ => RErrExpected!("(Sym Any)", RLstArgs![xs].variant()),
            },
            _ => RErrExpected!("(Sym Any)", RLstArgs![xs].variant()),
        }
    }
    fn builtin_if(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            0 => RErrExpected!("(Bool Any Any)", RLstArgs![xs].variant()),
            3 => match eval(&xs[0], self) {
                RBool(b) => {
                    let idx = if b { 0 } else { 1 };
                    eval(&xs[idx + 1], self)
                }
                _ => RErrExpected!("(Bool Any Any)", RLstArgs![xs].variant()),
            },
            _ => RErrExpected!("(Bool Any Any)", RLstArgs![xs].variant()),
        }
    }
    fn builtin_lfn(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            2 => match &xs[0] {
                RLst(ps) => {
                    if REnv::are_symbols(&ps[..]) {
                        RLfn(Arc::new(RLambda {
                            params: Arc::new(xs[0].clone()),
                            body: Arc::new(xs[1].clone()),
                        }))
                    } else {
                        RErr("parameters must be symbols")
                    }
                }
                _ => RErr("parameters must be in list form"),
            },
            _ => RErrExpected!("(parameters) body"),
        }
    }
    fn are_symbols(params: &[RVal]) -> bool {
        for v in params.iter() {
            match &v {
                _RSym(_) => (),
                _ => return false,
            }
        }
        true
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
    fn builtin_quote(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            1 => xs[0].clone(),
            _ => RErrExpected!("(Any)", RLstArgs!(xs).variant()),
        }
    }
    fn builtin_eval(&mut self, xs: &[RVal]) -> RVal {
        match xs.len() {
            1 => match &xs[0] {
                _RStr(s) => rep(&s[..], self),
                RLst(_) => {
                    let _x0 = rep(&xs[0].to_string(), self);
                    eval(&_x0, self)
                }
                _ => eval(&xs[0], self),
            },
            _ => RErrExpected!("(Any)", RLstArgs!(xs).variant()),
        }
    }
}

/******************************************************************************
** @repl io
******************************************************************************/

impl REnv {
    pub fn load<S>(&mut self, path: S) -> RVal
    where
        S: Into<String>,
    {
        let new_path = path.into();
        if let Ok(src) = fs::read_to_string(&new_path) {
            let new_src = vec!["(do\n", &src[..], ")"].join("");
            rep(new_src, self)
        } else {
            RErr(format!("could not load {}", &new_path))
        }
    }
}
