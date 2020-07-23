use std::collections::HashMap;

pub fn test() {
    let text = "hello world hello";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("Word frequencies: {:#?}", freqs);
}
