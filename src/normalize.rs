use ast::*;
use mongo_config::*;

// TODO: use some sort of Cow schema when there is no change
/*
pub trait Normalize {
    fn normalize(self) -> Result<Box<Self>, &'static str>;
}

impl Normalize for Expr {
    fn normalize(self) -> Result<Box<Expr>, &'static str> {
        match self {
            _ => Ok(Box::new(self)),
        }
    }
}

impl Normalize for Pipeline {
    fn normalize(self) -> Result<Box<Pipeline>, &'static str> {
        let mut stages = Vec::new();
        for (i, PipelineItem{stage_name: stage_name, object: stage}) in self.stages.into_iter().enumerate() {
            if !STAGES.contains(&stage_name) {
                return Err(
                    format_args!("{} is not a valid mongo stage", stage_name));
            }
            match stage {
                Expr::Object(obj) => {
                    let fvs = obj.free_variables();
                    if obj.fvs.len() != 0 {
                        return Err(
                            format_args!("Stage {} with type {} contains undefined variables {:?}", 
                                         i, stage_name, fvs));
                    }
                    stages.push((stage_name, stage.normalize()?));
                }
            }
        }
    }
}

// Do not check free variables here, those will be checked
// at the top level
impl Normalize for Let {
    fn normalize(self) -> Result<Box<Let>, &'static str> {
        let mut assignments = Vec::new();
        for (s, e) in self.assignments.into_iter() {
            assignments.push((s, e.normalize()?));
        }
        Ok(Box::new(Let{assignments: assignments,
                        expr: self.expr.normalize()?}))
    }
}
*/
