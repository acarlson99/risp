/******************************************************************************
** @crates and modules
******************************************************************************/

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};
use std::sync::Arc;

/******************************************************************************
** @base data types
******************************************************************************/

#[derive(Clone)]
pub enum RVal {
    _RErr(Arc<String>),
    _RStr(Arc<String>),
    _RSym(Arc<String>),
    RNil,
    RBool(bool),
    RFlt(f64),
    RInt(i64),
    RVec(Arc<Vec<RVal>>),
}

/******************************************************************************
** @strings and errors
******************************************************************************/

// types that require strings
macro_rules! rval_impl_s {
    ($rty: ident, $_rty: expr) => {
        #[allow(non_snake_case)]
        pub fn $rty<S>(s: S) -> RVal
        where
            S: Into<String>,
        {
            $_rty(std::sync::Arc::new(s.into()))
        }
    };
}
rval_impl_s! {RErr, RVal::_RErr}
rval_impl_s! {RStr, RVal::_RStr}
rval_impl_s! {RSym, RVal::_RSym}

// error messages
#[allow(non_snake_case)]
macro_rules! RErrUnexpected {
    ($unexpected: expr) => {
        RErr(format!("unexpected {}", $unexpected))
    };
}

#[allow(non_snake_case)]
macro_rules! RErrExpected {
    ($expected: expr, $received: expr) => {
        RErr(format!("expected {}, received {}", $expected, $received))
    };
}

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
rval_impl_ish! {Shl, shl, checked_shl, "arithmetic overflow"}
rval_impl_ish! {Shr, shr, checked_shr, "arithmetic overflow"}

/******************************************************************************
** @output
******************************************************************************/

impl fmt::Display for RVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RVal::*;
        let s = match self {
            _RErr(e) => format!("Err: {}", e),
            _RStr(s) => format!("\"{}\"", s),
            _RSym(s) => s.to_string(),
            RNil => "nil".to_string(),
            RBool(b) => b.to_string(),
            RFlt(f) => f.to_string(),
            RInt(i) => i.to_string(),
            RVec(vs) => {
                let xs: Vec<String> = vs.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(" "))
            }
        };
        write!(f, "{}", s)
    }
}

impl RVal {
    pub fn variant(&self) -> String {
        use RVal::*;
        match self {
            _RErr(_) => "Err".to_string(),
            _RStr(_) => "Str".to_string(),
            _RSym(_) => "Sym".to_string(),
            RNil => "Nil".to_string(),
            RBool(_) => "Bool".to_string(),
            RFlt(_) => "Flt".to_string(),
            RInt(_) => "Int".to_string(),
            RVec(vs) => {
                let xs: Vec<String> = vs.iter().map(|x| x.variant()).collect();
                format!("({})", xs.join(" "))
            }
        }
    }
}
