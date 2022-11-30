use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Op1 {
    Neg,
    Not,
}

#[derive(Clone, Debug)]
pub enum Op2 {
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
}

#[derive(Clone, Debug)]
pub enum Term {
    Int(i32),
    Bool(bool),
    Var(String),
    Op1(Op1, Rc<Term>),
    Op2(Op2, Rc<Term>, Rc<Term>),
    Fun(String, String, Rc<Term>),
    App(Rc<Term>, Rc<Term>),
    LetIn(String, Rc<Term>, Rc<Term>),
    Ifte(Rc<Term>, Rc<Term>, Rc<Term>),
}
