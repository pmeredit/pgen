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

macro_rules! hash_map {
     ( $( $x:expr => $y:expr ),* ) => {
        {
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        }
    };
}

macro_rules! linked_hash_map {
     ( $( $x:expr => $y:expr ),* ) => {
        {
            let mut temp_map = LinkedHashMap::new();
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        }
    };
}

macro_rules! obox {
    ( $x:expr ) => { Some(Box::new($x)) }
}
