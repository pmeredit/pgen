use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Number(i64),
    Float(f64),
    Bool(bool),
    Null,
    Str(String),
    ID(String),
    Col(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Cond(Box<Cond>),
    Switch(Box<Switch>),
    Let(Box<Let>),
    App(String, Vec<Box<Expr>>),
    Array(Vec<Box<Expr>>),
    Object(Vec<(String, Box<Expr>)>),
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

pub struct Switch {
    pub cases: Vec<(Box<Expr>, Box<Expr>)>,
    pub default: Box<Expr>,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Eq,
    Neq,
    Lte,
    Gte,
    And,
    Or,
}

impl Debug for Let {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let Let{assignments: ref a, expr: ref e} = *self;
        let _ = write!(fmt, "\nlet\n");
        for &(ref var, ref expr) in a {
            let _ = write!(fmt, "  {:?}={:?}\n", var, expr);
        }
        write!(fmt, "in\n  {:?}\n", e)
    }
}

impl Debug for Switch {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let Switch{cases: ref cs, default: ref d} = *self;
        let _ = write!(fmt, "\nswitch{{\n");
        for &(ref cond, ref then) in cs {
            let _ = write!(fmt, "  {:?} => {:?}\n", cond, then);
        }
        write!(fmt, "  default: {:?}\n}}", d)
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n)            => write!(fmt, "{:?}", n),
            Float(f)             => write!(fmt, "{:?}", f),
            Null                 => write!(fmt, "<null>"),
            Str(ref s)           => write!(fmt, "Str({:?})", s),
            Bool(b)              => write!(fmt, "{:?}", b),
            ID(ref s)            => write!(fmt, "ID({:?})", s),
            Col(ref s)           => write!(fmt, "Col({:?})", s),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Let(ref l)           => write!(fmt, "{:?}", l),
            Cond(ref c)          => write!(fmt, "\nif {:?}\nthen {:?}\nelse {:?}\n", c.cond, c.then, c.otherwise),
            Switch(ref s)        => write!(fmt, "{:?}", s), 
            App(ref s, ref v)    => write!(fmt, "App({:?}({:?}))", s, v),
            Array(ref v)         => write!(fmt, "{:?}", v),
            Object(ref v)        => format_obj(fmt, v),
            Error                => write!(fmt, "error"),
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
            Eq  => write!(fmt, "=="),
            Neq => write!(fmt, "!="),
            Lte => write!(fmt, "<="),
            Gte => write!(fmt, ">="),
            And => write!(fmt, "and"),
            Or  => write!(fmt, "or"),
        }
    }
}

fn format_obj(fmt: &mut Formatter, obj: &Vec<(String, Box<Expr>)>) -> Result<(), Error> {
    let _ = write!(fmt, "{{");
    for &(ref key, ref val) in obj {
        let _ = write!(fmt, "{:?}: {:?}", key, val);
    }
    write!(fmt, "}}")
}

