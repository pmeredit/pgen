use std::collections::HashMap;

// TODO?: Add function replacements for things we don't currently have in
// server??

pub enum Arity {
    Fixed(u32),
    Variadic,
}

// This will be any meta info for a function that we want to check
// in sanity.rs, currently we only check that the arity matches,
// eventually we might also do type checking for literals (but it won't
// save use from field values)
pub struct MongoFuncInfo {
    pub arity: Arity,
}


// TODO: $meta
lazy_static! {
    pub static ref FUNCTIONS: HashMap<&'static str, MongoFuncInfo> =
        map!["not" => MongoFuncInfo{arity: Arity::Variadic},
             "setEquals" => MongoFuncInfo{arity: Arity::Variadic},
             "setIntersection" => MongoFuncInfo{arity: Arity::Variadic},
             "setUnion" => MongoFuncInfo{arity: Arity::Variadic},
             "setDifference" => MongoFuncInfo{arity: Arity::Variadic},
             "setIsSubset" => MongoFuncInfo{arity: Arity::Variadic},
             "anyElementTrue" => MongoFuncInfo{arity: Arity::Variadic},
             "allElementsTrue" => MongoFuncInfo{arity: Arity::Variadic},
             "cmp" => MongoFuncInfo{arity: Arity::Variadic},
             "abs" => MongoFuncInfo{arity: Arity::Variadic},
             "add" => MongoFuncInfo{arity: Arity::Variadic},
             "ceil" => MongoFuncInfo{arity: Arity::Variadic},
             "divide" => MongoFuncInfo{arity: Arity::Variadic},
             "exp" => MongoFuncInfo{arity: Arity::Variadic},
             "floor" => MongoFuncInfo{arity: Arity::Variadic},
             "ln" => MongoFuncInfo{arity: Arity::Variadic},
             "log" => MongoFuncInfo{arity: Arity::Variadic},
             "log10" => MongoFuncInfo{arity: Arity::Variadic},
             "mod" => MongoFuncInfo{arity: Arity::Variadic},
             "multiply" => MongoFuncInfo{arity: Arity::Variadic},
             "pow" => MongoFuncInfo{arity: Arity::Variadic},
             "sqrt" => MongoFuncInfo{arity: Arity::Variadic},
             "subtract" => MongoFuncInfo{arity: Arity::Variadic},
             "trunc" => MongoFuncInfo{arity: Arity::Variadic},
             "concat" => MongoFuncInfo{arity: Arity::Variadic},
             "indexOfBytes" => MongoFuncInfo{arity: Arity::Variadic},
             "indexOfCP" => MongoFuncInfo{arity: Arity::Variadic},
             "split" => MongoFuncInfo{arity: Arity::Variadic},
             "strLenBytes" => MongoFuncInfo{arity: Arity::Variadic},
             "strLenCP" => MongoFuncInfo{arity: Arity::Variadic},
             "strcasecmp" => MongoFuncInfo{arity: Arity::Variadic},
             "substr" => MongoFuncInfo{arity: Arity::Variadic},
             "substrBytes" => MongoFuncInfo{arity: Arity::Variadic},
             "substrCP" => MongoFuncInfo{arity: Arity::Variadic},
             "toLower" => MongoFuncInfo{arity: Arity::Variadic},
             "toUpper" => MongoFuncInfo{arity: Arity::Variadic},
             "arrayElementAt" => MongoFuncInfo{arity: Arity::Variadic},
             "arrayToObject" => MongoFuncInfo{arity: Arity::Variadic},
             "concatArrays" => MongoFuncInfo{arity: Arity::Variadic},
             "filter" => MongoFuncInfo{arity: Arity::Variadic},
             "in" => MongoFuncInfo{arity: Arity::Variadic},
             "indexOfArray" => MongoFuncInfo{arity: Arity::Variadic},
             "isArray" => MongoFuncInfo{arity: Arity::Variadic},
             "map" => MongoFuncInfo{arity: Arity::Variadic},
             "objectToArray" => MongoFuncInfo{arity: Arity::Variadic},
             "range" => MongoFuncInfo{arity: Arity::Variadic},
             "reduce" => MongoFuncInfo{arity: Arity::Variadic},
             "reverseArray" => MongoFuncInfo{arity: Arity::Variadic},
             "size" => MongoFuncInfo{arity: Arity::Variadic},
             "slice" => MongoFuncInfo{arity: Arity::Variadic},
             "zip" => MongoFuncInfo{arity: Arity::Variadic},
             "literal" => MongoFuncInfo{arity: Arity::Variadic},
             "dayOfYear" => MongoFuncInfo{arity: Arity::Variadic},
             "dayOfMonth" => MongoFuncInfo{arity: Arity::Variadic},
             "dayOfWeek" => MongoFuncInfo{arity: Arity::Variadic},
             "year" => MongoFuncInfo{arity: Arity::Variadic},
             "month" => MongoFuncInfo{arity: Arity::Variadic},
             "week" => MongoFuncInfo{arity: Arity::Variadic},
             "hour" => MongoFuncInfo{arity: Arity::Variadic},
             "minute" => MongoFuncInfo{arity: Arity::Variadic},
             "second" => MongoFuncInfo{arity: Arity::Variadic},
             "millisecond" => MongoFuncInfo{arity: Arity::Variadic},
             "dateToString" => MongoFuncInfo{arity: Arity::Variadic},
             "isoDayOfWeek" => MongoFuncInfo{arity: Arity::Variadic},
             "isoWeek" => MongoFuncInfo{arity: Arity::Variadic},
             "isoWeekYear" => MongoFuncInfo{arity: Arity::Variadic},
             "ifNull" => MongoFuncInfo{arity: Arity::Variadic},
             "type" => MongoFuncInfo{arity: Arity::Variadic},
             "sum" => MongoFuncInfo{arity: Arity::Variadic},
             "avg" => MongoFuncInfo{arity: Arity::Variadic},
             "first" => MongoFuncInfo{arity: Arity::Variadic},
             "last" => MongoFuncInfo{arity: Arity::Variadic},
             "max" => MongoFuncInfo{arity: Arity::Variadic},
             "min" => MongoFuncInfo{arity: Arity::Variadic},
             "push" => MongoFuncInfo{arity: Arity::Variadic},
             "addToSet" => MongoFuncInfo{arity: Arity::Variadic},
             "stdDevPop" => MongoFuncInfo{arity: Arity::Variadic},
             "stdDevSamp" => MongoFuncInfo{arity: Arity::Variadic}
    ];
}
