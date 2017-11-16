use ast::*;
use linked_hash_map::LinkedHashMap;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonType {
    I(i64),
    F(f64),
    B(bool),
    S(String),
    // Any recursive JsonTypes must be Options to support null
    A(Vec<Option<Box<JsonType>>>),
    O(LinkedHashMap<String, Option<Box<JsonType>>>),
}

pub trait Convert<T> {
    fn convert(self) -> T;
}

impl Convert<Option<Box<JsonType>>> for Expr {
     fn convert(self) -> Option<Box<JsonType>> {
          use self::JsonType::*;
          match self {
              Expr::Null          => None,
              Expr::Number(n)     => obox!(I(n)),
              Expr::Float(f)      => obox!(F(f)),
              Expr::Bool(b)       => obox!(B(b)),
              Expr::Str(s)        => obox!(S(s)),
              // Variables introduced by let should have $$ prefixed
              Expr::ID(s)         => obox!(S("$$".to_string() + &s)),
              // Cols still have $ prefixed, whereas IDs did not
              Expr::Col(s)        => obox!(S(s)),
              Expr::Cond(c)       => c.convert(),
              Expr::Switch(sw )   => sw.convert(),
              Expr::Let(l)        => l.convert(),
              Expr::Map(m)        => m.convert(),
              Expr::App(s,args)   => {
                 obox!(O(linked_hash_map![
                                            "$".to_string() + &s => 
                                             if args.len() > 1 { args.convert() } 
                                             else              { args.into_iter().nth(0).convert() }
                                         ]
                 ))
              },
              Expr::Array(arr)    => arr.convert(),
              Expr::Object(items) => {
                 let ret: LinkedHashMap<String, Option<Box<JsonType>>> = items.
                                                                         into_iter().
                                                                         map(|(k,v)| {
                                                                            (k, v.convert()) 
                                                                         }).
                                                                         collect();
                 obox!(O(ret))
              },
              Expr::Op(_,_,_) => panic!("Should not be converting Op, make sure to normalize first"),
              Expr::Error => panic!("Should not be converting Error!"),
          }
     }
}

// Essentially remove a layer of Option for Some.  Sadly
// Option Map can't handle this
impl Convert<Option<Box<JsonType>>> for Option<Box<Expr>> {
    fn convert(self) -> Option<Box<JsonType>> {
        match self {
            None       => None,
            Some(expr) => expr.convert()
        }
    }
}

impl Convert<Option<Box<JsonType>>> for Vec<Box<Expr>> {
    fn convert(self) -> Option<Box<JsonType>> {
        use self::JsonType::*;
        let ret: Vec<Option<Box<JsonType>>> = self
                                                  .into_iter()
                                                  .map(|e| {
                                                              e.convert()  
                                                           }
                                                      )
                                                  .collect();
        obox!(A(ret))
    }
}


impl Convert<Option<Box<JsonType>>> for Pipeline {
    fn convert(self) -> Option<Box<JsonType>> {
        use self::JsonType::*;
        let ret: Vec<Option<Box<JsonType>>> = self.stages
                                                  .into_iter()
                                                  .map(|PipelineItem{stage_name, object}| {
                                                              obox!(O(
                                                                      linked_hash_map![stage_name => object.convert()]
                                                                      ))
                                                           }
                                                      )
                                                  .collect();
        obox!(A(ret))
    }
}

//
//{ 
//   $cond: { 
//        if: <boolean-expression>, 
//        then: <true-case>, 
//        else: <false-case-> 
//        } 
//}
//
impl Convert<Option<Box<JsonType>>> for Cond {
    fn convert(self) -> Option<Box<JsonType>> {
        use self::JsonType::*;
        obox!(O(
                linked_hash_map![
                    "$cond".to_string() => 
                          obox!(O(linked_hash_map![
                                      "if".to_string() => self.cond.convert(),
                                      "then".to_string() => self.then.convert(),
                                      "else".to_string() => self.otherwise.convert()
                                  ]
                                  ))
                ]
                ))
    }
}

//{
//   $switch: {
//      branches: [
//         { case: <expression>, then: <expression> },
//         { case: <expression>, then: <expression> },
//         ...
//      ],
//      default: <expression>
//   }
//}
//
impl Convert<Option<Box<JsonType>>> for Switch {
    fn convert(self) -> Option<Box<JsonType>> {
        use self::JsonType::*;
        let branches: Vec<Option<Box<JsonType>>> = self.cases
                                                       .into_iter()
                                                       .map(|(i,t)| {
                                                           obox!(O(
                                                                   linked_hash_map![
                                                                    "case".to_string() => i.convert(), 
                                                                    "then".to_string() => t.convert() 
                                                                   ]))
                                                       })
                                                       .collect();
        let default:  Option<Box<JsonType>> = self.default.convert();
        obox!(O(
                linked_hash_map![
                   "$switch".to_string() => 
                       obox!(O(linked_hash_map![
                               "branches".to_string() => obox!(A(branches)),
                               "default".to_string()  => default
                               ]
                               ))
                ]
                ))
    }
}

//  {
//     "$let":
//       {
//         "vars" : { "var1" : exp1, ...},
//         "in" : expr
//       }
//  }
//
impl Convert<Option<Box<JsonType>>> for Let {
    fn convert(self) -> Option<Box<JsonType>> {
        use self::JsonType::*;
        let vars: LinkedHashMap<String, Option<Box<JsonType>>> = self.assignments
                                                                     .into_iter()
                                                                     .map(|(s, e)| {(s, e.convert())})
                                                                     .collect();
        let expr: Option<Box<JsonType>> = self.expr.convert();
        obox!(O(
            linked_hash_map![
                "$let".to_string() => 
                     obox!(O(linked_hash_map![
                         "vars".to_string() => obox!(JsonType::O(vars)),
                         "in".to_string()   => expr
                                       ]
                          ))
                ]
                ))
    }
}


//  {
//     "$map":
//       {
//         "input": expr,
//         "as"   : string,
//         "in"   : expr
//       }
//  }
//
impl Convert<Option<Box<JsonType>>> for Map {
    fn convert(self) -> Option<Box<JsonType>> {
        use self::JsonType::*;
        let input: Option<Box<JsonType>> = self.input.convert();
        let ename                        = obox!(S(self.ename));
        let expr:  Option<Box<JsonType>> = self.expr.convert();
        obox!(O(
            linked_hash_map![
                "$map".to_string() => 
                     obox!(O(linked_hash_map![
                             "input".to_string() => input,
                             "as".to_string()    => ename,
                             "in".to_string()    => expr
                                       ]
                          ))
                ]
                ))
    }
}
