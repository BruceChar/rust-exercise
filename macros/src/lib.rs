#[macro_export]
macro_rules! hashmap {
    ($($($k:expr => $v: expr)+$(,)?)*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $($(map.insert($k, $v);)*)*
            map

        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_invalid() {
        let _hm1: std::collections::HashMap<_, _> = hashmap!('a' => 1, 'b' => 2);
        let _hm2: std::collections::HashMap<_, _> = hashmap!('a' => 1,'b'=>2);
        print!("-----{:?}\n{:?}", _hm1, _hm2);
    }
}
