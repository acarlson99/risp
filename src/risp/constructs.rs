use crate::risp::{eval, REnv, RErr, RVal, RVal::*};

pub fn load_constructs(env: &mut REnv) {
    env.def("cons", RBfn(ccons));
    env.def("cond", RBfn(ccond));
    env.def("if", RBfn(cif));
    env.def("for", RBfn(cfor));
    env.def("while", RBfn(cwhile));
}

fn ccons(xs: &[RVal], _: &mut REnv) -> RVal {
    match xs.len() {
        2 => RLstArgs![xs],
        _ => RErrExpected!["(Any Any)", RLstArgs![xs].variant()]
    }
}

fn ccond(xs: &[RVal], env: &mut REnv) -> RVal {
    if xs.is_empty() {
        RErrExpected!("((Bool Any) ...)", RLstArgs![xs].variant())
    } else {
        for pair in xs.iter() {
            let tmp = match &pair {
                RLst(vs) => if vs.len() == 2 {
                    if let RBool(b) = eval(&vs[0], env) {
                        if b {
                            return eval(&vs[1], env);
                        } else {
                            RLstArgs![vec![]]
                        }
                    } else {
                        RErrExpected!("(Bool Any)", pair.variant())
                    }
                } else {
                    RErrExpected!("(Bool Any)", pair.variant())
                }
                _ => RErrExpected!("(Bool Any)", pair.variant())
            };
            if tmp != RLstArgs![vec![]] {
                return tmp.clone();
            }
        }
        RLstArgs![vec![]]
    }
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

fn cfor(xs: &[RVal], env: &mut REnv) -> RVal {
    match xs.len() {
        4 => match (&xs[0], eval(&xs[1], env), eval(&xs[2], env)) {
            (_RSym(s), RInt(_from), RInt(_to)) => {
                let from = if _from < _to { _from } else { _to };
                let to = if _to > _from { _to } else { _from };
                let past = env.get(&s[..]);
                let mut out = RLstArgs![vec![]];
                for it in from..to {
                    env.def(&s[..], RInt(it));
                    out = eval(&xs[3], env);
                }
                env.restore(&s[..], past);
                out
            }
            _ => RErrExpected!("(Sym Int Int Any)", RLstArgs![xs].variant()),
        }
        _ => RErrExpected!("(Sym Int Int Any)", RLstArgs![xs].variant()),
    }
}

fn cwhile(xs: &[RVal], env: &mut REnv) -> RVal {
    match xs.len() {
        2 => match eval(&xs[0], env) {
            RBool(b) => {
                let mut out = RLstArgs![vec![]];
                let mut cond = b;
                while cond {
                    out = eval(&xs[1], env);
                    if let RBool(new_b) = eval(&xs[0], env) {
                        cond = new_b;
                    } else {
                        return RErrExpected!("(Bool Any)", RLstArgs![xs].variant());
                    }
                }
                out
            },
            _ => RErrExpected!("(Bool Any)", RLstArgs![xs].variant()),
        }
        _ => RErrExpected!("(Bool Any)", RLstArgs![xs].variant()),
    }
}
