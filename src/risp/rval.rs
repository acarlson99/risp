/******************************************************************************
** @crates and modules
******************************************************************************/

use std::fmt;
use std::sync::Arc;

use crate::risp::REnv;

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
    RBfn(fn(&[RVal], &mut REnv) -> RVal),
    RLfn(Arc<RLambda>),
}

#[derive(Clone)]
pub struct RLambda {
    pub params: Arc<RVal>,
    pub body: Arc<RVal>,
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

#[allow(non_snake_case)]
macro_rules! RErrUnboundSymbol {
    ($symbol: expr) => {
        RErr(format!("unbound symbol '{}'", $symbol))
    };
}

#[allow(non_snake_case)]
macro_rules! RVecArgs {
    ($args: expr) => {
        RVec(std::sync::Arc::new($args.to_vec()))
    };
}


/******************************************************************************
** @output
******************************************************************************/

impl fmt::Display for RVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RVal::*;
        let s = match self {
            _RErr(e) => format!("(Err: {})", e),
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
            RBfn(_) => "Builtin-Fn".to_string(),
            RLfn(l) => format!("(Fn {} {})", l.params, l.body),
        };
        write!(f, "{}", s)
    }
}

impl RVal {
    pub fn variant(&self) -> String {
        use RVal::*;
        match self {
            _RErr(_) => self.to_string(),
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
            RBfn(_) => "Builtin-Fn".to_string(),
            RLfn(_) => "Fn".to_string(),
        }
    }
}
