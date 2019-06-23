/******************************************************************************
** @crates and modules
******************************************************************************/

use std::ops::{Add, Div, Mul, Rem, Sub};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

use crate::risp::{eval, REnv, RErr, RVal, RVal::*};

/******************************************************************************
** @arithmetic operators
******************************************************************************/

// operations for floats and integers
macro_rules! rval_impl_op {
    ($tname: ty, $op: ident, $cop: ident, $msg: expr) => {
        impl $tname for RVal {
            type Output = RVal;
            fn $op(self, other: Self) -> Self {
                use RVal::*;
                match (&self, &other) {
                    (RFlt(a), RFlt(b)) => RFlt(a.$op(b)),
                    (RFlt(a), RInt(b)) => RFlt(a.$op(*b as f64)),
                    (RInt(a), RFlt(b)) => RFlt((*a as f64).$op(b)),
                    (RInt(a), RInt(b)) => match a.$cop(*b) {
                        Some(c) => RInt(c),
                        None => RErr($msg),
                    },
                    _ => RErrExpected!(
                        "(Num Num)",
                        format!("({} {})", self.clone().variant(), other.clone().variant())
                    ),
                }
            }
        }
    };
}
rval_impl_op! {Add, add, checked_add, "arithmetic overflow"}
rval_impl_op! {Sub, sub, checked_sub, "arithmetic overflow"}
rval_impl_op! {Mul, mul, checked_mul, "arithmetic overflow"}
rval_impl_op! {Div, div, checked_div, "division by zero or arithmetic overflow"}

// operations for integers only
macro_rules! rval_impl_iop {
    ($tname: ty, $op: ident, $cop: ident, $msg: expr) => {
        impl $tname for RVal {
            type Output = RVal;
            fn $op(self, other: Self) -> Self {
                use RVal::*;
                match (&self, &other) {
                    (RInt(a), RInt(b)) => match a.$cop(*b) {
                        Some(c) => RInt(c),
                        None => RErr($msg),
                    },
                    _ => RErrExpected!(
                        "(Int Int)",
                        format!("({} {})", self.clone().variant(), other.clone().variant())
                    ),
                }
            }
        }
    };
}
rval_impl_iop! {Rem, rem, checked_rem, "division by zero or arithmetic overflow"}

// integer right and left shifts
macro_rules! rval_impl_ish {
    ($tname: ty, $op: ident, $cop: ident, $msg: expr) => {
        impl $tname for RVal {
            type Output = RVal;
            fn $op(self, other: Self) -> Self {
                use RVal::*;
                match (&self, &other) {
                    (RInt(a), RInt(b)) => match a.$cop(*b as u32) {
                        Some(c) => RInt(c),
                        None => RErr($msg),
                    },
                    _ => RErrExpected!(
                        "(Int Int)",
                        format!("({} {})", self.clone().variant(), other.clone().variant())
                    ),
                }
            }
        }
    };
}

/******************************************************************************
** @arithmetic operators into environment
******************************************************************************/

pub fn load_arithmetic(env: &mut REnv) {
    env.def("+", RBfn(add));
    env.def("/", RBfn(div));
    env.def("*", RBfn(mul));
    env.def("-", RBfn(sub));
    env.def("%", RBfn(rem));
    env.def("&", RBfn(bitand));
    env.def("|", RBfn(bitor));
    env.def("~", RBfn(not));
    env.def("^", RBfn(bitxor));
    env.def("<<", RBfn(shl));
    env.def(">>", RBfn(shr));
    env.def("floor", RBfn(floor));
}

macro_rules! rval_binop {
    ($op: ident, $env: ident) => {
        fn binop(a: RVal, b: &RVal, env: &mut REnv) -> RVal {
            eval(&a, env).$op(eval(b, env).clone())
        }
    };
}

macro_rules! rval_varop {
    ($op: ident, $env: ident, $args: ident, $arg0: expr, $arg1: expr) => {
        rval_binop! {$op, $env};
        if $args.len() > 1 {
            let res = $arg0.iter().fold($arg1, |acc, x| binop(acc, x, $env));
            match &res {
                _ => res.clone(),
            }
        } else {
            RErrExpected!("(Num Num ...)", RLstArgs!($args).variant())
        }
    };
}

macro_rules! rval_arithmetic {
    ($op: ident, $idx: expr, $acc: expr) => {
        fn $op(args: &[RVal], env: &mut REnv) -> RVal {
            rval_varop! {$op, env, args, args[$idx..], $acc}
        }
    };
    ($op: ident, $idx: expr) => {
        fn $op(args: &[RVal], env: &mut REnv) -> RVal {
            rval_varop! {$op, env, args, args[$idx..], args[0].clone()}
        }
    };
}

rval_arithmetic! {add, 0, RInt(0)}
rval_arithmetic! {div, 1}
rval_arithmetic! {mul, 0, RInt(1)}
rval_arithmetic! {sub, 1}
rval_arithmetic! {rem, 1}

rval_impl_ish! {Shl, shl, checked_shl, "arithmetic overflow"}
rval_impl_ish! {Shr, shr, checked_shr, "arithmetic overflow"}
rval_arithmetic! {shl, 1}
rval_arithmetic! {shr, 1}

macro_rules! rval_impl_bit {
    ($tname: ty, $op: ident, $msg: expr) => {
        impl $tname for RVal {
            type Output = RVal;
            fn $op(self, other: Self) -> Self {
                use RVal::*;
                match (&self, &other) {
                    (RInt(a), RInt(b)) => RInt((*a as u64).$op(*b as u64) as i64),
                    _ => RErrExpected!(
                        "(Int Int)",
                        format!("({} {})", self.clone().variant(), other.clone().variant())
                    ),
                }
            }
        }
    };
}

rval_impl_bit! {BitAnd, bitand, "arithmetic overflow"}
rval_impl_bit! {BitOr, bitor, "arithmetic overflow"}
rval_impl_bit! {BitXor, bitxor, "arithmetic overflow"}

rval_arithmetic! {bitand, 1}
rval_arithmetic! {bitor, 1}
rval_arithmetic! {bitxor, 1}

fn not(args: &[RVal], env: &mut REnv) -> RVal {
    if args.is_empty() {
        RErrExpected!("(Num ...)", RLstArgs!(args).variant())
    } else {
        let mut out = vec![];
        for v in args.iter() {
            match eval(&v, env) {
                RInt(i) => out.push(RInt(i.not())),
                _ => return RErrExpected!("(Num ...)", RLstArgs!(args).variant()),
            };
        }
        RLstArgs!(out)
    }
}

fn floor(args: &[RVal], env: &mut REnv) -> RVal {
    if args.is_empty() {
        RErrExpected!("(Num ...)", RLstArgs!(args).variant())
    } else {
        let mut out = vec![];
        for v in args.iter() {
            match eval(&v, env) {
                RInt(i) => out.push(RInt(i)),
                RFlt(f) => out.push(RInt(f.floor() as i64)),
                _ => return RErrExpected!("(Num ...)", RLstArgs!(args).variant()),
            };
        }
        RLstArgs!(out)
    }
}
