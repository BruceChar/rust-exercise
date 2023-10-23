pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    let mut factor_push = |f| {
        match (f, n % f) {
            (3, 0) => s.push_str("Pling"),
            (5, 0) => s.push_str("Plang"),
            (7, 0) => s.push_str("Plong"),
            _ => (),
        }
    };
    factor_push(3);
    factor_push(5);
    factor_push(7);
    if s.is_empty() {
        s.push_str(&n.to_string());
    }
    s
}
