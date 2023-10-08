/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut len = 0;
    for (i, c) in code.chars().rev().filter(|&c| c != ' ').enumerate() {
        len += 1;
        match (i % 2, c.to_digit(10)) {
            (0, Some(d)) => sum += d,
            (1, Some(d)) => {
                if d > 4 {
                    sum += d * 2 - 9
                } else {
                    sum += d * 2
                }
            }
            _ => return false,
        }
    }
    if len < 2 {
        return false;
    }
    sum % 10 == 0
}
