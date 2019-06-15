#[macro_use]
extern crate lazy_static;

#[macro_use]
mod risp;
use risp::{RErr, RStr, RSym, RVal, RVal::*};

fn main() {
    println!("{}", risp::rep("(3.14 56)"))
}
