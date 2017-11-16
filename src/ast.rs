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
pub struct Map{
    pub input: Box<Expr>, 
    pub ename: String, 
    pub expr:  Box<Expr>,
}

#[derive(Debug)] 
pub struct Filter{
    pub input: Box<Expr>, 
    pub ename: String, 
    pub cond:  Box<Expr>,
}

#[derive(Debug)] 
pub struct Zip{
    pub inputs:   Box<Expr>, 
    pub longest:  bool, 
    pub defaults: Option<Box<Expr>>,
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
    Null,
    Number(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    ID(String),
    Col(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Cond(Box<Cond>),
    Switch(Box<Switch>),
    Let(Box<Let>),
    Map(Box<Map>),
    Filter(Box<Filter>),
    Zip(Box<Zip>),
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
                    // Remove any assigned variables from free vars
                    for &(ref s, _) in &l.assignments {
                        ret.remove(s);
                    }
                },
                Map(ref m) => {
                    aux(&*m.input, &mut ret);
                    aux(&*m.expr,  &mut ret);
                    // Remove assigned name from free vars
                    ret.remove(&m.ename);
                }
                Filter(ref f) => {
                    aux(&*f.input, &mut ret);
                    aux(&*f.cond,  &mut ret);
                    // Remove assigned name from free vars
                    ret.remove(&f.ename);
                }
                Zip(ref z) => {
                    aux(&*z.inputs,   &mut ret);
                    match z.defaults.as_ref() {
                        Some(ref d) => aux(d, &mut ret),
                        None        => ()
                    };
                }
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

    pub fn variant(&self) -> &'static str {
        use self::Expr::*;
        match *self {
            ID(_)     => "Identifier",
            Op(_,_,_) => "Binary Operation",
            Cond(_)   => "If Expression",
            Switch(_) => "Switch", 
            Let(_)    => "Let Expression",
            Map(_)    => "Map Expression",
            Filter(_) => "Filter Expression",
            Zip(_) => "Zip Expression",
            App(_,_)  => "Function Application",
            Array(_)  => "Array",
            Object(_) => "Object",
            Number(_) => "Int",
            Float(_)  => "Float",
            Bool(_)   => "Bool",
            Null      => "Null",
            Str(_)    => "String",
            Col(_)    => "Column Reference",
            //Should never happen
            Error     => "Error",
        }
    }

    pub fn get_op(&self) -> Option<Opcode> {
        use self::Expr::*;
        match *self {
            Op(_,op,_) => Some(op),
            _          => None
        }
    }

    pub fn take_args(self) -> Vec<Box<Expr>> {
        match self {
            self::Expr::App(_, args) => args,
            _                        => Vec::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    Eq,
    Neq,
    Lte,
    Gte,
    Lt,
    Gt,
    And,
    Or,
    Exp,
}

impl Opcode {
    // Lower an opcode to the mongo agg function name
    pub fn to_func_str(&self) -> &'static str {
        use self::Opcode::*;
        match *self {
            Mul => "multiply",
            Div => "divide",
            Mod => "mod",
            Add => "add",
            Sub => "subtract",
            Eq  => "eq",
            Neq => "ne",
            Lte => "lte",
            Gte => "gte",
            Lt  => "lt",
            Gt  => "gt",
            And => "and",
            Or  => "or",
            Exp => "pow",
        }
    }

    pub fn to_func(&self) -> String {
        String::from(self.to_func_str())
    }

    pub fn is_assoc(&self) -> bool {
        use self::Opcode::*;
        match *self {
            Mul | Add | And | Or => true,
            _ => false
        }
    }
}
