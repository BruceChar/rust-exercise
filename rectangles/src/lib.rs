pub fn count(lines: &[&str]) -> u32 {
    if lines.len() < 2 {
        return 0;
    }

    let mut sum = 0;
    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            let mut dot = 0;
            for (k, (a, b)) in lines[i].chars().zip(lines[j].chars()).enumerate() {
                match (a, b) {
                    ('+', '+') => {
                        let h = (i + 1..j)
                            .filter(|ind| {
                                let c = lines[*ind].chars().nth(k);
                                c == Some('|') || c == Some('+')
                            })
                            .count();
                        if h == j - i - 1 {
                            dot += 1;
                        }
                    }
                    ('-' | '+', '-' | '+') => (),
                    _ => {
                        sum += if dot > 0 { (dot - 1) * (dot) / 2 } else { 0 };
                        dot = 0; // reset after count
                    }
                }
            }
            if dot > 0 {
                sum += (dot - 1) * (dot) / 2;
            }
        }
    }
    sum
}
