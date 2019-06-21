/******************************************************************************
** @crates and modules
******************************************************************************/

use crate::risp::{parse, tokenize, REnv, RErr, RVal, RVal::*, RLambda};

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
            let xs = &vs[1..];  // TODO: fix, this could segv
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

fn eval_lambda(lambda: &RLambda, args: &[RVal], env: &REnv) -> RVal {
    if args.len() == lambda.params.len() {
        match &*lambda.params {
            RVec(vs) => {
                let mut new_env = env.clone();
                for (k, v) in vs.iter().zip(args.iter()) {
                    match &k {
                        _RSym(s) => new_env.def(&s[..], v.clone()),
                        _ => return RErr("internal error (eval_lambda)"),
                    };
                }
                eval(&lambda.body, &mut new_env)
            },
            _ => RErr("internal error (eval_lambda)"),
        }
        /*
        let new_env = env.clone();
        for (k, v) in lambda.params.iter().zip(args.iter()) {
            new_env.def(k, v);
        }*/
    } else {
        RErrExpected!(
            format!("{} arguments", lambda.params.len()),
            format!("{}", args.len()))
    }
}
