use std::collections::HashSet;

#[inline]
fn pair(s: &str) -> (String, Vec<char>) {
    let origin = s.to_lowercase();
    let mut word: Vec<_> = origin.chars().collect();
    word.sort();
    (origin, word)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let (origin_w, word) = pair(word);
    possible_anagrams.iter().filter(|&s| {
        let (origin_s, s)= pair(s);
        *s == word && origin_w != origin_s
    }).copied().collect()
}
