/******************************************************************************
** @crates and modules
******************************************************************************/

use crate::risp::{
    RVal, RErr, RStr, RSym, RVal::*,
    parse, tokenize,
};

/******************************************************************************
** @read-eval-print
******************************************************************************/

// TODO: add environment and eval
pub fn rep<S>(expr: S) -> RVal where S: Into<String> {
    let tokens = tokenize(expr.into());
    let parsed = parse(&tokens);
    match parsed {
        Ok(v) => v.0,
        Err(e) => e,
    }
}