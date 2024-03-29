use std::{collections::HashMap, sync::Arc, thread};

pub fn frequency1(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut handlers = Vec::new();
    let s = Arc::new(input.join(""));
    println!("{}", s);
    let len = s.len();
    if len == 0 {
        return HashMap::new();
    }
    let chunk_size = len / worker_count;
    for i in 0..worker_count {
        let chunk = Arc::clone(&s);
        let start = i * chunk_size;
        let end = if i == worker_count - 1 {
            len
        } else {
            i * chunk_size + chunk_size
        };
        let h = thread::spawn(move || char_frequency(&chunk[start..end]));
        handlers.push(h);
    }
    handlers
        .into_iter()
        .map(|h| h.join().unwrap())
        .fold(HashMap::new(), |mut acc, h| {
            for (k, v) in h {
                *acc.entry(k).or_insert(0) += v;
            }
            acc
        })
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter = |input: &[&str]| {
        let mut map = HashMap::new();
        for line in input {
            for c in line
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
            {
                *map.entry(c).or_default() += 1;
            }
        }
        map
    };
    // redirect to the best implementation.
    match input.len() {
        0 => HashMap::new(),
        n if n < 500 => counter(input),
        _ => thread::scope(|s| {
            let mut handles = Vec::with_capacity(worker_count);
            for lines in input.chunks(input.len() / worker_count + 1) {
                handles.push(s.spawn(|| counter(lines)))
            }
            let mut map = handles.pop().unwrap().join().unwrap();
            for res in handles {
                res.join().unwrap().into_iter().for_each(|(k, v)| {
                    *map.entry(k).or_default() += v;
                })
            }
            map
        }),
    }
}

fn char_frequency(input: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in input.chars() {
        if c.is_numeric() || c.is_ascii_punctuation() {
            continue;
        }
        *freq.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }
    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_1_worker() {
        let input = vec!["abc"];
        let expected: HashMap<char, usize> =
            vec![('a', 1), ('b', 1), ('c', 1)].into_iter().collect();
        let actual = frequency(&input, 1);
        assert_eq!(expected, actual);

        let input = vec!["abc", "abc"];
        let expected: HashMap<char, usize> =
            vec![('a', 2), ('b', 2), ('c', 2)].into_iter().collect();
        let actual = frequency(&input, 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_frequency_2_workers() {
        let input = vec!["abc"; 999];
        let expected: HashMap<char, usize> = vec![('a', 999), ('b', 999), ('c', 999)]
            .into_iter()
            .collect();
        let actual = frequency(&input, 2);
        assert_eq!(expected, actual);

        let input = vec!["abc", "abc"];
        let expected: HashMap<char, usize> =
            vec![('a', 2), ('b', 2), ('c', 2)].into_iter().collect();
        let actual = frequency(&input, 2);
        assert_eq!(expected, actual);
    }
}
