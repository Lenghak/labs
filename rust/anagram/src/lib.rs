use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // 0. Init hashset
    let mut result: HashSet<&str> = HashSet::new();

    // 1. convert the word to all lowercase
    let word_lowercase = word.to_lowercase();
    // 2. convert the word to char
    let word_bytes: Vec<char> = word_lowercase.chars().collect();
    let sum_word_bytes: u32 = word_bytes.iter().map(|&b| (b as u32) * (b as u32)).sum();
    // 3. loop in the possible anagrams
    possible_anagrams.iter().for_each(|possible_anagram| {
        // 4. each possible anagram -> convert to lowercase
        let possible_lowercase = possible_anagram.to_lowercase();
        // 5. FIRST compare if the word original is the same as the one from the loop then ignore that
        if word_lowercase == possible_lowercase {
            return;
        }

        let possible_bytes: Vec<char> = possible_lowercase.chars().collect();

        let sum_of_possible: u32 = possible_bytes
            .iter()
            .map(|&b| (b as u32) * (b as u32))
            .sum();

        // 6. SECOND comapre, sum of each squared element of the original to the sum of bytes of the possibility
        if sum_word_bytes == sum_of_possible {
            result.insert(possible_anagram);
        }
    });

    // 7. If equal that word in the index is the anagram -> add it to the result HashSet
    result
}
