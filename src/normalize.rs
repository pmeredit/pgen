use ast::*;
use mongo_config::*;
use std::collections::HashSet;

// TODO: use some sort of Cow schema so we can reuse memory that doesn't
// need to be changed: look into MaybeOwned
//
pub trait Normalize {
    fn normalize(self) -> Result<Box<Self>, String>;
}

pub trait AggNormalize {
    fn agg_normalize(self) -> Result<Box<Self>, String>;
}

pub trait MatchNormalize {
    fn match_normalize(self) -> Result<Box<Self>, String>;
}

impl AggNormalize for Expr {
    fn agg_normalize(self) -> Result<Box<Self>, String> {
        use self::Expr::*;
        use mongo_config::Arity::*;
        match self {
            Op(e1, op, e2) => {
                                if op.is_assoc() {
                                    let (left_op, right_op)  = (e1.get_op(),     e2.get_op());
                                    let (left, right)        = (e1.agg_normalize()?, e2.agg_normalize()?);
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
                                    let (left, right) = (e1.agg_normalize()?, e2.agg_normalize()?);
                                    Ok(Box::new(App(op.to_func(), vec![left,right])))
                                }
                              },
            Cond(c)        => Ok(Box::new(Cond(c.agg_normalize()?))),
            Switch(sw)     => Ok(Box::new(Switch(sw.agg_normalize()?))),
            Let(l)         => Ok(Box::new(Let(l.agg_normalize()?))),
            Map(m)         => Ok(Box::new(Map(m.agg_normalize()?))),
            Filter(f)      => Ok(Box::new(Filter(f.agg_normalize()?))),
            Reduce(r)      => Ok(Box::new(Reduce(r.agg_normalize()?))),
            Zip(z)         => Ok(Box::new(Zip(z.agg_normalize()?))),
            Object(o)      => {
                                let o: Result<Vec<(String, Box<Expr>)>, String> = o.into_iter()
                                                                                   .map(|(k,v)| {Ok((k, v.agg_normalize()?))})
                                                                                   .collect();
                                Ok(Box::new(Object(o?)))
                              },
            Array(a)       => {
                                let a: Result<Vec<Box<Expr>>, String> = a.into_iter()
                                                                         .map(|e| {e.agg_normalize()})
                                                                         .collect();
                                Ok(Box::new(Array(a?)))
                              },
            App(s, args)   => {
                                if !AGG_FUNCTIONS.contains_key(s.as_str()) {
                                    return Err(format!("'{}' is not a known Mongo function", s));
                                }
                                let args: Result<Vec<Box<Expr>>, String> = args.into_iter()
                                                                               .map(|e| {e.agg_normalize()})
                                                                               .collect();
                                let args     = args?;
                                let args_len = args.len();
                                match AGG_FUNCTIONS[s.as_str()].arity {
                                  Fixed(i) => if args_len != i {
                                                  return Err(format!("'{}' takes exactly {} arguments, but {} provided", s, i, args_len))
                                              },
                                  Variadic(i) => if args_len < i {
                                                  return Err(format!("'{}' takes at least {} arguments, but {} provided", s, i, args_len))
                                              },
                                  Optional(min,max) => if args_len < min || args_len > max {
                                                  return Err(format!("'{}' takes between {} and {} arguments, but {} provided", s, min, max, args_len))
                                              },
                                }
                                Ok(Box::new(App(s, args)))
                              },
            _ => Ok(Box::new(self)),
        }
    }
}

fn check_vars(stage: &Expr, i: usize, stage_name: &str) -> Result<(),String> {
    let fvs = stage.free_variables();
    if fvs.len() != 0 {
        return Err(
        format!("Stage: {} with Type: '{}' contains undefined variables: {:?}", 
                        i, stage_name, fvs));
    }
    Ok(())
}

impl Normalize for Pipeline {
    fn normalize(self) -> Result<Box<Self>, String> {
        let mut stages = Vec::new();
        for (i, PipelineItem{stage_name, stage}) in self.stages.into_iter().enumerate() {
            if !STAGES.contains(stage_name.as_str()) {
                return Err(
                    format!("{} is not a valid mongo stage", stage_name));
            }
            if stage_name == "match" {
                check_vars(&*stage, i, &stage_name)?;
                if let Expr::Object(_) = *stage {
                    stages.push(PipelineItem{stage_name, stage: stage.match_normalize()?});
                } else {
                    stages.push(PipelineItem{stage_name, stage: 
                        Box::new(Expr::App("expr".to_string(), vec![stage.agg_normalize()?]))});
                }
            }
            else if let Expr::Object(_) = *stage {
                check_vars(&*stage, i, &stage_name)?;
                stages.push(PipelineItem{stage_name, stage: stage.agg_normalize()?});    
            } 
            else {
                let variant = stage.variant();
                let article = if set!['I', 'A', 'O']
                                         .contains(&variant.chars().nth(0).unwrap()) 
                                  { "an" } else { "a" };
                return Err(
                        format!("Stage: {} with Type: '{} must contain an Object not {} {}", i, stage_name, article, variant)
                );
            }
        }
        Ok(Box::new(Pipeline{stages: stages}))
    }
}

// Do not check free variables here, those will be checked
// at the top level
impl AggNormalize for Let {
    fn agg_normalize(self) -> Result<Box<Self>, String> {
        let mut assignments = Vec::new();
        for (s, e) in self.assignments.into_iter() {
            assignments.push((s, e.agg_normalize()?));
        }
        Ok(Box::new(Let{assignments: assignments,
                        expr: self.expr.agg_normalize()?}))
    }
}

impl AggNormalize for Map {
    fn agg_normalize(self) -> Result<Box<Self>, String> {
        Ok(Box::new(Map{input: self.input.agg_normalize()?,
                        ename: self.ename,
                        expr:  self.expr.agg_normalize()?}))
    }
}

impl AggNormalize for Filter {
    fn agg_normalize(self) -> Result<Box<Self>, String> {
        Ok(Box::new(Filter{input: self.input.agg_normalize()?,
                           ename: self.ename,
                           cond:  self.cond.agg_normalize()?}))
    }
}

impl AggNormalize for Reduce {
    fn agg_normalize(self) -> Result<Box<Self>, String> {
        Ok(Box::new(Reduce{input: self.input.agg_normalize()?,
                           init:  self.init.agg_normalize()? ,
                           expr:  self.expr.agg_normalize()?  }))
    }
}

impl AggNormalize for Zip {
    fn agg_normalize(self) -> Result<Box<Self>, String> {
        let inputs   = self.inputs.agg_normalize()?;
        let defaults = match self.defaults {
            None    => None,
            Some(d) => Some(d.agg_normalize()?)
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

impl AggNormalize for Cond {
    fn agg_normalize(self) -> Result<Box<Self>, String> {
        Ok(Box::new(Cond{ cond:      self.cond.agg_normalize()?,
                          then:      self.then.agg_normalize()?,
                          otherwise: self.otherwise.agg_normalize()?,
                        }
                    )
           )
    }
}

impl AggNormalize for Switch {
    fn agg_normalize(self) -> Result<Box<Self>, String> {
        let mut cases = Vec::new();
        for (cond, expr) in self.cases.into_iter() {
            cases.push((cond.agg_normalize()?, expr.agg_normalize()?));
        }
        Ok(Box::new(Switch{cases: cases,
                        default: self.default.agg_normalize()?}))
    }
}

////////////////////////////
// Match Normalize
////////////////////////////
impl MatchNormalize for Expr {
    fn match_normalize(self) -> Result<Box<Self>, String> {
        use self::Expr::*;
        use mongo_config::Arity::*;
        match self {
            Op(e1, op, e2) => {
                                let func = op.to_func();
                                if !MATCH_FUNCTIONS.contains_key(func.as_str()) {
                                    return Err(format!("'{:?}' is not a valid Mongo match operator", op));
                                }
                                if op.is_assoc() {
                                    let (left_op, right_op)  = (e1.get_op(),     e2.get_op());
                                    let (left, right)        = (e1.match_normalize()?, e2.match_normalize()?);
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
                                    let (left, right) = (e1.match_normalize()?, e2.match_normalize()?);
                                    Ok(Box::new(App(op.to_func(), vec![left,right])))
                                }
                              },
            Cond(_)        => Err("Cannot use $cond in $match".to_string()),
            Switch(_)      => Err("Cannot use $switch in $match".to_string()),
            Let(_)         => Err("Cannot use $let in $match".to_string()),
            Map(_)         => Err("Cannot use $map in $match".to_string()),
            Filter(_)      => Err("Cannot use $filter in $match".to_string()),
            Reduce(_)      => Err("Cannot use $reduce in $match".to_string()),
            Zip(_)         => Err("Cannot use $zip in $match".to_string()),
            Object(o)      => {
                                let o: Result<Vec<(String, Box<Expr>)>, String> = o.into_iter()
                                                                                   .map(|(k,v)| {Ok((k, v.match_normalize()?))})
                                                                                   .collect();
                                Ok(Box::new(Object(o?)))
                              },
            Array(a)       => {
                                let a: Result<Vec<Box<Expr>>, String> = a.into_iter()
                                                                         .map(|e| {e.match_normalize()})
                                                                         .collect();
                                Ok(Box::new(Array(a?)))
                              },
            App(s, args)   => {
                                if !MATCH_FUNCTIONS.contains_key(s.as_str()) {
                                    return Err(format!("'{}' is not a known Mongo match function", s));
                                }
                                if s == "expr" {
                                    let args: Result<Vec<Box<Expr>>, String> = args.into_iter()
                                                                                   .map(|e| {e.agg_normalize()})
                                                                                   .collect();
                                    return Ok(Box::new(App(s, args?)));
                                }
                                let args: Result<Vec<Box<Expr>>, String> = args.into_iter()
                                                                               .map(|e| {e.match_normalize()})
                                                                               .collect();
                                let args     = args?;
                                let args_len = args.len();
                                match MATCH_FUNCTIONS[s.as_str()].arity {
                                  Fixed(i) => if args_len != i {
                                                  return Err(format!("'{}' takes exactly {} arguments, but {} provided", s, i, args_len))
                                              },
                                  Variadic(i) => if args_len < i {
                                                  return Err(format!("'{}' takes at least {} arguments, but {} provided", s, i, args_len))
                                              },
                                  Optional(min,max) => if args_len < min || args_len > max {
                                                  return Err(format!("'{}' takes between {} and {} arguments, but {} provided", s, min, max, args_len))
                                              },
                                }
                                Ok(Box::new(App(s, args)))
                              },
            _ => Ok(Box::new(self)),
        }
    }
}
