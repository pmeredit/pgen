use ast::*;
use linked_hash_map::LinkedHashMap;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonType {
    #[serde(rename = "null")] Null,
    I(i64),
    F(f64),
    B(bool),
    S(String),
    // Any recursive JsonTypes must be Options to support null
    A(Vec<JsonType>),
    O(LinkedHashMap<String, JsonType>),
}

fn pad(depth: u32) -> String {
    (0..depth).map(|_| "\t").collect()
}

pub fn to_go_bson(input: JsonType) -> String {
    input.to_go_bson(0)
}

pub trait ToGoBson {
    fn to_go_bson(&self, depth: u32) -> String;
}

impl ToGoBson for JsonType {
    fn to_go_bson(&self, depth: u32) -> String {
        use self::JsonType::*;
        match *self {
            Null => format!("{}{}", pad(depth), "nil"),
            I(i) => format!("{}{}", pad(depth), i.to_string()),
            F(f) => format!("{}{}", pad(depth), f.to_string()),
            B(b) => format!("{}{}", pad(depth), b.to_string()),
            S(ref s) => format!("{}\"{}\"", pad(depth), s),
            A(ref v) => {
                pad(depth) + "[]interface{}{\n" +
                    v.iter()
                        .fold("".to_string(), |acc, ref x| {
                            acc + x.to_go_bson(depth + 1).as_str() + ",\n"
                        })
                        .as_str() + &pad(depth) + "}"
            }
            O(ref o) => {
                pad(depth) + "bson.D{\n" +
                    o.iter()
                        .fold("".to_string(), |acc, (ref k, ref v)| {
                            acc + pad(depth + 1).as_str() + "{\"" + &k + "\",\n" +
                                &v.to_go_bson(depth + 1) + ",\n" +
                                &pad(depth + 1) + "}" + ",\n"
                        })
                        .as_str() + &pad(depth) + "}"
            }
        }
    }
}

pub trait Convert<T> {
    fn convert(self) -> T;
}

pub trait ConvertMatch<T> {
    fn convert_match(self) -> T;
}

impl Convert<JsonType> for Expr {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        match self {
            Expr::Null => Null,
            Expr::Number(n) => I(n),
            Expr::Float(f) => F(f),
            Expr::Bool(b) => B(b),
            Expr::Str(s) => S(s.replace(r#"\"#, r#""#)),
            // Variables introduced by let should have $$ prefixed
            Expr::ID(s) => S("$$".to_string() + &s),
            // Cols still have $ prefixed, whereas IDs did not
            Expr::Col(s) => S(s),
            Expr::Cond(c) => c.convert(),
            Expr::Switch(sw) => sw.convert(),
            Expr::Let(l) => l.convert(),
            Expr::Map(m) => m.convert(),
            Expr::Filter(f) => f.convert(),
            Expr::Reduce(r) => r.convert(),
            Expr::Zip(z) => z.convert(),
            Expr::App(s, args) => {
                O(linked_hash_map![
                    // check for inArray, we had to use that because in is
                    // a keyword
                    "$".to_string() + if s == "inArray" { "in" } else { &s }
                        =>
                           if args.len() > 1 {
                               args.convert()
                           }
                           // TODO: Revaluate if we get functions
                           // that take 0 args.
                           else  {
                               args.into_iter().nth(0).unwrap().convert()
                           }
               ])
            }
            Expr::Array(arr) => arr.convert(),
            Expr::Object(items) => {
                let ret: LinkedHashMap<String, JsonType> =
                    items.into_iter().map(|(k, v)| (k, v.convert())).collect();
                O(ret)
            }
            Expr::Op(..) => panic!("Should not be converting Op, make sure to normalize first"),
            Expr::Error => panic!("Should not be converting Error!"),
        }
    }
}

impl Convert<JsonType> for Vec<Box<Expr>> {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        let ret: Vec<JsonType> = self.into_iter().map(|e| e.convert()).collect();
        A(ret)
    }
}

impl Convert<JsonType> for Pipeline {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        let ret: Vec<JsonType> = self.stages
            .into_iter()
            .map(|PipelineItem { stage_name, stage }| {
                O(linked_hash_map!["$".to_string() + &stage_name => stage.convert()])
            })
            .collect();
        A(ret)
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
impl Convert<JsonType> for Cond {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        O(linked_hash_map![
                    "$cond".to_string() =>
                          O(linked_hash_map![
                              "if".to_string() => self.cond.convert(),
                              "then".to_string() => self.then.convert(),
                              "else".to_string() => self.otherwise.convert()
                           ]
                          )
                ])
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
impl Convert<JsonType> for Switch {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        let branches: Vec<JsonType> = self.cases
            .into_iter()
            .map(|(i, t)| {
                O(linked_hash_map![
                    "case".to_string() => i.convert(),
                    "then".to_string() => t.convert()
                  ])
            })
            .collect();
        let default: JsonType = self.default.convert();
        O(linked_hash_map![
                   "$switch".to_string() =>
                       O(linked_hash_map![
                               "branches".to_string() => A(branches),
                               "default".to_string()  => default
                               ]
                               )
                ])
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
impl Convert<JsonType> for Let {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        let vars: LinkedHashMap<String, JsonType> = self.assignments
            .into_iter()
            .map(|(s, e)| (s, e.convert()))
            .collect();
        let expr: JsonType = self.expr.convert();
        O(linked_hash_map![
                "$let".to_string() =>
                     O(linked_hash_map![
                         "vars".to_string() => JsonType::O(vars),
                         "in".to_string()   => expr
                                       ]
                          )
                ])
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
impl Convert<JsonType> for Map {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        let input: JsonType = self.input.convert();
        let ename = S(self.ename);
        let expr: JsonType = self.expr.convert();
        O(linked_hash_map![
                "$map".to_string() =>
                     O(linked_hash_map![
                             "input".to_string() => input,
                             "as".to_string()    => ename,
                             "in".to_string()    => expr
                                       ]
                          )
                ])
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
impl Convert<JsonType> for Filter {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        let input: JsonType = self.input.convert();
        let ename = S(self.ename);
        let cond: JsonType = self.cond.convert();
        O(linked_hash_map![
                "$filter".to_string() =>
                     O(linked_hash_map![
                             "input".to_string() => input,
                             "as".to_string()    => ename,
                             "cond".to_string()  => cond
                                       ]
                    )
                ])
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
impl Convert<JsonType> for Reduce {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        let input: JsonType = self.input.convert();
        let init: JsonType = self.init.convert();
        let expr: JsonType = self.expr.convert();
        O(linked_hash_map![
                "$reduce".to_string() =>
                     O(linked_hash_map![
                             "input".to_string()        => input,
                             "initialValue".to_string() => init,
                             "in".to_string()           => expr
                                       ]
                          )
                ])
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
impl Convert<JsonType> for Zip {
    fn convert(self) -> JsonType {
        use self::JsonType::*;
        let inputs: JsonType = self.inputs.convert();
        let longest = B(self.longest);
        let defaults: Option<JsonType> = self.defaults.map(|x| x.convert());
        O(linked_hash_map![
                "$zip".to_string() =>
                     if self.longest {
                         O(linked_hash_map![
                                 "inputs".to_string()   => inputs,
                                 "useLongestLength".to_string()  => longest,
                                 "defaults".to_string() =>
                                     if let Some(def) = defaults {
                                        def
                                     } else {
                                        Null
                                     }
                           ]
                          )
                     } else {
                         O(linked_hash_map![
                                 "inputs".to_string()   => inputs
                           ]
                          )
                     }
                ])
    }
}
