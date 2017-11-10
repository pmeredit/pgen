use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Number(i32),
    Bool(bool),
    ID(String),
    Col(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Cond(Box<Cond>),
    Let(Box<Let>),
    App(String, Vec<Box<Expr>>),
    Error,
}

pub struct Let {
    pub assignments: Vec<(String, Box<Expr>)>,
    pub expr: Box<Expr>,
}

pub struct Cond {
    pub cond: Box<Expr>,
    pub then: Box<Expr>,
    pub otherwise: Box<Expr>,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Let {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let Let{assignments: ref a, expr: ref e} = *self;
        let _ = write!(fmt, "\nlet\n");
        for &(ref var, ref expr) in a {
            let _ = write!(fmt, "  ID({:?})={:?}\n", var, expr);
        }
        write!(fmt, "in\n  {:?}\n", e)
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Bool(b) => write!(fmt, "{:?}", b),
            ID(ref s) => write!(fmt, "ID({:?})", s),
            Col(ref s) => write!(fmt, "Col({:?})", s),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Let(ref l) => write!(fmt, "{:?}", l),
            Cond(ref c) => write!(fmt, "\nif {:?}\nthen {:?}\nelse {:?}\n", c.cond, c.then, c.otherwise),
            App(ref s, ref v) => write!(fmt, "App({:?}({:?}))", s, v),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

