use std::env;

#[macro_use]
extern crate lazy_static;
extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[macro_use]
mod risp;
use risp::{rep, REnv, RErr, RVal, RVal::*};

const REPL0: &str = include_str!("../.repl_logo");
const REPL1: &str = "# ";

// TODO: autocompletion
fn main() {
    let mut env = REnv::new();
    let mut rl = Editor::<()>::new();
    match rl.load_history(".repl_history") {
        _ => (),
    }
    println!("{}", REPL0);
    env.load("stdlib/prelude.rs");
    if let Some(path) = env::args().nth(1) {
        env.load(path);
    }
    // TODO: fix module support
    env.def("load", RBfn(fake_load));
    env.def("help", RBfn(help));
    loop {
        let readline = rl.readline(REPL1);
        match readline {
            Ok(line) => {
                let linestr = line.as_str();
                rl.add_history_entry(linestr);
                rl.save_history(".repl_history").unwrap();
                if !line.is_empty() && !line.starts_with(';') {
                    println!("{}", rep(line, &mut env));
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn fake_load(_: &[RVal], _: &mut REnv) -> RVal {
    RErr("loading modules from the REPL is not currently supported")
}

fn help(_: &[RVal], _: &mut REnv) -> RVal {
    println!(r#"################################################################################

# data types:
  # Str: "Hello, World!"
  # Sym: hello-world
  # Bool: true | false
  # Flt: 42.42
  # Int: 42
  # Lst: (1 2 3 4)
  # Vec: [1 2 3 4]
  # Map: {{:one 1 :two 2}}
  # Fn: (fn (x y) (+ x y))

# builtin functions:
  # arithmetic: / * - +
  # io: read write
  # logic: = != < <= > >=

# constructs:
  # at: get the nth element of a Vec
  # car: get the first element of a Lst
  # cdr: get the second element of a Lst
  # do: evaluate several expressions in sequence, return last one
  # let: bind a value to a name
  # if: check a condition, return first expression if true, else the second
  # mod: create a module
  # quote: return a value without evaluating it first
  # eval: evaluate a string or a list
  # get: get an element from a Map using a key

################################################################################"#);
    RLstArgs![vec![]]
}

