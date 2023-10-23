const LOOKUP: [u32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

pub fn nth(n: u32) -> u32 {
    if n < 10 {
        return LOOKUP[n as usize];
    }

    let mut nth = 10;
    for i in 31.. {
        if is_prime(i) {
            if nth == n {
                return i;
            }
            nth += 1;
        }
    }
    panic!("nth prime greater than 1000000");
}

fn is_prime(n: u32) -> bool {
    if n < 30 {
        return LOOKUP.contains(&n);
    }
    let l = (n as f64).sqrt() as u32;
    match LOOKUP.iter().filter(|&d| *d <= l).any(|&d| n % d == 0) {
        // if any of the divisors are less than or equal to the square root of n, then n is not prime
        true => false,
        false => {
            if l > 30 {
                return !(31..=l).step_by(2).any(|d| n % d == 0);
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_nth() {
        assert_eq!(nth(0), 2);
        assert_eq!(nth(10), 31);
        assert_eq!(nth(100), 547);
        assert_eq!(nth(1000), 7927);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(961), false);
        assert_eq!(is_prime(97), true);
    }
}
