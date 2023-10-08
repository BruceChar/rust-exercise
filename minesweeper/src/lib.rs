pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row = minefield.len();
    if row == 0 {
        return vec![];
    }
    let col = minefield[0].len();
    let mut res = vec![vec![b' ' as u8; col]; row];

    for (i, s) in minefield.iter().enumerate() {
        for (j, p) in s.as_bytes().iter().enumerate() {
            if *p != b'*' {
                continue;
            }
            // assign '*'
            res[i][j] = *p;
            MINE.map(|(x, y)| {
                let ii = i as i8 + x;
                let jj = j as i8 + y;
                if ii != -1 && jj != -1 && ii < row as i8 && jj < col as i8 {
                    let cur = &mut res[(i as i8 + x) as usize][(j as i8 + y) as usize];
                    if *cur != b'*' {
                        mutate(cur);
                    }
                }
            });
        }
    }
    res.into_iter().map(|r| String::from_utf8(r).unwrap())
        .collect::<Vec<String>>()
}

fn mutate(cur: &mut u8) {
    if *cur == b' ' {
        *cur = 49; // '0' -> 48u8
    } else {
        *cur += 1;
    }
}

static MINE: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
