extern crate rustyline;
use rustyline::Editor;

use crate::risp::{REnv, eval, RErr, RVal, RStr, RVal::*};

pub fn load_io(env: &mut REnv) {
    env.def("read", RBfn(read));
    env.def("write", RBfn(print));
}

fn read(args: &[RVal], _env: &mut REnv) -> RVal {
    let mut rl = Editor::<()>::new();
    if args.is_empty() {
        match rl.readline("") {
            Ok(line) => RStr(line.as_str()),
            _ => RErr("could not read line"),
        }
    } else if args.len() == 1 {
        match &args[0] {
            _RStr(s) => match rl.readline(&s[..]) {
                Ok(line) => RStr(line.as_str()),
                _ => RErr("could not read line"),
            },
            _ => RErrExpected!("(Str)"),
        }
    } else {
        RErrExpected!("(Str)", RVecArgs![args].variant())
    }
}

fn print(args: &[RVal], env: &mut REnv) -> RVal {
    for v in args.iter() {
        match &v {
            _RStr(s) => print!("{}", s),
            _RSym(_) => match &eval(&v, env) {
                _RStr(s) => print!("{}", s),
                _ => return RErrUnboundSymbol!(v),
            },
            _ => print!("{}", v),
        }
    }
    println!("");
    RVecArgs![vec![]]
}
