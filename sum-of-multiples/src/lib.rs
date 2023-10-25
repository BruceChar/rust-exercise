use std::collections::HashMap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = HashMap::new();
    for factor in factors {
        if factor == &0 { 
            continue;
        }
        (1..limit).filter(|n| n % factor == 0).for_each(|n| {
            sum.insert(n, n);
        });
    }
    sum.values().sum()
}
