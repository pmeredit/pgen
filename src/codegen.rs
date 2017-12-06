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

fn pad(depth: u32) -> String {
    (0..depth).map(|_| "\t").collect()
}

// TODO: Make all to_go_bson use Cow
pub fn to_go_bson(input: &Option<Box<JsonType>>) -> String {
    input.to_go_bson(0)
}

pub trait ToGoBson {
    fn to_go_bson(&self, depth: u32) -> String;
}

impl ToGoBson for Option<Box<JsonType>> {
    fn to_go_bson(&self, depth: u32) -> String {
        match *self {
            None => "null".to_string(),
            Some(ref j) => j.to_go_bson(depth)
        }
    }
}

impl ToGoBson for JsonType {
    fn to_go_bson(&self, depth: u32) -> String {
        use self::JsonType::*;
        match *self {
            I(ref i) => pad(depth) + &i.to_string(), 
            F(ref f) => pad(depth) + &f.to_string(),
            B(ref b) => pad(depth) + &b.to_string(),
            S(ref s) => pad(depth) + "\"" + &s + "\"",
            A(ref v) => {
                pad(depth) + "[]interface{}{\n" +
                v.iter().fold("".to_string(), 
                    |acc, ref x|  acc + match **x {
                          None => pad(depth + 1) + "null",
                          Some(ref y) => y.to_go_bson(depth + 1)
                          }.as_str() + ",\n"
                      ).as_str() + &pad(depth) + "}"
            },
            O(ref o) => {
                pad(depth) + "bson.D{\n" +
                o.iter().fold("".to_string(), 
                    |acc, (ref k, ref v)|  acc + match **v {
                          None => pad(depth + 1) + "{\"" + &k + "\",null" + "},",
                          Some(ref y) => pad(depth + 1) + "{\"" +  &k + "\",\n" + &y.to_go_bson(depth + 1) + ",\n" + &pad(depth + 1) + "}"
                          }.as_str() + ",\n"
                      ).as_str() + &pad(depth) + "}"
            },
        }
    }
}

pub trait Convert<T> {
    fn convert(self) -> T;
}

pub trait ConvertMatch<T> {
    fn convert_match(self) -> T;
}

impl Convert<Option<Box<JsonType>>> for Expr {
     fn convert(self) -> Option<Box<JsonType>> {
          use self::JsonType::*;
          match self {
              Expr::Null          => None,
              Expr::Number(n)     => obox!(I(n)),
              Expr::Float(f)      => obox!(F(f)),
              Expr::Bool(b)       => obox!(B(b)),
              Expr::Str(s)        => obox!(S(s.replace(r#"\"#, r#""#))),
              // Variables introduced by let should have $$ prefixed
              Expr::ID(s)         => obox!(S("$$".to_string() + &s)),
              // Cols still have $ prefixed, whereas IDs did not
              Expr::Col(s)        => obox!(S(s)),
              Expr::Cond(c)       => c.convert(),
              Expr::Switch(sw )   => sw.convert(),
              Expr::Let(l)        => l.convert(),
              Expr::Map(m)        => m.convert(),
              Expr::Filter(f)     => f.convert(),
              Expr::Reduce(r)     => r.convert(),
              Expr::Zip(z)        => z.convert(),
              Expr::App(s,args)   => {  
                 obox!(O(linked_hash_map![
                                            // check for inArray, we had to use that because in is a
                                            // keyword
                                            "$".to_string() + if s == "inArray" { "in" } else { &s } 
                                                 => 
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
                                                  .map(|PipelineItem{stage_name, stage}| {
                                                              obox!(O(
                                                                      linked_hash_map!["$".to_string() + &stage_name => stage.convert()]
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


//  {
//     "$filter":
//       {
//         "input": expr,
//         "as"   : string,
//         "cond" : expr
//       }
//  }
//
impl Convert<Option<Box<JsonType>>> for Filter {
    fn convert(self) -> Option<Box<JsonType>> {
        use self::JsonType::*;
        let input: Option<Box<JsonType>> = self.input.convert();
        let ename                        = obox!(S(self.ename));
        let cond:  Option<Box<JsonType>> = self.cond.convert();
        obox!(O(
            linked_hash_map![
                "$filter".to_string() => 
                     obox!(O(linked_hash_map![
                             "input".to_string() => input,
                             "as".to_string()    => ename,
                             "cond".to_string()  => cond
                                       ]
                          ))
                ]
                ))
    }
}


//  {
//     "$reduce":
//       {
//         "input"        : expr,
//         "initialValue" : expr,
//         "in"           : expr
//       }
//  }
//
impl Convert<Option<Box<JsonType>>> for Reduce {
    fn convert(self) -> Option<Box<JsonType>> {
        use self::JsonType::*;
        let input: Option<Box<JsonType>> = self.input.convert();
        let init:  Option<Box<JsonType>> = self.init.convert();
        let expr:  Option<Box<JsonType>> = self.expr.convert();
        obox!(O(
            linked_hash_map![
                "$reduce".to_string() => 
                     obox!(O(linked_hash_map![
                             "input".to_string()           => input,
                             "initialValue".to_string()    => init,
                             "in".to_string()              => expr
                                       ]
                          ))
                ]
                ))
    }
}


//  {
//     "$zip":
//       {
//         "inputs"             : arrayExp,
//         "useLongestLength"   : bool,
//         "defaults"           : arrayExp
//       }
//  }
//
impl Convert<Option<Box<JsonType>>> for Zip {
    fn convert(self) -> Option<Box<JsonType>> {
        use self::JsonType::*;
        let inputs: Option<Box<JsonType>>   = self.inputs.convert();
        let longest                         = obox!(B(self.longest));
        let defaults: Option<Box<JsonType>> = self.defaults.convert();
        obox!(O(
            linked_hash_map![
                "$zip".to_string() => 
                     if self.longest {
                         obox!(O(linked_hash_map![
                                 "inputs".to_string()   => inputs,
                                 "useLongestLength".to_string()  => longest,
                                 "defaults".to_string() => defaults
                                 ]
                              ))
                     } else {
                         obox!(O(linked_hash_map![
                                 "inputs".to_string()   => inputs
                                 ]
                              ))
                     }
                ]
                ))
    }
}
