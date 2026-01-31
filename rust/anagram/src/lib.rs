use std::collections::HashSet;

fn sorted_key(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> HashSet<&'a str> {
    let word_norm = word.to_lowercase();
    let word_key = sorted_key(&word_norm);

    candidates
        .iter()
        .filter(|&&cand| {
            let cand_norm = cand.to_lowercase();
            cand_norm != word_norm && sorted_key(&cand_norm) == word_key
        })
        .copied()
        .collect()
}
