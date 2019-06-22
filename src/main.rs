use std::env;

#[macro_use]
extern crate lazy_static;
extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[macro_use]
mod risp;
use risp::{rep, REnv};

const REPL0: &str = include_str!("../.repl_logo");
const REPL1: &str = "# ";

// TODO: autocompletion
fn main() {
    let mut env = REnv::new();
    let mut rl = Editor::<()>::new();
    if let Some(path) = env::args().nth(1) {
        env.load(path);
    }
    match rl.load_history(".repl_history") {
        _ => (),
    }
    println!("{}", REPL0);
    println!("{}", env.load("stdlib/prelude.rs"));
    loop {
        let readline = rl.readline(REPL1);
        match readline {
            Ok(line) => {
                let linestr = line.as_str();
                rl.add_history_entry(linestr);
                rl.save_history(".repl_history").unwrap();
                if !line.is_empty() {
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
