/******************************************************************************
** @crates and modules
******************************************************************************/

use crate::risp::{parse, tokenize, REnv, RErr, RLambda, RVal, RVal::*};

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
            if s.starts_with(':') {
                return val.clone();
            }
            let _r = env
                .get(&s.to_string())
                .ok_or_else(|| RErrUnboundSymbol!(s))
                .map(|x| x.clone());
            match _r {
                Ok(v) => v,
                Err(e) => e,
            }
        }
        RLst(vs) => {
            if vs.is_empty() {
                return RLstArgs!(vec![]);
            }
            let x = &vs[0];
            let xs = &vs[1..];
            let is_builtin = env.try_builtin(x, xs);
            match is_builtin {
                RNil => match env.is_function(&x) {
                    RBfn(f) => f(xs, env),
                    RLfn(lambda) => eval_lambda(&lambda, xs, env),
                    _ => RErrExpected!("Fn", x.variant()),
                },
                _ => is_builtin,
            }
        }
        _ => val.clone(),
    }
}

pub fn eval_lambda(lambda: &RLambda, args: &[RVal], env: &REnv) -> RVal {
    if args.len() == lambda.params.len() {
        match &*lambda.params {
            RLst(vs) => {
                let mut new_env = env.clone();
                for (k, v) in vs.iter().zip(args.iter()) {
                    let new_val = eval(&v.clone(), &mut new_env);
                    match &k {
                        _RSym(s) => new_env.def(&s[..], new_val),
                        _ => return RErr("internal error (eval_lambda)"),
                    };
                }
                eval(&lambda.body, &mut new_env)
            }
            _ => RErr("internal error (eval_lambda)"),
        }
    } else {
        RErrExpected!(
            format!("{} arguments", lambda.params.len()),
            format!("{}", args.len())
        )
    }
}
