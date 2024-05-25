pub fn spin_words(words: &str) -> String {
    let mut results = Vec::new();
    for word in words.split(" ") {
        if word.len() >= 5 {
            results.push(word.chars().rev().collect::<String>());
        } else {
            results.push(word.to_string());
        }
    }

    results.join(" ")
}

fn right(words: &str) -> String {
    words.split_ascii_whitespace().map(|word| match word.len() >= 5 {
        true => word.chars().rev().collect(),
        false => word.to_string()
    }).collect::<Vec<String>>().join(" ")
}