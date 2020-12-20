use std::collections::{HashMap, HashSet};

fn char_freq(word: &str) -> HashMap<char, usize> {
    let mut letters = HashMap::new();

    for ch in word.chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    letters
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let freq = char_freq(&word);

    possible_anagrams
        .iter()
        .filter_map(|candidate| {
            let s = candidate.to_lowercase();
            if s.len() != word.len() || s == word {
                return None;
            }
            let f = char_freq(&s);
            if f == freq {
                return Some(*candidate);
            }
            return None;
        })
        .collect()
}
