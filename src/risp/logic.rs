/******************************************************************************
** @logic
******************************************************************************/

use std::cmp::Ordering;

use crate::risp::{eval, REnv, RErr, RVal, RVal::*};

/******************************************************************************
** @logical operators
******************************************************************************/

impl PartialEq for RVal {
    fn eq(&self, other: &Self) -> bool {
        use RVal::*;
        match (self, other) {
            (_RErr(a), _RErr(b)) => a.eq(b),
            (_RStr(a), _RStr(b)) => a.eq(b),
            (_RSym(a), _RSym(b)) => a.eq(b),
            (RNil, RNil) => true,
            (RBool(a), RBool(b)) => a.eq(b),
            (RFlt(a), RFlt(b)) => a.eq(b),
            (RFlt(a), RInt(b)) => a.eq(&(*b as f64)),
            (RInt(a), RFlt(b)) => (*a as f64).eq(b),
            (RInt(a), RInt(b)) => a.eq(b),
            (RVec(a), RVec(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl PartialOrd for RVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use RVal::*;
        match (self, other) {
            (_RStr(a), _RStr(b)) => a.partial_cmp(b),
            (RFlt(a), RFlt(b)) => a.partial_cmp(b),
            (RFlt(a), RInt(b)) => a.partial_cmp(&(*b as f64)),
            (RInt(a), RFlt(b)) => (*a as f64).partial_cmp(b),
            (RInt(a), RInt(b)) => Some(a.cmp(b)),
            (RVec(a), RVec(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

/******************************************************************************
** @logical operators into environment
******************************************************************************/

pub fn load_logic(env: &mut REnv) {
    env.def("=", RBfn(eq));
    env.def("!=", RBfn(ne));
    env.def("<", RBfn(lt));
    env.def("<=", RBfn(le));
    env.def(">", RBfn(gt));
    env.def(">=", RBfn(ge));
}

// TODO: implement max and min in arithmetic module
// find a way to report an unbound symbol
// perhaps get_value could return an error, and we set to an external var
macro_rules! rval_logic {
    ($lop: ident) => {
        fn $lop(args: &[RVal], env: &mut REnv) -> RVal {
            fn varlop(res: &mut RVal, env: &mut REnv, x0: &RVal, xs: &[RVal]) -> bool {
                match xs.first() {
                    Some(ref _v) => {
                        let v = eval(_v, env);
                        eval(x0, env).$lop(&v) && varlop(res, env, &v, &xs[1..])
                    }
                    None => true,
                }
            }
            if args.len() > 1 {
                let mut res = RNil;
                let b = varlop(&mut res, env, &args[0], &args[1..]);
                match &res {
                    RNil => RBool(b),
                    _ => res.clone(),
                }
            } else {
                RErrExpected!("(T T ...)", "TODO: VEC MACRO")
            }
        }
    };
}

rval_logic! {eq}
rval_logic! {ne}
rval_logic! {lt}
rval_logic! {le}
rval_logic! {gt}
rval_logic! {ge}
