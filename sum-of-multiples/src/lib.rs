use std::collections::HashSet;

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u32, b: u32) -> u32 {
    a * b / gcd(a, b)
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    let mut fs: Vec<_> = factors.into_iter().filter(|&&f| f != 0).collect();
    fs.sort();
    let mut i = 0;
    let mut j = fs.len() - 1;
    while i < j {
        let mut k = i;
        while k < j {
            if *fs[k] != 0 && fs[j] % fs[k] == 0 {
                fs[j] = &0;
                j -= 1;
            } else {
                k += 1;
            }
        }
    }
    for (i, &f) in fs.iter().skip(1).enumerate() {}

    let lsm: Vec<_> = fs.windows(2).map(|w| lcm(*w[0], *w[1])).collect();

    factors.iter().filter(|&&f| f != 0).for_each(|f| {
        let n = limit / f;
        sum += (n + 1) / 2 * n * f;
    });
    factors.windows(2).for_each(|w| {
        let (f1, f2) = (w[0], w[1]);
        if f1 == f2 {
            let n = limit / f1;
            sum -= n * (n + 1) * f1 / 2;
        }
    });
    sum
}

pub fn sum_of_multiples1(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = HashSet::new();
    for factor in factors {
        if factor == &0 {
            continue;
        }
        (1..limit).filter(|n| n % factor == 0).for_each(|n| {
            sum.insert(n);
        });
    }
    sum.iter().sum()
}
