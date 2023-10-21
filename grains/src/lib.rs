pub fn square(s: u32) -> u64 {
    if s > 64 || s == 0 {
        panic!("Square must be between 1 and 64")
    }
    // 2u64.pow(s - 1)
    1 << (s - 1)
}

pub fn total() -> u64 {
    // 18_446_744_073_709_551_615
    // (1..=64).fold(0, |acc, s| acc + square(s))
    // (2u128.pow(64) - 1) as u64
    ((1u128 << 64) - 1) as u64

}
