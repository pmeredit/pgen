use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Number(i32),
    ID(String),
    Col(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Let(Box<Let>),
    App(String, Vec<Box<Expr>>),
    Error,
}

pub struct Let {
    pub assignments: Vec<(String, Box<Expr>)>,
    pub expr: Box<Expr>,
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
        let _ = write!(fmt, "\nLet\n");
        for &(ref var, ref expr) in a {
            let _ = write!(fmt, "  Id({:?})={:?}\n", var, expr);
        }
        write!(fmt, "in\n  {:?}\n", e)
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            ID(ref s) => write!(fmt, "ID({:?})", s),
            Col(ref s) => write!(fmt, "Col({:?})", s),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Let(ref l) => write!(fmt, "{:?}", l),
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

