use crate::Op1;
use crate::Op2;
use crate::Term;
use im::hashmap::HashMap;

#[derive(Clone, Debug)]
pub enum Value<'a> {
    Int(i32),
    Bool(bool),
    Clo(
        &'a String,
        &'a String,
        &'a Term,
        HashMap<&'a String, Value<'a>>,
    ),
}

impl Term {
    pub fn eval<'a>(&'a self, mut env: HashMap<&'a String, Value<'a>>) -> Value<'a> {
        match self {
            Term::Int(i) => Value::Int(*i),
            Term::Bool(b) => Value::Bool(*b),
            Term::Op1(op, m) => {
                let v = m.eval(env);
                match (op, v) {
                    (Op1::Neg, Value::Int(i)) => Value::Int(-i),
                    (Op1::Not, Value::Bool(i)) => Value::Bool(!i),
                    _ => panic!("bad Op1"),
                }
            }
            Term::Op2(op, m1, m2) => {
                let v1 = m1.eval(env.clone());
                let v2 = m2.eval(env.clone());
                match (op, v1, v2) {
                    (Op2::Add, Value::Int(i), Value::Int(j)) => Value::Int(i + j),
                    (Op2::Sub, Value::Int(i), Value::Int(j)) => Value::Int(i - j),
                    (Op2::Mul, Value::Int(i), Value::Int(j)) => Value::Int(i * j),
                    (Op2::Div, Value::Int(i), Value::Int(j)) => Value::Int(i / j),
                    (Op2::Lte, Value::Int(i), Value::Int(j)) => Value::Bool(i <= j),
                    (Op2::Gte, Value::Int(i), Value::Int(j)) => Value::Bool(i >= j),
                    (Op2::Lt, Value::Int(i), Value::Int(j)) => Value::Bool(i < j),
                    (Op2::Gt, Value::Int(i), Value::Int(j)) => Value::Bool(i > j),
                    (Op2::Eq, Value::Int(i), Value::Int(j)) => Value::Bool(i == j),
                    (Op2::Neq, Value::Int(i), Value::Int(j)) => Value::Bool(i != j),
                    (Op2::And, Value::Bool(i), Value::Bool(j)) => Value::Bool(i && j),
                    (Op2::Or, Value::Bool(i), Value::Bool(j)) => Value::Bool(i || j),
                    _ => panic!("bad Op2"),
                }
            }
            Term::Var(x) => env.remove(x).unwrap(),
            Term::Fun(f, x, m) => Value::Clo(f, x, m, env),
            Term::App(m1, m2) => {
                let v1 = m1.eval(env.clone());
                let v2 = m2.eval(env.clone());
                match v1.clone() {
                    Value::Clo(f, x, m, mut env0) => {
                        env0.insert(f, v1);
                        env0.insert(x, v2);
                        m.eval(env0)
                    }
                    _ => panic!("bad App"),
                }
            }
            Term::LetIn(x, m1, m2) => {
                let v = m1.eval(env.clone());
                env.insert(x, v);
                m2.eval(env)
            }
            Term::Ifte(cond, m1, m2) => {
                let v = cond.eval(env.clone());
                match v {
                    Value::Bool(t) => {
                        if t {
                            m1.eval(env)
                        } else {
                            m2.eval(env)
                        }
                    }
                    _ => panic!("bad Ifte"),
                }
            }
        }
    }
}
