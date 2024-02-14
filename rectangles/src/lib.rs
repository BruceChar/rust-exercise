use std::cmp::Ordering;

pub fn count(lines: &[&str]) -> u32 {
    if lines.len() < 2 {
        return 0;
    }
    let mut map: Vec<Vec<usize>> = vec![];
    for l in lines.iter() {
        let mut t = vec![];
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '+' {
                t.push(j)
            }
        });
        map.push(t);
    }

    let mut sum = 0;
    for i in 0..map.len() - 1 {
        for j in i + 1..map.len() {
            let mut dot = 0;
            let mut p = 0;
            let mut q = 0;
            while p < map[i].len() && q < map[j].len() {
                match map[i][p].cmp(&map[j][q]) {
                    Ordering::Less => p += 1,
                    Ordering::Greater => q += 1,
                    Ordering::Equal => {
                        dot += 1;
                        p += 1;
                        q += 1;
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
