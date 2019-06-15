#[macro_use]
extern crate lazy_static;

#[macro_use]
mod risp;
use risp::{RErr, RStr, RSym, RVal, RVal::*, REnv, rep};

fn main() {
    let mut env = REnv::new();
    println!("{}", risp::rep("PI", &mut env))
}
