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

macro_rules! map {
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
