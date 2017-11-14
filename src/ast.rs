use std::collections::HashSet;

#[derive(Debug)]
pub struct Pipeline {
    pub stages: Vec<PipelineItem>,
}

#[derive(Debug)]
pub struct PipelineItem {
    pub stage_name: String,
    // Expr is more general than
    // Object, we will catch in normalize
    // rather than the parser, and give a better
    // error message
    pub object: Box<Expr>,
}

#[derive(Debug)]
pub struct Let {
    pub assignments: Vec<(String, Box<Expr>)>,
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Cond {
    pub cond: Box<Expr>,
    pub then: Box<Expr>,
    pub otherwise: Box<Expr>,
}

#[derive(Debug)]
pub struct Switch {
    pub cases: Vec<(Box<Expr>, Box<Expr>)>,
    pub default: Box<Expr>,
}

#[derive(Debug)]
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

impl Expr {
    //TODO: likely someday we will want to replace String with 
    //a reference equal pointer "atom"
    pub fn free_variables(&self) -> HashSet<String> {
        use self::Expr::*;
        let mut ret = HashSet::new();
        fn aux(e: &Expr, mut ret: &mut HashSet<String>) {
            match *e {
                ID(ref s) => {
                    ret.insert(s.clone());
                }
                Op(ref e1, _, ref e2) => { 
                    aux(&*e1, &mut ret); 
                    aux(&*e2, &mut ret); 
                },
                Cond(ref c) => { 
                    aux(&*c.cond,      &mut ret); 
                    aux(&*c.then,      &mut ret); 
                    aux(&*c.otherwise, &mut ret);
                },
                Switch(ref sw) => { 
                    aux(&*sw.default, &mut ret);
                    for &(ref cond, ref expr) in &sw.cases {
                        aux(&*cond, &mut ret);
                        aux(&*expr, &mut ret);
                    }
                },
                Let(ref l) => {
                    aux(&*l.expr, &mut ret);
                    for &(_, ref expr) in &l.assignments {
                        aux(&*expr, &mut ret);
                    }
                    for &(ref s, _) in &l.assignments {
                        ret.remove(s);
                    }
                },
                App(_, ref args) => {
                    for ref e in args {
                        aux(&*e, &mut ret)
                    }
                },
                Array(ref exprs) => {
                    for ref e in exprs {
                        aux(&*e, &mut ret)
                    }
                },
                Object(ref pairs) => {
                    for &(_, ref e) in pairs {
                        aux(&*e, &mut ret)
                    }
                },
                _ => (),
            }
        }
        aux(self, &mut ret);
        ret
    }
}

#[derive(Copy, Clone, Debug)]
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
    Exp,
}
