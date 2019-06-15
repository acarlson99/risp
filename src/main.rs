#[macro_use]
extern crate lazy_static;

#[macro_use]
mod risp;
use risp::{RErr, RStr, RSym, RVal, RVal::*, REnv, rep};

fn main() {
    let mut env = REnv::new();
    println!("{}", risp::rep("(def PI 3.14)", &mut env));
    println!("{}", risp::rep("(= PI 3.14)", &mut env));
}
