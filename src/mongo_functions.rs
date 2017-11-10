use std::collections::HashSet;

macro_rules! set {
     ( $( $x:expr ),* ) => {
        {
            let mut temp_hs = HashSet::new();
            $(
                temp_hs.insert($x);
            )*
            temp_hs
        }
    };
}

// Add function replacements for things we don't currently have in
// server??

// TODO: $meta
lazy_static! {
    pub static ref FUNCTIONS: HashSet<&'static str> =
        set!["not",
             "setEquals",
             "setIntersection",
             "setUnion",
             "setDifference",
             "setIsSubset",
             "anyElementTrue",
             "allElementsTrue",
             "cmp",
             "abs",
             "add",
             "ceil",
             "divide",
             "exp",
             "floor",
             "ln",
             "log",
             "log10",
             "mod",
             "multiply",
             "pow",
             "sqrt",
             "subtract",
             "trunc",
             "concat",
             "indexOfBytes",
             "indexOfCP",
             "split",
             "strLenBytes",
             "strLenCP",
             "strcasecmp",
             "substr",
             "substrBytes",
             "substrCP",
             "toLower",
             "toUpper",
             "arrayElementAt",
             "arrayToObject",
             "concatArrays",
             "filter",
             "in",
             "indexOfArray",
             "isArray",
             "map",
             "objectToArray",
             "range",
             "reduce",
             "reverseArray",
             "size",
             "slice",
             "zip",
             "literal",
             "dayOfYear",
             "dayOfMonth",
             "dayOfWeek",
             "year",
             "month",
             "week",
             "hour",
             "minute",
             "second",
             "millisecond",
             "dateToString",
             "isoDayOfWeek",
             "isoWeek",
             "isoWeekYear",
             "ifNull",
             "type",
             "sum",
             "avg",
             "first",
             "last",
             "max",
             "min",
             "push",
             "addToSet",
             "stdDevPop",
             "stdDevSamp"];
}
