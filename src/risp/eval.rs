/******************************************************************************
** @crates and modules
******************************************************************************/

use crate::risp::{parse, tokenize, REnv, RErr, RVal, RVal::*};

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

pub fn eval(val: &RVal, env: &mut REnv) -> RVal {
    match &val {
        _RSym(s) => {
            let _r = env
                .get(&s.to_string())
                .ok_or_else(|| RErrUnboundSymbol!(s))
                .map(|x| x.clone());
            match _r {
                Ok(v) => v,
                Err(e) => e,
            }
        }
        RVec(vs) => {
            if vs.is_empty() {
                return RNil;
            }
            let x = &vs[0];
            let xs = &vs[1..];
            let is_builtin = env.try_builtin(x, xs);
            match is_builtin {
                RNil => match env.is_function(&x) {
                    RBfn(f) => {
                        f(xs, env)
                    },
                    RLfn(lambda) => {
                        let new_env = &mut env.lambda_env(lambda.params, &xs);
                        match &new_env {
                            Ok(v) => {
                              RErr("TODO: LAMBDA")  
                            },
                            Err(e) => e.clone(),
                        }
                        //RErr("TODO: lambda 2")
                        //eval(&lambda.body, &lenv)
                    }
                    _ => RErrExpected!("Fn", x.variant()),
                },
                _ => is_builtin,
            }
        }
//        RLfn(_) => RErrUnexpected!("Fn"),  // TODO: is this needed?
        _ => val.clone(),
    }
}
