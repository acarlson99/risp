use std::env;

#[macro_use]
extern crate lazy_static;
extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[macro_use]
mod risp;
use risp::{rep, REnv, RVal, RVal::*};

const REPL0: &str = include_str!("../.repl_logo");
const REPL1: &str = "# ";

fn main() {
    let mut env = REnv::new();
    let mut rl = Editor::<()>::new();
    match rl.load_history(".repl_history") {
        _ => (),
    }
    env.load("stdlib/prelude.rs");
    //rep("(load stdlib/prelude.rs)", &mut env);
    if let Some(path) = env::args().nth(1) {
        env.load(path);
        return;
    }
    println!("{}", REPL0);
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

fn help(_: &[RVal], _: &mut REnv) -> RVal {
    println!(
        r#"################################################################################

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
  # bitwise: & | ~ ^
  # io: read write
  # logic: = != < <= > >=
  # while: (Bool Any)
  # for: (Sym Num Num Any)
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
  # load: load a module

################################################################################"#
    );
    RLstArgs![vec![]]
}
