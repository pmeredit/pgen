use ast::*;
use mongo_config::*;
use std::collections::HashSet;

// TODO: use some sort of Cow schema so we can reuse memory that doesn't
// need to be changed: look into MaybeOwned
pub trait Normalize {
    fn normalize(self) -> Result<Box<Self>, String>;
}

impl Normalize for Expr {
    fn normalize(self) -> Result<Box<Self>, String> {
        use self::Expr::*;
        use mongo_config::Arity::*;
        match self {
            Op(e1, op, e2) => {
                                if op.is_assoc() {
                                    let (left_op, right_op)  = (e1.get_op(),     e2.get_op());
                                    let (left, right)        = (e1.normalize()?, e2.normalize()?);
                                    let mut args = Vec::new();
                                    if left_op == Some(op) {
                                        let left_args = left.take_args();
                                        args.extend(left_args.into_iter());
                                    } else {
                                        args.push(left);
                                    }
                                    if right_op == Some(op) {
                                        let right_args = right.take_args();
                                        args.extend(right_args.into_iter());
                                    } else {
                                        args.push(right);
                                    }
                                    Ok(Box::new(App(op.to_func(), args)))
                                }
                                else {
                                    let (left, right) = (e1.normalize()?, e2.normalize()?);
                                    Ok(Box::new(App(op.to_func(), vec![left,right])))
                                }
                              },
            Cond(c)        => Ok(Box::new(Cond(c.normalize()?))),
            Switch(sw)     => Ok(Box::new(Switch(sw.normalize()?))),
            Let(l)         => Ok(Box::new(Let(l.normalize()?))),
            Map(m)         => Ok(Box::new(Map(m.normalize()?))),
            Filter(f)      => Ok(Box::new(Filter(f.normalize()?))),
            Reduce(r)      => Ok(Box::new(Reduce(r.normalize()?))),
            Zip(z)         => Ok(Box::new(Zip(z.normalize()?))),
            Object(o)      => {
                                let o: Result<Vec<(String, Box<Expr>)>, String> = o.into_iter()
                                                                                   .map(|(k,v)| {Ok((k, v.normalize()?))})
                                                                                   .collect();
                                Ok(Box::new(Object(o?)))
                              },
            Array(a)       => {
                                let a: Result<Vec<Box<Expr>>, String> = a.into_iter()
                                                                         .map(|e| {e.normalize()})
                                                                         .collect();
                                Ok(Box::new(Array(a?)))
                              },
            App(s, args)   => {
                                if !FUNCTIONS.contains_key(s.as_str()) {
                                    return Err(format!("'{}' is not a known Mongo function", s));
                                }
                                let args: Result<Vec<Box<Expr>>, String> = args.into_iter()
                                                                               .map(|e| {e.normalize()})
                                                                               .collect();
                                let args     = args?;
                                let args_len = args.len();
                                match FUNCTIONS[s.as_str()].arity {
                                  Fixed(i) => if args_len != i {
                                                  return Err(format!("'{}' takes exactly {} arguments, but {} provided", s, i, args_len))
                                              },
                                  Variadic(i) => if args_len < i {
                                                  return Err(format!("'{}' takes at least {} arguments, but {} provided", s, i, args_len))
                                              },
                                  // Eventually we need to pass down if we are in a match context
                                  Match(min,max) | Optional(min,max) => if args_len < min || args_len > max {
                                                  return Err(format!("'{}' takes between {} and {} arguments, but {} provided", s, min, max, args_len))
                                              },
                                }
                                Ok(Box::new(App(s, args)))
                              },
            _ => Ok(Box::new(self)),
        }
    }
}

impl Normalize for Pipeline {
    fn normalize(self) -> Result<Box<Self>, String> {
        let mut stages = Vec::new();
        for (i, PipelineItem{stage_name, object}) in self.stages.into_iter().enumerate() {
            if !STAGES.contains(stage_name.as_str()) {
                return Err(
                    format!("{} is not a valid mongo stage", stage_name));
            }
            if let Expr::Object(_) = *object {
                let fvs = object.free_variables();
                if fvs.len() != 0 {
                    return Err(
                    format!("Stage: {} with Type: '{}' contains undefined variables: {:?}", 
                                    i, stage_name, fvs));
                }
                stages.push(PipelineItem{stage_name, object: object.normalize()?});    
            } 
            else {
                let variant = object.variant();
                let article = if set!['I', 'A', 'O']
                                         .contains(&variant.chars().nth(0).unwrap()) 
                                  { "an" } else { "a" };
                return Err(
                        format!("Stage must contain an Object not {} {}", article, variant)
                );
            }
        }
        Ok(Box::new(Pipeline{stages: stages}))
    }
}

// Do not check free variables here, those will be checked
// at the top level
impl Normalize for Let {
    fn normalize(self) -> Result<Box<Self>, String> {
        let mut assignments = Vec::new();
        for (s, e) in self.assignments.into_iter() {
            assignments.push((s, e.normalize()?));
        }
        Ok(Box::new(Let{assignments: assignments,
                        expr: self.expr.normalize()?}))
    }
}

impl Normalize for Map {
    fn normalize(self) -> Result<Box<Self>, String> {
        Ok(Box::new(Map{input: self.input.normalize()?,
                        ename: self.ename,
                        expr:  self.expr.normalize()?}))
    }
}

impl Normalize for Filter {
    fn normalize(self) -> Result<Box<Self>, String> {
        Ok(Box::new(Filter{input: self.input.normalize()?,
                           ename: self.ename,
                           cond:  self.cond.normalize()?}))
    }
}

impl Normalize for Reduce {
    fn normalize(self) -> Result<Box<Self>, String> {
        Ok(Box::new(Reduce{input: self.input.normalize()?,
                           init:  self.init.normalize()? ,
                           expr:  self.expr.normalize()?  }))
    }
}

impl Normalize for Zip {
    fn normalize(self) -> Result<Box<Self>, String> {
        let inputs   = self.inputs.normalize()?;
        let defaults = match self.defaults {
            None    => None,
            Some(d) => Some(d.normalize()?)
        };
        match *inputs {
            Expr::Array(ref v) => {
                match defaults {
                    None => (),
                    Some(ref d) => {
                        match d.as_ref() {
                            &Expr::Array(ref a) => {
                                if v.len() != a.len() {
                                    return Err(format!("Inputs and defaults for zip must have same size, sizes are {} and {} respectively", v.len(), a.len()))
                                }
                            },
                            _ => return Err(format!("Defaults for zip must be an array, not {:?}", defaults))
                        }
                    }
                }
            },
            _ => return Err(format!("Inputs to zip must be an array, not {}", inputs.variant()))
        };
        Ok(Box::new(Zip{inputs,
                        longest: self.longest,
                        defaults }))
    }
}

impl Normalize for Cond {
    fn normalize(self) -> Result<Box<Self>, String> {
        Ok(Box::new(Cond{ cond:      self.cond.normalize()?,
                          then:      self.then.normalize()?,
                          otherwise: self.otherwise.normalize()?,
                        }
                    )
           )
    }
}

impl Normalize for Switch {
    fn normalize(self) -> Result<Box<Self>, String> {
        let mut cases = Vec::new();
        for (cond, expr) in self.cases.into_iter() {
            cases.push((cond.normalize()?, expr.normalize()?));
        }
        Ok(Box::new(Switch{cases: cases,
                        default: self.default.normalize()?}))
    }
}
