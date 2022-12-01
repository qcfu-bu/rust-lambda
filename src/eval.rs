use crate::Op1;
use crate::Op2;
use crate::Term;
use core::panic;
use derivative::Derivative;
use im::vector;
use im::vector::Vector;

#[derive(Clone, Debug)]
pub enum CMD {
    INT(i32),
    BOOL(bool),
    ADD,
    SUB,
    MUL,
    DIV,
    LTE,
    GTE,
    LT,
    GT,
    EQ,
    NEQ,
    AND,
    OR,
    NEG,
    NOT,
    ACCESS(usize),
    CLOSURE(Vector<CMD>),
    IFTE(Vector<CMD>, Vector<CMD>),
    LET,
    ENDLET,
    APPLY,
    RETURN,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Clo(Vector<CMD>, #[derivative(Debug = "ignore")] Vector<Value>),
}

impl Op1 {
    fn compile(&self) -> CMD {
        match self {
            Op1::Neg => CMD::NEG,
            Op1::Not => CMD::NOT,
        }
    }
}

impl Op2 {
    fn compile(&self) -> CMD {
        match self {
            Op2::Add => CMD::ADD,
            Op2::Sub => CMD::SUB,
            Op2::Mul => CMD::MUL,
            Op2::Div => CMD::DIV,
            Op2::Lte => CMD::LTE,
            Op2::Gte => CMD::GTE,
            Op2::Lt => CMD::LT,
            Op2::Gt => CMD::GT,
            Op2::Eq => CMD::EQ,
            Op2::Neq => CMD::NEQ,
            Op2::And => CMD::AND,
            Op2::Or => CMD::OR,
        }
    }
}

impl Term {
    pub fn compile<'a>(&'a self, mut names: Vector<&'a String>) -> Vector<CMD> {
        match self {
            Term::Int(i) => {
                vector![CMD::INT(*i)]
            }
            Term::Bool(b) => {
                vector![CMD::BOOL(*b)]
            }
            Term::Var(x) => {
                let i = names.index_of(&x).unwrap();
                vector![CMD::ACCESS(i)]
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
                vector![CMD::CLOSURE(cmds + vector![CMD::RETURN])]
            }
            Term::App(m, n) => {
                let cmds1 = m.compile(names.clone());
                let cmds2 = n.compile(names.clone());
                cmds1 + cmds2 + vector![CMD::APPLY]
            }
            Term::LetIn(x, m, n) => {
                let cmds1 = m.compile(names.clone());
                names.push_front(x);
                let cmds2 = n.compile(names.clone());
                cmds1 + vector![CMD::LET] + cmds2 + vector![CMD::ENDLET]
            }
            Term::Ifte(cond, m, n) => {
                let cmds_cond = cond.compile(names.clone());
                let cmds1 = m.compile(names.clone());
                let cmds2 = n.compile(names.clone());
                cmds_cond + vector![CMD::IFTE(cmds1, cmds2)]
            }
        }
    }

    pub fn run(&self) -> Value {
        let cmds = self.compile(Vector::new());
        Self::secd(Vector::new(), Vector::new(), cmds)
    }

    fn secd(mut stack: Vector<Value>, mut env: Vector<Value>, mut cmds: Vector<CMD>) -> Value {
        loop {
            match cmds.pop_front() {
                Some(cmd) => match cmd {
                    CMD::INT(i) => stack.push_front(Value::Int(i)),
                    CMD::BOOL(b) => stack.push_front(Value::Bool(b)),
                    CMD::ADD => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Int(i + j)),
                            _ => panic!("bad ADD"),
                        }
                    }
                    CMD::SUB => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Int(i - j)),
                            _ => panic!("bad SUB"),
                        }
                    }
                    CMD::MUL => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Int(i * j)),
                            _ => panic!("bad MUL"),
                        }
                    }
                    CMD::DIV => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Int(i / j)),
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::LTE => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Bool(i <= j)),
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::GTE => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Bool(i >= j)),
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::LT => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Bool(i < j)),
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::GT => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Bool(i > j)),
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::EQ => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Bool(i == j)),
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::NEQ => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Int(j), Value::Int(i)) => stack.push_front(Value::Bool(i != j)),
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::AND => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Bool(j), Value::Bool(i)) => {
                                stack.push_front(Value::Bool(i && j))
                            }
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::OR => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match (v1, v2) {
                            (Value::Bool(j), Value::Bool(i)) => {
                                stack.push_front(Value::Bool(i || j))
                            }
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::NEG => {
                        let v = stack.pop_front().unwrap();
                        match v {
                            Value::Int(i) => stack.push_front(Value::Int(-i)),
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::NOT => {
                        let v = stack.pop_front().unwrap();
                        match v {
                            Value::Bool(i) => stack.push_front(Value::Bool(!i)),
                            _ => panic!("bad DIV"),
                        }
                    }
                    CMD::ACCESS(i) => stack.push_front(env[i].clone()),
                    CMD::CLOSURE(c) => stack.push_front(Value::Clo(c, env.clone())),
                    CMD::IFTE(c1, c2) => {
                        let v = stack.pop_front().unwrap();
                        match v {
                            Value::Bool(true) => cmds = c1 + cmds,
                            Value::Bool(false) => cmds = c2 + cmds,
                            _ => panic!("bad IFTE"),
                        }
                    }
                    CMD::LET => {
                        let v = stack.pop_front().unwrap();
                        env.push_front(v)
                    }
                    CMD::ENDLET => {
                        env.pop_front();
                    }
                    CMD::APPLY => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match v2.clone() {
                            Value::Clo(c, e) => {
                                stack.push_front(Value::Clo(cmds, env));
                                cmds = c;
                                env = e;
                                env.push_front(v1);
                                env.push_front(v2);
                            }
                            _ => panic!("bad APPLY"),
                        }
                    }
                    CMD::RETURN => {
                        let v1 = stack.pop_front().unwrap();
                        let v2 = stack.pop_front().unwrap();
                        match v2 {
                            Value::Clo(c, e) => {
                                stack.push_front(v1);
                                cmds = c;
                                env = e;
                            }
                            _ => panic!("bad RETURN"),
                        }
                    }
                },
                None => break,
            }
        }
        stack.pop_front().unwrap()
    }
}
