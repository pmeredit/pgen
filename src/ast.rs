use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Number(i32),
    ID(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Let(Box<Let>),
    Error,
}

#[derive(Debug)]
pub struct Let {
    bindings: Vec<(Box<Expr>, Box<Expr>)>,
    expr: Box<Expr>,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            ID(ref s) => write!(fmt, "{:?}", s),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Let(ref l) => write!(fmt, "{:?}", l),
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

