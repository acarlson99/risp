use crate::risp::{eval, REnv, RErr, RVal, RVal::*};

pub fn load_constructs(env: &mut REnv) {
    env.def("if", RBfn(cif));
}

fn cif(xs: &[RVal], env: &mut REnv) -> RVal {
   match xs.len() {
       2 => match eval(&xs[0], env) {
           RBool(b) => if b {
               eval(&xs[1], env)
           } else {
               RLstArgs![vec![]]
           }
           _ => RErrExpected!("(Bool Any)", RLstArgs![xs].variant()),
       }
       3 => match eval(&xs[0], env) {
           RBool(b) => if b {
               eval(&xs[1], env)
           } else {
               eval(&xs[2], env)
           }
           _ => RErrExpected!("(Bool Any Any)", RLstArgs![xs].variant()),
       }
       _ => RErrExpected!("(Bool Any Any) | (Bool Any)", RLstArgs![xs].variant()),
   }
}
