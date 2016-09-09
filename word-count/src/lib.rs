use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    let  mut result = HashMap::new();

    let text = text.to_lowercase();
    let words = text.split(|text: char| !text.is_alphanumeric())
        .filter(|text| !text.is_empty())
        .collect::<Vec<&str>>();

    for word in words {
        let count = match result.get(word){
            Some(n) => n + 1,
            None => 1,
        };
        result.insert(word.to_string(), count);
    }
    result
}
