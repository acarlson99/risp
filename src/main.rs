#[macro_use]
extern crate lazy_static;
extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[macro_use]
mod risp;
use risp::{rep, REnv};

const REPL0: &str = include_str!("../res/repl_logo");
const REPL1: &str = "# ";

// TODO: autocompletion
fn main() {
    let mut env = REnv::new();
    let mut r1 = Editor::<()>::new();
    println!("{}", REPL0);
    loop {
        let readline = r1.readline(REPL1);
        match readline {
            Ok(line) => {
                let linestr = line.as_str();
                r1.add_history_entry(linestr);
                //r1.save_history("res/repl_history").unwrap();
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
