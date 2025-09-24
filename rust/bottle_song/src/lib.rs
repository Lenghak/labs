const WORDS: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();

    for i in 0..take_down {
        let numeric_word = num_to_words((start_bottles - i) as usize);

        let first = format!(
            "{} green bottle{} hanging on the wall",
            numeric_word,
            pluralize(start_bottles - i)
        );

        let third: String = format!(
            "There'll be {} green bottle{} hanging on the wall.",
            num_to_words((start_bottles - i - 1) as usize).to_lowercase(),
            pluralize(start_bottles - i - 1)
        );

        result.push_str(&format!(
            "{},\n{},\nAnd if one green bottle should accidentally fall,\n{}",
            &first, &first, &third
        ));

        if i < take_down - 1 {
            result.push_str("\n\n");
        }
    }

    result
}

fn num_to_words(num: usize) -> &'static str {
    WORDS.get(num).copied().unwrap_or("")
}

fn pluralize(n: u32) -> &'static str {
    if n == 1 { "" } else { "s" }
}
