use crate::Op1;
use crate::Op2;
use crate::Term;
use core::panic;
use derivative::Derivative;
use im::vector;
use im::vector::Vector;

#[derive(Clone, Debug)]
pub enum Cmd {
    Int(i32),
    Bool(bool),
    Add,
    Sub,
    Mul,
    Div,
    Lte,
    Gte,
    Lt,
    Gt,
    Eq,
    Neq,
    And,
    Or,
    Neg,
    Not,
    Access(usize),
    Closure(Vector<Cmd>),
    Ifte(Vector<Cmd>, Vector<Cmd>),
    Let,
    EndLet,
    Apply,
    Return,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Clo(Vector<Cmd>, #[derivative(Debug = "ignore")] Vector<Value>),
}

impl Op1 {
    fn compile(&self) -> Cmd {
        match self {
            Op1::Neg => Cmd::Neg,
            Op1::Not => Cmd::Not,
        }
    }
}

impl Op2 {
    fn compile(&self) -> Cmd {
        match self {
            Op2::Add => Cmd::Add,
            Op2::Sub => Cmd::Sub,
            Op2::Mul => Cmd::Mul,
            Op2::Div => Cmd::Div,
            Op2::Lte => Cmd::Lte,
            Op2::Gte => Cmd::Gte,
            Op2::Lt => Cmd::Lt,
            Op2::Gt => Cmd::Gt,
            Op2::Eq => Cmd::Eq,
            Op2::Neq => Cmd::Neq,
            Op2::And => Cmd::And,
            Op2::Or => Cmd::Or,
        }
    }
}

impl Term {
    fn compile<'a>(&'a self, mut names: Vector<&'a String>) -> Vector<Cmd> {
        match self {
            Term::Int(i) => {
                vector![Cmd::Int(*i)]
            }
            Term::Bool(b) => {
                vector![Cmd::Bool(*b)]
            }
            Term::Var(x) => {
                let i = names.index_of(&x).unwrap();
                vector![Cmd::Access(i)]
            }
            Term::Op1(opr, m) => {
                let cmds = m.compile(names.clone());
                cmds + vector![opr.compile()]
            }
            Term::Op2(opr, m, n) => {
                let cmds1 = m.compile(names.clone());
                let cmds2 = n.compile(names.clone());
                cmds1 + cmds2 + vector![opr.compile()]
            }
            Term::Fun(f, x, m) => {
                names.push_front(x);
                names.push_front(f);
                let cmds = m.compile(names);
                vector![Cmd::Closure(cmds + vector![Cmd::Return])]
            }
            Term::App(m, n) => {
                let cmds1 = m.compile(names.clone());
                let cmds2 = n.compile(names.clone());
                cmds1 + cmds2 + vector![Cmd::Apply]
            }
            Term::LetIn(x, m, n) => {
                let cmds1 = m.compile(names.clone());
                names.push_front(x);
                let cmds2 = n.compile(names.clone());
                cmds1 + vector![Cmd::Let] + cmds2 + vector![Cmd::EndLet]
            }
            Term::Ifte(cond, m, n) => {
                let cmds_cond = cond.compile(names.clone());
                let cmds1 = m.compile(names.clone());
                let cmds2 = n.compile(names.clone());
                cmds_cond + vector![Cmd::Ifte(cmds1, cmds2)]
            }
        }
    }

    pub fn run(&self) -> Value {
        let cmds = self.compile(Vector::new());
        Self::secd(Vec::new(), Vector::new(), cmds)
    }

    fn secd(mut stack: Vec<Value>, mut env: Vector<Value>, mut cmds: Vector<Cmd>) -> Value {
        while let Some(cmd) = cmds.pop_front() {
            match cmd {
                Cmd::Int(i) => stack.push(Value::Int(i)),
                Cmd::Bool(b) => stack.push(Value::Bool(b)),
                Cmd::Add => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Int(i + j)),
                        _ => panic!("bad Add"),
                    }
                }
                Cmd::Sub => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Int(i - j)),
                        _ => panic!("bad Sub"),
                    }
                }
                Cmd::Mul => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Int(i * j)),
                        _ => panic!("bad Mul"),
                    }
                }
                Cmd::Div => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Int(i / j)),
                        _ => panic!("bad Div"),
                    }
                }
                Cmd::Lte => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Bool(i <= j)),
                        _ => panic!("bad Lte"),
                    }
                }
                Cmd::Gte => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Bool(i >= j)),
                        _ => panic!("bad Gte"),
                    }
                }
                Cmd::Lt => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Bool(i < j)),
                        _ => panic!("bad Lt"),
                    }
                }
                Cmd::Gt => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Bool(i > j)),
                        _ => panic!("bad Gt"),
                    }
                }
                Cmd::Eq => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Bool(i == j)),
                        _ => panic!("bad Eq"),
                    }
                }
                Cmd::Neq => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Int(j), Value::Int(i)) => stack.push(Value::Bool(i != j)),
                        _ => panic!("bad Neq"),
                    }
                }
                Cmd::And => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Bool(j), Value::Bool(i)) => stack.push(Value::Bool(i && j)),
                        _ => panic!("bad And"),
                    }
                }
                Cmd::Or => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match (v1, v2) {
                        (Value::Bool(j), Value::Bool(i)) => stack.push(Value::Bool(i || j)),
                        _ => panic!("bad Or"),
                    }
                }
                Cmd::Neg => {
                    let v = stack.pop().unwrap();
                    match v {
                        Value::Int(i) => stack.push(Value::Int(-i)),
                        _ => panic!("bad Neg"),
                    }
                }
                Cmd::Not => {
                    let v = stack.pop().unwrap();
                    match v {
                        Value::Bool(i) => stack.push(Value::Bool(!i)),
                        _ => panic!("bad Not"),
                    }
                }
                Cmd::Access(i) => stack.push(env[i].clone()),
                Cmd::Closure(c) => stack.push(Value::Clo(c, env.clone())),
                Cmd::Ifte(c1, c2) => {
                    let v = stack.pop().unwrap();
                    match v {
                        Value::Bool(true) => cmds = c1 + cmds,
                        Value::Bool(false) => cmds = c2 + cmds,
                        _ => panic!("bad Ifte"),
                    }
                }
                Cmd::Let => {
                    let v = stack.pop().unwrap();
                    env.push_front(v)
                }
                Cmd::EndLet => {
                    env.pop_front();
                }
                Cmd::Apply => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match v2.clone() {
                        Value::Clo(c, e) => {
                            stack.push(Value::Clo(cmds, env));
                            cmds = c;
                            env = e;
                            env.push_front(v1);
                            env.push_front(v2);
                        }
                        _ => panic!("bad Apply"),
                    }
                }
                Cmd::Return => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    match v2 {
                        Value::Clo(c, e) => {
                            stack.push(v1);
                            cmds = c;
                            env = e;
                        }
                        _ => panic!("bad Return"),
                    }
                }
            }
        }
        stack.pop().unwrap()
    }
}
