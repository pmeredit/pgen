use std::collections::HashMap;

// TODO?: Add function replacements for things we don't currently have in
// server??

pub enum Arity {
    // Fixed arity, u32 = size
    Fixed(u32),
    // Variadic arity, u32 = min size
    Variadic(u32),
    // Optional arguments, u32, u32 = min, max
    Optional(u32, u32),
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
        map![
             //special functions argument must be an object
             "map"             => MongoFuncInfo{arity: Arity::Fixed(1)},
             "reduce"          => MongoFuncInfo{arity: Arity::Fixed(1)},
             "filter"          => MongoFuncInfo{arity: Arity::Fixed(1)},
             "zip"             => MongoFuncInfo{arity: Arity::Fixed(1)},
             //these have different formats depending on usage in
             //$group and $project, TODO: actually handle the difference.
             "sum"             => MongoFuncInfo{arity: Arity::Variadic(1)},
             "avg"             => MongoFuncInfo{arity: Arity::Variadic(1)},
             "max"             => MongoFuncInfo{arity: Arity::Variadic(1)},
             "min"             => MongoFuncInfo{arity: Arity::Variadic(1)},
             "stdDevPop"       => MongoFuncInfo{arity: Arity::Variadic(1)},
             "stdDevSamp"      => MongoFuncInfo{arity: Arity::Variadic(1)},
             //$group only, TODO: enforce that
             "first"           => MongoFuncInfo{arity: Arity::Fixed(1)},
             "last"            => MongoFuncInfo{arity: Arity::Fixed(1)},
             "push"            => MongoFuncInfo{arity: Arity::Fixed(1)},
             "addToSet"        => MongoFuncInfo{arity: Arity::Fixed(1)},
             //normal functions
             "not"             => MongoFuncInfo{arity: Arity::Fixed(1)},
             "and"             => MongoFuncInfo{arity: Arity::Variadic(1)},
             "or"              => MongoFuncInfo{arity: Arity::Variadic(1)},
             "setEquals"       => MongoFuncInfo{arity: Arity::Variadic(2)},
             "setIntersection" => MongoFuncInfo{arity: Arity::Variadic(1)},
             "setUnion"        => MongoFuncInfo{arity: Arity::Variadic(1)},
             "setDifference"   => MongoFuncInfo{arity: Arity::Fixed(2)},
             "setIsSubset"     => MongoFuncInfo{arity: Arity::Fixed(2)},
             "anyElementTrue"  => MongoFuncInfo{arity: Arity::Fixed(1)},
             "allElementsTrue" => MongoFuncInfo{arity: Arity::Fixed(1)},
             "cmp"             => MongoFuncInfo{arity: Arity::Fixed(2)},
             "eq"              => MongoFuncInfo{arity: Arity::Fixed(2)},
             "gt"              => MongoFuncInfo{arity: Arity::Fixed(2)},
             "gte"             => MongoFuncInfo{arity: Arity::Fixed(2)},
             "lt"              => MongoFuncInfo{arity: Arity::Fixed(2)},
             "lte"             => MongoFuncInfo{arity: Arity::Fixed(2)},
             "ne"              => MongoFuncInfo{arity: Arity::Fixed(2)},
             "abs"             => MongoFuncInfo{arity: Arity::Fixed(1)},
             "add"             => MongoFuncInfo{arity: Arity::Variadic(2)},
             "ceil"            => MongoFuncInfo{arity: Arity::Fixed(1)},
             "divide"          => MongoFuncInfo{arity: Arity::Fixed(2)},
             "exp"             => MongoFuncInfo{arity: Arity::Fixed(1)},
             "floor"           => MongoFuncInfo{arity: Arity::Fixed(1)},
             "ln"              => MongoFuncInfo{arity: Arity::Fixed(1)},
             "log"             => MongoFuncInfo{arity: Arity::Fixed(2)},
             "log10"           => MongoFuncInfo{arity: Arity::Fixed(1)},
             "mod"             => MongoFuncInfo{arity: Arity::Fixed(2)},
             "multiply"        => MongoFuncInfo{arity: Arity::Variadic(2)},
             "pow"             => MongoFuncInfo{arity: Arity::Fixed(2)},
             "sqrt"            => MongoFuncInfo{arity: Arity::Fixed(1)},
             "subtract"        => MongoFuncInfo{arity: Arity::Fixed(2)},
             "trunc"           => MongoFuncInfo{arity: Arity::Fixed(1)},
             "concat"          => MongoFuncInfo{arity: Arity::Variadic(1)},
             "indexOfBytes"    => MongoFuncInfo{arity: Arity::Optional(2,4)},
             "indexOfCP"       => MongoFuncInfo{arity: Arity::Optional(2,4)},
             "split"           => MongoFuncInfo{arity: Arity::Fixed(2)},
             "strLenBytes"     => MongoFuncInfo{arity: Arity::Fixed(1)},
             "strLenCP"        => MongoFuncInfo{arity: Arity::Fixed(1)},
             "strcasecmp"      => MongoFuncInfo{arity: Arity::Fixed(2)},
             "substr"          => MongoFuncInfo{arity: Arity::Fixed(3)},
             "substrBytes"     => MongoFuncInfo{arity: Arity::Fixed(3)},
             "substrCP"        => MongoFuncInfo{arity: Arity::Fixed(3)},
             "toLower"         => MongoFuncInfo{arity: Arity::Fixed(1)},
             "toUpper"         => MongoFuncInfo{arity: Arity::Fixed(1)},
             "arrayElementAt"  => MongoFuncInfo{arity: Arity::Fixed(2)},
             "arrayToObject"   => MongoFuncInfo{arity: Arity::Fixed(1)},
             "concatArrays"    => MongoFuncInfo{arity: Arity::Variadic(2)},
             "in"              => MongoFuncInfo{arity: Arity::Fixed(2)},
             "indexOfArray"    => MongoFuncInfo{arity: Arity::Optional(2,4)},
             "isArray"         => MongoFuncInfo{arity: Arity::Fixed(1)},
             "objectToArray"   => MongoFuncInfo{arity: Arity::Fixed(1)},
             "range"           => MongoFuncInfo{arity: Arity::Optional(2,3)},
             "reverseArray"    => MongoFuncInfo{arity: Arity::Fixed(1)},
             "size"            => MongoFuncInfo{arity: Arity::Fixed(1)},
             "slice"           => MongoFuncInfo{arity: Arity::Optional(2,3)},
             "literal"         => MongoFuncInfo{arity: Arity::Fixed(1)},
             "dayOfYear"       => MongoFuncInfo{arity: Arity::Fixed(1)},
             "dayOfMonth"      => MongoFuncInfo{arity: Arity::Fixed(1)},
             "dayOfWeek"       => MongoFuncInfo{arity: Arity::Fixed(1)},
             "year"            => MongoFuncInfo{arity: Arity::Fixed(1)},
             "month"           => MongoFuncInfo{arity: Arity::Fixed(1)},
             "week"            => MongoFuncInfo{arity: Arity::Fixed(1)},
             "hour"            => MongoFuncInfo{arity: Arity::Fixed(1)},
             "minute"          => MongoFuncInfo{arity: Arity::Fixed(1)},
             "second"          => MongoFuncInfo{arity: Arity::Fixed(1)},
             "millisecond"     => MongoFuncInfo{arity: Arity::Fixed(1)},
             "dateToString"    => MongoFuncInfo{arity: Arity::Fixed(2)},
             "isoDayOfWeek"    => MongoFuncInfo{arity: Arity::Fixed(1)},
             "isoWeek"         => MongoFuncInfo{arity: Arity::Fixed(1)},
             "isoWeekYear"     => MongoFuncInfo{arity: Arity::Fixed(1)},
             "ifNull"          => MongoFuncInfo{arity: Arity::Fixed(2)},
             "type"            => MongoFuncInfo{arity: Arity::Fixed(1)}
    ];
}
