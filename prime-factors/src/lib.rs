pub fn factors(n: u64) -> Vec<u64> {
    let mut l = (n as f64).sqrt() as u64;
    let mut num = n;
    let mut factors = Vec::new();
    while num != 1 {
        let mut flag = true;
        for i in 2..=l {
            if is_prime(i) && num % i == 0 {
                factors.push(i);
                num /= i;
                l = (num as f64).sqrt() as u64;
                flag = false;
                break;
            }
        }
        if flag {
            factors.push(num);
            num = 1;
        }
    }
    factors
}
fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let l = (n as f64).sqrt() as u64;
    for i in 3..=l {
        if n % i == 0 {
            return false;
        }
    }
    true
}
