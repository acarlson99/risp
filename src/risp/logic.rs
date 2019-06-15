/******************************************************************************
** @logic
******************************************************************************/

use std::cmp::Ordering;
use std::sync::Arc;

use crate::risp::{RErr, RStr, RSym, RVal, RVal::*, REnv};

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
//    env.def("!=", RBfn(ne));
}

macro_rules! varlogic {
    ($fname: ident, $lop: ident) => {
        fn $fname(_x: &RVal, xs: &[RVal], env: &REnv) -> bool {
            let _x0 = env.get_value(_x);
            let x0 = match &_x0 {
                RNil => RNil,
                _ => _x0,
            };
            match xs.first() {
                Some(v) => {
                    x0.$lop(v) && $fname(v, &xs[1..], env)
                },
                None => true,
            }
        }
    };
}

fn eq(args: &[RVal], env: &REnv) -> RVal {
    varlogic!(vareq, eq);
    if args.len() > 1 {
        RBool(vareq(&args[0], &args[1..], env))
    } else {
        RErrExpected!("(Any Any ...)",
        RVec(Arc::new(args.to_vec())).variant())
        // TODO: vec macroooo
    }
}

