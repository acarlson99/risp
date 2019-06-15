#[macro_use]
mod risp;
use risp::{RErr, RStr, RSym, RVal, RVal::*};

fn main() {
    println!("{}", RFlt(3.69) + RFlt(7.28));
}
