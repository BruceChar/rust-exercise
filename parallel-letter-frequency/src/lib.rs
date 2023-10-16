use std::{collections::HashMap, thread, sync::Arc};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
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
        let start = i*chunk_size;
        let end = if i == worker_count - 1 { len } else { i*chunk_size + chunk_size };
        let h = thread::spawn(move || {
            char_frequency(&chunk[start..end])
        });
        handlers.push(h);
    }
    handlers.into_iter().map(|h| h.join().unwrap()).fold(HashMap::new(), |mut acc, h| { 
        for (k, v) in h {
            *acc.entry(k).or_insert(0) += v;
        }
        acc
    })

}

fn char_frequency(input: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in input.chars() {
        *freq.entry(c as char).or_insert(0) += 1;
    }
    freq
}
