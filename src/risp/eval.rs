/******************************************************************************
** @crates and modules
******************************************************************************/

use std::sync::Arc;

use crate::risp::{parse, tokenize, REnv, RErr, RStr, RSym, RVal, RVal::*};

/******************************************************************************
** @read-eval-print
******************************************************************************/

pub fn rep<S>(expr: S, env: &mut REnv) -> RVal
where
    S: Into<String>,
{
    let tokens = tokenize(expr.into());
    let parsed = parse(&tokens);
    match parsed {
        Ok(v) => eval(&v.0, env),
        Err(e) => e,
    }
}

/******************************************************************************
** @eval
******************************************************************************/

fn eval(val: &RVal, env: &mut REnv) -> RVal {
    match &val {
        _RSym(s) => {
            let _r = env
                .symbols
                .get(&s.to_string())
                .ok_or_else(|| RErrUnboundSymbol!(s))
                .map(|x| x.clone());
            match _r {
                Ok(v) => v,
                Err(e) => e,
            }
        }
        RVec(vs) => {
            if vs.len() < 2 {
                return RErrExpected!("(Any ...)", val.variant());
            }
            let x = &vs[0];
            let xs = &vs[1..];
            let is_builtin = env.try_builtin(x, xs);
            match is_builtin {
                RNil => match env.is_function(&x) {
                    RBfn(f) => {
                        // TODO: implement macro that adds nil
                        f(xs, env)
                    }
                    _ => RErrExpected!("Fn", x.variant()),
                },
                _ => is_builtin,
            }
        }
        _ => val.clone(),
    }
}
