extern crate rustyline;
use rustyline::Editor;

use crate::risp::{eval, REnv, RErr, RStr, RVal, RVal::*};

pub fn load_io(env: &mut REnv) {
    env.def("read", RBfn(read));
    env.def("write", RBfn(write));
    env.def("load", RBfn(load));
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
        RErrExpected!("(Str)", RLstArgs![args].variant())
    }
}

fn write(args: &[RVal], env: &mut REnv) -> RVal {
    for v in args.iter() {
        match &v {
            _RStr(s) => print!("{}", s),
            _RSym(_) => {
                let new_v = eval(&v, env);
                match &new_v {
                    _RErr(_) => return RErrUnboundSymbol!(v),
                    _RStr(s) => print!("{}", s),
                    _ => print!("{}", new_v),
                }
            }
            _ => print!("{}", v),
        }
    }
    println!();
    RLstArgs![vec![]]
}

// TODO: fix. this works but always returns error in the end.
fn load(args: &[RVal], env: &mut REnv) -> RVal {
    match args.len() {
        1 => match &args[0] {
            _RStr(path) => env.load(&path[..]),
            _ => RErrExpected!("(Str)", RLstArgs![args].variant()),
        },
        _ => RErrExpected!("(Str)", RLstArgs![args].variant()),
    }
}
