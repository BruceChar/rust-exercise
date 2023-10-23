pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut candidates = vec![2].into_iter().chain((3..).step_by(2).into_iter());
    let mut c = candidates.next().unwrap();
    let mut l = (n as f64).sqrt() as u64;
    while n != 1 {
        if c > l {
            factors.push(n);
            break;
        }
        while n % c == 0 {
            factors.push(c);
            n /= c;
            l = (n as f64).sqrt() as u64;
        }
        c = candidates.next().unwrap();
    }
    factors
}